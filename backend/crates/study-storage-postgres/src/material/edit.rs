use super::{
    access, children, load,
    write::{Entity, MaterialWriteError, owner_institution},
};
use crate::error::map_sqlx_error;
use sqlx::PgConnection;
use std::time::SystemTime;
use study_core::ids::{MaterialId, MaterialRevisionId, UserId};
use study_core::material::RevisionContent;

pub(super) async fn replace_in(
    conn: &mut PgConnection,
    user: UserId,
    material_id: MaterialId,
    id: MaterialRevisionId,
    expected_edit_version: i64,
    content: RevisionContent,
    at: SystemTime,
) -> Result<super::StoredRevision, MaterialWriteError> {
    let at = study_core::timestamp::normalize(at)?;

    let material = load::material(&mut *conn, material_id, true)
        .await?
        .ok_or(MaterialWriteError::NotFound(Entity::Material))?;
    let actor = access::load(
        &mut *conn,
        user,
        Some(material_id),
        owner_institution(material.owner()),
    )
    .await?;
    let mut stored = load::revision(&mut *conn, id, true)
        .await?
        .ok_or(MaterialWriteError::NotFound(Entity::Revision))?;
    let changed = stored.revision.content() != &content;
    stored
        .revision
        .replace_content(&actor.actor(), &material, content, at)?;
    if stored.edit_version != expected_edit_version {
        return Err(MaterialWriteError::DraftConflict {
            expected: expected_edit_version,
            current: stored.edit_version,
        });
    }
    if !changed {
        return Ok(stored);
    }
    let content = stored.revision.content();
    let meta = &content.metadata;
    let result = sqlx::query(
        r#"
                        UPDATE material_revisions
                        SET
                          title = $1,
                          description = $2,
                          material_type = $3,
                          source_year = $4,
                          preview_url = $5,
                          duration_seconds = $6,
                          file_size_bytes = $7,
                          updated_at = $8,
                          edit_version = edit_version + 1
                        WHERE
                          id = $9
                          AND state = 'draft'
                          AND edit_version = $10
                    "#,
    )
    .bind(&content.title)
    .bind(&content.description)
    .bind(content.material_type.as_db_value())
    .bind(meta.source_year)
    .bind(meta.preview_url.as_deref())
    .bind(meta.duration_seconds.map(i64::from))
    .bind(meta.file_size_bytes)
    .bind(study_core::timestamp::to_utc(stored.revision.updated_at())?)
    .bind(id.0)
    .bind(expected_edit_version)
    .execute(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    if result.rows_affected() != 1 {
        return Err(MaterialWriteError::DraftConflict {
            expected: expected_edit_version,
            current: stored.edit_version,
        });
    }
    children::save_children(&mut *conn, &stored.revision).await?;
    stored.edit_version += 1;

    Ok(stored)
}
