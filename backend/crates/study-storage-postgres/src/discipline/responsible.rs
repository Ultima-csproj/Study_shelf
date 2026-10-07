use crate::error::map_sqlx_error;
use futures_util::TryStreamExt;
use sqlx::PgConnection;
use std::collections::HashMap;
use study_core::catalog::TeacherReference;
use study_core::discipline::DisciplineSummary;
use study_core::error::RepositoryResult;
use study_core::ids::TeacherId;

pub(super) async fn populate(
    conn: &mut PgConnection,
    items: &mut [DisciplineSummary],
) -> RepositoryResult<()> {
    if items.is_empty() {
        return Ok(());
    }
    let ids: Vec<i64> = items.iter().map(|item| item.id.0).collect();
    let positions: HashMap<i64, usize> = items
        .iter()
        .enumerate()
        .map(|(index, item)| (item.id.0, index))
        .collect();
    let mut rows = sqlx::query_as::<_, (i64, i64, String)>(
        r#"
            SELECT DISTINCT
              p.discipline_id,
              u.id,
              u.display_name
            FROM
              discipline_placements p
              JOIN public_materials m ON m.id = p.material_id
              JOIN users u ON u.id = p.responsible_user_id
            WHERE
              p.discipline_id = ANY ($1)
            ORDER BY
              p.discipline_id,
              u.id
        "#,
    )
    .bind(&ids)
    .fetch(conn);
    while let Some((parent, id, name)) = rows.try_next().await.map_err(map_sqlx_error)? {
        let index = *positions
            .get(&parent)
            .ok_or_else(|| crate::error::invalid_relation("discipline responsible parent"))?;
        items[index].responsible.push(TeacherReference {
            id: TeacherId(id),
            short_name: name.clone(),
            name,
        });
    }
    Ok(())
}
