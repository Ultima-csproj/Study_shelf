use crate::error::map_sqlx_error;
use sqlx::PgConnection;
use study_core::catalog::normalize_search_text;
use study_core::error::RepositoryResult;
use study_core::ids::{MaterialId, MaterialRevisionId};
use study_core::material::RevisionCredit;

pub(crate) struct SearchDocument<'a> {
    pub material_id: MaterialId,
    pub revision_id: MaterialRevisionId,
    pub title: &'a str,
    pub description: &'a str,
    pub tags: &'a [String],
    pub credits: &'a [RevisionCredit],
}

pub(crate) async fn upsert(
    conn: &mut PgConnection,
    document: SearchDocument<'_>,
) -> RepositoryResult<()> {
    let capacity = document.title.len()
        + document.description.len()
        + document.tags.iter().map(|tag| tag.len() + 1).sum::<usize>()
        + document
            .credits
            .iter()
            .map(|credit| credit.display_name.len() + 1)
            .sum::<usize>()
        + 1;
    let mut text = String::with_capacity(capacity);
    text.push_str(document.title);
    text.push(' ');
    text.push_str(document.description);
    for tag in document.tags {
        text.push(' ');
        text.push_str(tag);
    }
    for credit in document.credits {
        text.push(' ');
        text.push_str(&credit.display_name);
    }
    sqlx::query(
        r#"
            INSERT INTO material_search (
                material_id,
                revision_id,
                search_text,
                sort_title
            )
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (material_id) DO UPDATE
            SET
                revision_id = EXCLUDED.revision_id,
                search_text = EXCLUDED.search_text,
                sort_title = EXCLUDED.sort_title
        "#,
    )
    .bind(document.material_id.0)
    .bind(document.revision_id.0)
    .bind(normalize_search_text(&text))
    .bind(normalize_search_text(document.title))
    .execute(conn)
    .await
    .map_err(map_sqlx_error)?;
    Ok(())
}
