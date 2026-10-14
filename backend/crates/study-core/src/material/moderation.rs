use std::time::SystemTime;

use crate::ids::{MaterialRevisionId, UserId};

use super::MaterialRevision;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModerationDecision {
    Approved,
    Rejected,
}

/// A decision snapshot. Self-review never depends on subsequent access/profile changes.
#[derive(Debug, Clone)]
pub struct ModerationAction {
    pub(super) reviewer_id: UserId,
    pub(super) revision_id: MaterialRevisionId,
    pub(super) decision: ModerationDecision,
    pub(super) reason: Option<String>,
    pub(super) reviewed_at: SystemTime,
    pub(super) was_self_review: bool,
}

impl ModerationAction {
    pub(super) fn new(
        reviewer_id: UserId,
        revision: &MaterialRevision,
        decision: ModerationDecision,
        reason: Option<String>,
        reviewed_at: SystemTime,
    ) -> Self {
        Self {
            reviewer_id,
            revision_id: revision.id(),
            decision,
            reason,
            reviewed_at,
            was_self_review: revision.has_participated(reviewer_id),
        }
    }

    pub fn reviewer_id(&self) -> UserId {
        self.reviewer_id
    }
    pub fn revision_id(&self) -> MaterialRevisionId {
        self.revision_id
    }
    pub fn decision(&self) -> ModerationDecision {
        self.decision
    }
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
    pub fn reviewed_at(&self) -> SystemTime {
        self.reviewed_at
    }
    pub fn was_self_review(&self) -> bool {
        self.was_self_review
    }
}
