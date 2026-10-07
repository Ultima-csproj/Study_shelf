use super::{
    access, load,
    write::{Entity, MaterialWriteError, PostgresMaterialRepository, owner_institution},
};
use crate::catalog::projection::{self, SearchDocument};
use crate::error::map_sqlx_error;
use sqlx::PgConnection;
use std::time::SystemTime;
use study_core::ids::{MaterialId, MaterialRevisionId, UserId};
use study_core::material::{ModerationAction, ModerationDecision};

impl PostgresMaterialRepository {
    pub async fn approve(
        &self,
        user: UserId,
        material_id: MaterialId,
        id: MaterialRevisionId,
        at: SystemTime,
    ) -> Result<ModerationAction, MaterialWriteError> {
        super::retry::run(|| async move {
            let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
            let result = approve_in(&mut tx, user, material_id, id, at).await?;
            tx.commit().await.map_err(map_sqlx_error)?;
            Ok(result)
        })
        .await
    }

    pub async fn reject(
        &self,
        user: UserId,
        material_id: MaterialId,
        id: MaterialRevisionId,
        reason: String,
        at: SystemTime,
    ) -> Result<ModerationAction, MaterialWriteError> {
        super::retry::run(|| {
            let reason = reason.clone();
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
                let mut revision = load::revision(&mut tx, id, true)
                    .await?
                    .ok_or(MaterialWriteError::NotFound(Entity::Revision))?
                    .revision;
                let action = material.reject_revision(&actor.actor(), &mut revision, reason, at)?;
                sqlx::query(
                    r#"
                        UPDATE material_revisions
                        SET
                          state = 'rejected',
                          updated_at = $1
                        WHERE
                          id = $2
                          AND state = 'pending_moderation'
                    "#,
                )
                .bind(study_core::timestamp::to_utc(at)?)
                .bind(id.0)
                .execute(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
                save_action(&mut tx, &action).await?;
                tx.commit().await.map_err(map_sqlx_error)?;
                Ok(action)
            }
        })
        .await
    }
}
async fn save_action(
    conn: &mut PgConnection,
    action: &ModerationAction,
) -> Result<(), MaterialWriteError> {
    let decision = match action.decision() {
        ModerationDecision::Approved => "approved",
        ModerationDecision::Rejected => "rejected",
    };
    sqlx::query(
        r#"
            INSERT INTO
              moderation_actions (
                revision_id,
                reviewer_id,
                decision,
                reason,
                reviewed_at,
                was_self_review
              )
            VALUES
              ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(action.revision_id().0)
    .bind(action.reviewer_id().0)
    .bind(decision)
    .bind(action.reason())
    .bind(study_core::timestamp::to_utc(action.reviewed_at())?)
    .bind(action.was_self_review())
    .execute(conn)
    .await
    .map_err(map_sqlx_error)?;
    Ok(())
}

pub(super) async fn approve_in(
    conn: &mut PgConnection,
    user: UserId,
    material_id: MaterialId,
    id: MaterialRevisionId,
    at: SystemTime,
) -> Result<ModerationAction, MaterialWriteError> {
    let at = study_core::timestamp::normalize(at)?;
    let mut material = load::material(&mut *conn, material_id, true)
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
    let expected = material.published_revision_id();
    let action = material.approve_revision(&actor.actor(), &mut revision, at)?;
    let utc = study_core::timestamp::to_utc(at)?;
    sqlx::query(
        r#"
            UPDATE material_revisions
            SET
              state = 'published',
              published_at = $1,
              updated_at = $1
            WHERE
              id = $2
              AND state = 'pending_moderation'
        "#,
    )
    .bind(utc)
    .bind(id.0)
    .execute(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    let result = sqlx::query(
        r#"
            UPDATE materials
            SET
              published_revision_id = $1,
              first_published_at = COALESCE(first_published_at, $2)
            WHERE
              id = $3
              AND published_revision_id IS NOT DISTINCT FROM $4
        "#,
    )
    .bind(id.0)
    .bind(utc)
    .bind(material_id.0)
    .bind(expected.map(|id| id.0))
    .execute(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    if result.rows_affected() != 1 {
        return Err(MaterialWriteError::RevisionConflict {
            expected,
            current: load::material(&mut *conn, material_id, false)
                .await?
                .and_then(|value| value.published_revision_id()),
        });
    }
    let content = revision.content();
    projection::upsert(
        conn,
        SearchDocument {
            material_id,
            revision_id: id,
            title: &content.title,
            description: &content.description,
            tags: &content.tags,
            credits: &content.credits,
        },
    )
    .await?;
    save_action(&mut *conn, &action).await?;
    Ok(action)
}
