use super::{
    access, children, load,
    write::{Entity, MaterialWriteError, PostgresMaterialRepository, next_id, owner_institution},
};
use crate::error::map_sqlx_error;
use std::time::SystemTime;
use study_core::ids::{MaterialId, MaterialRevisionId, UserId};
use study_core::material::{MaterialError, MaterialRevision, RevisionContent, policy};

impl PostgresMaterialRepository {
    pub async fn create_draft(
        &self,
        user: UserId,
        material_id: MaterialId,
        expected_base: Option<MaterialRevisionId>,
        content: RevisionContent,
        at: SystemTime,
    ) -> Result<MaterialRevision, MaterialWriteError> {
        super::retry::run(|| {
            let content = content.clone();
            async move {
                let at = study_core::timestamp::normalize(at)?;
                let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
                let material = load::material(&mut tx, material_id, true)
                    .await?
                    .ok_or(MaterialWriteError::NotFound(Entity::Material))?;
                let actor = access::load(
                    &mut tx,
                    user,
                    Some(material_id),
                    owner_institution(material.owner()),
                )
                .await?;
                if !policy::can_create_revision(&actor.actor(), &material) {
                    return Err(MaterialError::PermissionDenied.into());
                }
                if material.published_revision_id() != expected_base {
                    return Err(MaterialWriteError::RevisionConflict {
                        expected: expected_base,
                        current: material.published_revision_id(),
                    });
                }
                let id = MaterialRevisionId(
                    next_id(
                        &mut tx,
                        r#"
                            SELECT
                              nextval(
                                pg_get_serial_sequence('material_revisions', 'id')
                              )
                        "#,
                    )
                    .await?,
                );
                let revision =
                    MaterialRevision::new_draft(&actor.actor(), &material, id, content, at)?;
                children::insert_draft(&mut tx, &revision).await?;
                tx.commit().await.map_err(map_sqlx_error)?;
                Ok(revision)
            }
        })
        .await
    }

    /// expected_edit_version detects lost updates even when timestamps are equal.
    pub async fn replace_draft(
        &self,
        user: UserId,
        material_id: MaterialId,
        id: MaterialRevisionId,
        expected_edit_version: i64,
        content: RevisionContent,
        at: SystemTime,
    ) -> Result<super::StoredRevision, MaterialWriteError> {
        super::retry::run(|| {
            let content = content.clone();
            async move {
                let at = study_core::timestamp::normalize(at)?;
                let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
                let material = load::material(&mut tx, material_id, true)
                    .await?
                    .ok_or(MaterialWriteError::NotFound(Entity::Material))?;
                let actor = access::load(
                    &mut tx,
                    user,
                    Some(material_id),
                    owner_institution(material.owner()),
                )
                .await?;
                let mut stored = load::revision(&mut tx, id, true)
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
                    tx.commit().await.map_err(map_sqlx_error)?;
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
                .execute(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
                if result.rows_affected() != 1 {
                    return Err(MaterialWriteError::DraftConflict {
                        expected: expected_edit_version,
                        current: stored.edit_version,
                    });
                }
                children::save_children(&mut tx, &stored.revision).await?;
                stored.edit_version += 1;
                tx.commit().await.map_err(map_sqlx_error)?;
                Ok(stored)
            }
        })
        .await
    }

    pub async fn submit(
        &self,
        user: UserId,
        material_id: MaterialId,
        id: MaterialRevisionId,
        at: SystemTime,
    ) -> Result<MaterialRevision, MaterialWriteError> {
        super::retry::run(|| async move {
            let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
            let result = submit_in(&mut tx, user, material_id, id, at).await?;
            tx.commit().await.map_err(map_sqlx_error)?;
            Ok(result)
        })
        .await
    }
}

pub(super) async fn submit_in(
    conn: &mut sqlx::PgConnection,
    user: UserId,
    material_id: MaterialId,
    id: MaterialRevisionId,
    at: SystemTime,
) -> Result<MaterialRevision, MaterialWriteError> {
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
    let mut revision = load::revision(&mut *conn, id, true)
        .await?
        .ok_or(MaterialWriteError::NotFound(Entity::Revision))?
        .revision;
    revision.submit_for_moderation(&actor.actor(), &material, at)?;
    sqlx::query(
        r#"
            UPDATE material_revisions
            SET
              state = 'pending_moderation',
              submitted_by_user_id = $1,
              submitted_at = $2,
              updated_at = $2
            WHERE
              id = $3
              AND state = 'draft'
        "#,
    )
    .bind(user.0)
    .bind(study_core::timestamp::to_utc(at)?)
    .bind(id.0)
    .execute(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    Ok(revision)
}
