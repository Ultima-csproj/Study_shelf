use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use study_core::catalog::{
    CatalogPage, CatalogQuery, CatalogSort, Material, normalize_search_text,
};
use study_core::error::{RepositoryError, RepositoryResult as Result};

use super::{MaterialRow, load_tags};

const DEFAULT_PAGE_SIZE: u32 = 5;
const MAX_PAGE_SIZE: u32 = 100;

pub(super) async fn run(pool: &SqlitePool, query: CatalogQuery) -> Result<CatalogPage> {
    let page_size = if query.page_size == 0 {
        DEFAULT_PAGE_SIZE
    } else {
        query.page_size.min(MAX_PAGE_SIZE)
    };

    let mut count_sql: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
            SELECT COUNT(*)
            FROM materials m
            WHERE 1 = 1
        "#,
    );
    push_filters(&mut count_sql, &query);
    let (total,): (i64,) = count_sql
        .build_query_as()
        .fetch_one(pool)
        .await
        .map_err(crate::error::map_sqlx_error)?;
    let total = u64::try_from(total).map_err(|error| {
        RepositoryError::invalid_data_with_source("negative catalog count", error)
    })?;
    let page_count = ((total.saturating_add(u64::from(page_size) - 1) / u64::from(page_size))
        .max(1))
    .min(u64::from(u32::MAX)) as u32;
    let page = query.page.max(1).min(page_count);
    let offset = i64::from(page - 1) * i64::from(page_size);

    let mut items_sql: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
            SELECT m.*
            FROM materials m
            WHERE 1 = 1
        "#,
    );
    push_filters(&mut items_sql, &query);
    match query.sort {
        CatalogSort::Newest => items_sql.push(" ORDER BY m.created_at DESC, m.id ASC"),
        CatalogSort::Oldest => items_sql.push(" ORDER BY m.created_at ASC, m.id ASC"),
        CatalogSort::Title => items_sql.push(" ORDER BY m.sort_title ASC, m.id ASC"),
    };
    items_sql.push(" LIMIT ").push_bind(i64::from(page_size));
    items_sql.push(" OFFSET ").push_bind(offset);
    let rows: Vec<MaterialRow> = items_sql
        .build_query_as()
        .fetch_all(pool)
        .await
        .map_err(crate::error::map_sqlx_error)?;
    let mut items: Vec<Material> = rows
        .into_iter()
        .map(Material::try_from)
        .collect::<Result<_>>()?;
    load_tags(pool, &mut items).await?;

    Ok(CatalogPage {
        items,
        total,
        page,
        page_count,
    })
}

fn push_filters(builder: &mut QueryBuilder<Sqlite>, query: &CatalogQuery) {
    for word in normalize_search_text(query.text.trim()).split_whitespace() {
        builder
            .push(" AND m.search_text LIKE ")
            .push_bind(format!("%{}%", escape_like(word)))
            .push(" ESCAPE '\\'");
    }

    push_values(
        builder,
        "m.material_type",
        query
            .material_types
            .iter()
            .map(|kind| kind.as_db_value().to_owned()),
    );
    push_values(
        builder,
        "m.discipline_id",
        query.discipline_ids.iter().map(|id| id.0),
    );
    push_values(
        builder,
        "m.teacher_id",
        query.teacher_ids.iter().map(|id| id.0),
    );
    push_values(
        builder,
        "m.course",
        query.courses.iter().map(|course| i64::from(*course)),
    );
    push_values(builder, "m.year", query.years.iter().copied());

    for tag in &query.tags {
        builder
            .push(
                r#"
                    AND EXISTS (
                        SELECT 1
                        FROM material_tags mt
                        WHERE mt.material_id = m.id
                            AND mt.tag =
                "#,
            )
            .push_bind(tag.clone())
            .push(")");
    }
}

fn push_values<T, I>(builder: &mut QueryBuilder<Sqlite>, column: &str, values: I)
where
    T: for<'q> sqlx::Encode<'q, Sqlite> + sqlx::Type<Sqlite> + Send,
    I: IntoIterator<Item = T>,
{
    let mut values = values.into_iter().peekable();
    if values.peek().is_none() {
        return;
    }
    builder.push(" AND ").push(column).push(" IN (");
    let mut separated = builder.separated(", ");
    for value in values {
        separated.push_bind(value);
    }
    separated.push_unseparated(")");
}

fn escape_like(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}
