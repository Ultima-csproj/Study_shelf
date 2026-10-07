use sqlx::{
    Postgres,
    postgres::PgArguments,
    query::{QueryAs, QueryScalar},
};
use std::sync::LazyLock;
use study_core::catalog::{CatalogQuery, CatalogSort, normalize_search_text};

pub(super) struct PreparedFilter<'a> {
    text: String,
    types: Vec<&'static str>,
    disciplines: Vec<i64>,
    teachers: Vec<i64>,
    courses: Vec<i16>,
    years: &'a [i32],
    tags: &'a [String],
}
impl<'a> PreparedFilter<'a> {
    pub fn new(query: &'a CatalogQuery) -> Self {
        Self {
            text: normalize_search_text(&query.text),
            types: query
                .material_types
                .iter()
                .map(|kind| kind.as_db_value())
                .collect(),
            disciplines: query.discipline_ids.iter().map(|id| id.0).collect(),
            teachers: query.teacher_ids.iter().map(|id| id.0).collect(),
            courses: query
                .courses
                .iter()
                .map(|value| i16::from(*value))
                .collect(),
            years: &query.years,
            tags: &query.tags,
        }
    }
    pub fn bind<'b, O>(
        &'b self,
        query: QueryScalar<'b, Postgres, O, PgArguments>,
    ) -> QueryScalar<'b, Postgres, O, PgArguments> {
        query
            .bind(&self.text)
            .bind(&self.types)
            .bind(&self.disciplines)
            .bind(&self.teachers)
            .bind(&self.courses)
            .bind(self.years)
            .bind(self.tags)
    }
    pub fn bind_as<'b, O>(
        &'b self,
        query: QueryAs<'b, Postgres, O, PgArguments>,
    ) -> QueryAs<'b, Postgres, O, PgArguments> {
        query
            .bind(&self.text)
            .bind(&self.types)
            .bind(&self.disciplines)
            .bind(&self.teachers)
            .bind(&self.courses)
            .bind(self.years)
            .bind(self.tags)
    }
}
const FILTER: &str = include_str!("sql/filter.sql");
static COUNT: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"
            SELECT
              count(*) {FILTER}
        "#
    )
});
static NEWEST: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"
            SELECT
              p.* {FILTER}
            ORDER BY
              p.published_at DESC,
              p.id
            LIMIT
              $8
            OFFSET
              $9
        "#
    )
});
static OLDEST: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"
            SELECT
              p.* {FILTER}
            ORDER BY
              p.published_at,
              p.id
            LIMIT
              $8
            OFFSET
              $9
        "#
    )
});
static TITLE: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"
            SELECT
              p.* {FILTER}
            ORDER BY
              s.sort_title NULLS LAST,
              p.id
            LIMIT
              $8
            OFFSET
              $9
        "#
    )
});
pub(super) fn count() -> &'static str {
    &COUNT
}
pub(super) fn select(sort: CatalogSort) -> &'static str {
    match sort {
        CatalogSort::Newest => &NEWEST,
        CatalogSort::Oldest => &OLDEST,
        CatalogSort::Title => &TITLE,
    }
}
