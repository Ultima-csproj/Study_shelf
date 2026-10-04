use super::{ModerationAction, ModerationDecision};
use crate::ids::{MaterialRevisionId, UserId};
use std::time::SystemTime;
use thiserror::Error;

/// Historical data only. Restoring it never re-evaluates today's reviewer permissions.
pub struct ModerationSnapshot {
    pub reviewer_id: UserId,
    pub revision_id: MaterialRevisionId,
    pub decision: ModerationDecision,
    pub reason: Option<String>,
    pub reviewed_at: SystemTime,
    pub was_self_review: bool,
}

#[derive(Debug, Error)]
pub enum ModerationSnapshotError {
    #[error("invalid moderation identifier")]
    InvalidIdentifier,
    #[error("rejected moderation requires a nonempty reason")]
    MissingRejectionReason,
    #[error("approved moderation cannot contain a rejection reason")]
    UnexpectedReason,
}

impl ModerationAction {
    pub fn try_restore(snapshot: ModerationSnapshot) -> Result<Self, ModerationSnapshotError> {
        if snapshot.reviewer_id.0 <= 0 || snapshot.revision_id.0 <= 0 {
            return Err(ModerationSnapshotError::InvalidIdentifier);
        }
        match snapshot.decision {
            ModerationDecision::Rejected
                if snapshot
                    .reason
                    .as_deref()
                    .is_none_or(|reason| reason.trim().is_empty()) =>
            {
                return Err(ModerationSnapshotError::MissingRejectionReason);
            }
            ModerationDecision::Approved if snapshot.reason.is_some() => {
                return Err(ModerationSnapshotError::UnexpectedReason);
            }
            _ => {}
        }
        Ok(Self {
            reviewer_id: snapshot.reviewer_id,
            revision_id: snapshot.revision_id,
            decision: snapshot.decision,
            reason: snapshot.reason,
            reviewed_at: snapshot.reviewed_at,
            was_self_review: snapshot.was_self_review,
        })
    }
}
