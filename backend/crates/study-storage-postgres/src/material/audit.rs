use super::{PostgresMaterialRepository, mapping};
use crate::error::{map_sqlx_error, unknown};
use sqlx::FromRow;
use study_core::error::{RepositoryError, RepositoryResult};
use study_core::ids::{MaterialRevisionId, UserId};
use study_core::material::{ModerationAction, ModerationDecision, ModerationSnapshot};
use time::OffsetDateTime;

#[derive(FromRow)]
struct ActionRow {
    reviewer_id: i64,
    revision_id: i64,
    decision: String,
    reason: Option<String>,
    reviewed_at: OffsetDateTime,
    was_self_review: bool,
}

impl PostgresMaterialRepository {
    /// Internal audit read. An authenticated application service must authorize exposure.
    pub async fn load_moderation(
        &self,
        id: MaterialRevisionId,
    ) -> RepositoryResult<Option<ModerationAction>> {
        let Some(row) = sqlx::query_as::<_, ActionRow>(
            r#"
                SELECT
                    reviewer_id,
                    revision_id,
                    decision,
                    reason,
                    reviewed_at,
                    was_self_review
                FROM moderation_actions
                WHERE revision_id = $1
            "#,
        )
        .bind(id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?
        else {
            return Ok(None);
        };
        let decision = match row.decision.as_str() {
            "approved" => ModerationDecision::Approved,
            "rejected" => ModerationDecision::Rejected,
            value => return Err(unknown("moderation decision", value)),
        };
        ModerationAction::try_restore(ModerationSnapshot {
            reviewer_id: UserId(row.reviewer_id),
            revision_id: MaterialRevisionId(row.revision_id),
            decision,
            reason: row.reason,
            reviewed_at: mapping::stored_time(row.reviewed_at)?,
            was_self_review: row.was_self_review,
        })
        .map(Some)
        .map_err(|error| {
            RepositoryError::invalid_data_with_source("invalid moderation snapshot", error)
        })
    }
}
