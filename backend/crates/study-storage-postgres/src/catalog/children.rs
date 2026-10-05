use crate::error::map_sqlx_error;
use crate::material::mapping;
use futures_util::TryStreamExt;
use sqlx::{FromRow, PgConnection};
use std::collections::HashMap;
use study_core::catalog::{
    DisciplineReference, EducationalPlacement, MaterialSummary, TeacherReference,
};
use study_core::error::RepositoryResult;
use study_core::ids::{ContributorId, DisciplineId, TeacherId};
use study_core::material::RevisionCredit;

#[derive(FromRow)]
struct PlacementRow {
    material_id: i64,
    discipline_id: i64,
    slug: String,
    name: String,
    responsible_user_id: Option<i64>,
    responsible_name: Option<String>,
    course: Option<i16>,
}

pub(super) async fn populate(
    conn: &mut PgConnection,
    items: &mut [MaterialSummary],
) -> RepositoryResult<()> {
    if items.is_empty() {
        return Ok(());
    }
    let mut ids = Vec::with_capacity(items.len());
    let mut positions = HashMap::with_capacity(items.len());
    for (index, item) in items.iter().enumerate() {
        ids.push(item.id.0);
        positions.insert(item.id.0, index);
    }
    {
        let mut rows = sqlx::query_as::<_, (i64, String)>(
            r#"
                SELECT
                    p.id,
                    t.tag
                FROM materials AS p
                JOIN revision_tags AS t
                    ON t.revision_id = p.published_revision_id
                WHERE p.id = ANY($1)
                ORDER BY p.id, t.position
            "#,
        )
        .bind(&ids)
        .fetch(&mut *conn);
        while let Some((id, tag)) = rows.try_next().await.map_err(map_sqlx_error)? {
            let index = *positions
                .get(&id)
                .ok_or_else(|| crate::error::invalid_relation("tag parent"))?;
            items[index].tags.push(tag);
        }
    }
    {
        let mut rows = sqlx::query_as::<_, (i64, i64, String, String)>(
            r#"
                SELECT
                    p.id,
                    c.contributor_id,
                    c.display_name,
                    c.role
                FROM materials AS p
                JOIN revision_credits AS c
                    ON c.revision_id = p.published_revision_id
                WHERE p.id = ANY($1)
                ORDER BY p.id, c.position
            "#,
        )
        .bind(&ids)
        .fetch(&mut *conn);
        while let Some((id, contributor, name, role)) =
            rows.try_next().await.map_err(map_sqlx_error)?
        {
            let index = *positions
                .get(&id)
                .ok_or_else(|| crate::error::invalid_relation("credit parent"))?;
            items[index].credits.push(RevisionCredit {
                contributor_id: ContributorId(contributor),
                display_name: name,
                role: mapping::role(&role)?,
            });
        }
    }
    {
        let mut rows = sqlx::query_as::<_, PlacementRow>(
            r#"
                SELECT
                    p.material_id,
                    d.id AS discipline_id,
                    d.slug,
                    d.name,
                    p.responsible_user_id,
                    u.display_name AS responsible_name,
                    p.course
                FROM discipline_placements AS p
                JOIN disciplines AS d
                    ON d.id = p.discipline_id
                LEFT JOIN users AS u
                    ON u.id = p.responsible_user_id
                WHERE p.material_id = ANY($1)
                ORDER BY p.material_id, d.id
            "#,
        )
        .bind(&ids)
        .fetch(conn);
        while let Some(row) = rows.try_next().await.map_err(map_sqlx_error)? {
            let index = *positions
                .get(&row.material_id)
                .ok_or_else(|| crate::error::invalid_relation("placement parent"))?;
            let responsible = match (row.responsible_user_id, row.responsible_name) {
                (Some(id), Some(name)) => Some(TeacherReference {
                    id: TeacherId(id),
                    short_name: name.clone(),
                    name,
                }),
                (None, None) => None,
                _ => return Err(crate::error::invalid_relation("responsible identity")),
            };
            let course = row.course.map(u8::try_from).transpose().map_err(|error| {
                study_core::error::RepositoryError::invalid_data_with_source(
                    "invalid course",
                    error,
                )
            })?;
            items[index].placements.push(EducationalPlacement {
                discipline: DisciplineReference {
                    id: DisciplineId(row.discipline_id),
                    slug: row.slug,
                    name: row.name,
                },
                responsible,
                course,
            });
        }
    }
    Ok(())
}
