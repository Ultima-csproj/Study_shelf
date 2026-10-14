use super::super::{
    PostgresMaterialRepository,
    draft::submit_in,
    publication::{approve_in, save_action},
};
use super::{Result, RevisionTransition, audit, locked, reason, revision_version};
use crate::error::map_sqlx_error;
use serde_json::json;
use std::time::SystemTime;
use study_core::ids::{MaterialId, MaterialRevisionId};

impl PostgresMaterialRepository {
    pub async fn transition_authenticated(
        &self,
        digest: &[u8],
        material_id: MaterialId,
        id: MaterialRevisionId,
        expected: i64,
        transition: RevisionTransition,
        cause: &str,
    ) -> Result<()> {
        reason(cause)?;
        let (mut tx, user) = self.begin_session(digest).await?;
        let (material, actor) = locked(&mut tx, user, material_id).await?;
        let mut stored = revision_version(&mut tx, &material, id, expected).await?;
        let at = SystemTime::now();
        let action = match transition {
            RevisionTransition::Discard => {
                stored.revision.discard(&actor.actor(), &material, at)?;
                sqlx::query(include_str!("sql/discard.sql"))
                    .bind(study_core::timestamp::to_utc(at)?)
                    .bind(id.0)
                    .execute(&mut *tx)
                    .await
                    .map_err(map_sqlx_error)?;
                "discard_draft"
            }
            RevisionTransition::Submit => {
                submit_in(&mut tx, user, material_id, id, at).await?;
                "submit_revision"
            }
            RevisionTransition::Withdraw => {
                stored.revision.withdraw(&actor.actor(), &material, at)?;
                sqlx::query(include_str!("sql/withdraw.sql"))
                    .bind(study_core::timestamp::to_utc(at)?)
                    .bind(id.0)
                    .execute(&mut *tx)
                    .await
                    .map_err(map_sqlx_error)?;
                "withdraw_revision"
            }
            RevisionTransition::Approve => {
                approve_in(&mut tx, user, material_id, id, at).await?;
                "approve_revision"
            }
            RevisionTransition::Reject => {
                let at = study_core::timestamp::normalize(at)?;
                let decision = material.reject_revision(
                    &actor.actor(),
                    &mut stored.revision,
                    cause.to_owned(),
                    at,
                )?;
                sqlx::query(include_str!("sql/reject.sql"))
                    .bind(study_core::timestamp::to_utc(at)?)
                    .bind(id.0)
                    .execute(&mut *tx)
                    .await
                    .map_err(map_sqlx_error)?;
                save_action(&mut tx, &decision).await?;
                "reject_revision"
            }
        };
        audit::record(
            &mut tx,
            user,
            material_id,
            Some(id),
            action,
            cause,
            json!({"expectedVersion":expected}),
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
