use super::ScopeSql;
use std::sync::LazyLock;
use study_core::ids::{CollectionId, DisciplineId};
use study_core::learning::SectionScope;

pub(super) struct Queries {
    pub headers: String,
    pub initial: String,
    pub exists: String,
    pub count: String,
    pub page: String,
}
static DISCIPLINE: LazyLock<Queries> =
    LazyLock::new(|| build(SectionScope::Discipline(DisciplineId(0))));
static COLLECTION: LazyLock<Queries> =
    LazyLock::new(|| build(SectionScope::Collection(CollectionId(0))));

pub(super) fn get(scope: SectionScope) -> &'static Queries {
    match scope {
        SectionScope::Discipline(_) => &DISCIPLINE,
        SectionScope::Collection(_) => &COLLECTION,
    }
}
fn build(scope: SectionScope) -> Queries {
    let scope = ScopeSql::new(scope);
    let base = scope.visible_entries();
    let headers = format!(
        r#"
            WITH
              visible AS (
                SELECT
                  e.section_id,
                  count(*) AS step_count
                  {base}
                GROUP BY
                  e.section_id
              )
            SELECT
              s.id,
              s.title,
              s.description,
              row_number() OVER (
                ORDER BY
                  s.position,
                  s.id
              ) AS position,
              v.step_count
            FROM
              {} s
              JOIN visible v ON v.section_id = s.id
            WHERE
              s.{} = $1
            ORDER BY
              s.position,
              s.id
        "#,
        scope.sections, scope.parent_column
    );
    let exists = format!(
        r#"
            SELECT
              EXISTS (
                SELECT
                  1
                FROM
                  {}
                WHERE
                  id = $1
                  AND {} = $2
              )
        "#,
        scope.sections, scope.parent_column
    );
    let count = format!(
        r#"
            SELECT
              count(*) {base}
              AND e.section_id = $2
        "#
    );
    Queries {
        headers,
        initial: entries(&scope, true),
        exists,
        count,
        page: entries(&scope, false),
    }
}
fn entries(scope: &ScopeSql, initial: bool) -> String {
    let base = scope.visible_entries();
    let extra = if initial { "" } else { "AND e.section_id = $2" };
    let suffix = if initial {
        r#"WHERE position <= $2
ORDER BY section_id, position"#
    } else {
        r#"ORDER BY position
LIMIT $3
OFFSET $4"#
    };
    format!(
        r#"
            WITH
              visible AS (
                SELECT
                  e.section_id,
                  e.id,
                  {} AS material_id,
                  {} AS placement_id,
                  row_number() OVER (
                    PARTITION BY
                      e.section_id
                    ORDER BY
                      e.position,
                      e.id
                  ) AS position,
                  {} AS responsible_id,
                  u.display_name AS responsible_name,
                  {} AS course
                  {base}
                  {extra}
              )
            SELECT
              *
            FROM
              visible
            {suffix}
        "#,
        scope.material, scope.placement, scope.responsible, scope.course
    )
}
