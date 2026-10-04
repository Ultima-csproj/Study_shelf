use std::time::SystemTime;

use super::record::PublishedRevision;
use super::{
    Actor, Material, MaterialError, MaterialRevision, ModerationAction, ModerationDecision,
    RevisionState, policy,
};

impl Material {
    /// In-memory transition only. Persistence must update revision, pointer, search and audit
    /// in one transaction, rechecking the current base under concurrent writes.
    pub fn approve_revision(
        &mut self,
        actor: &Actor<'_>,
        revision: &mut MaterialRevision,
        at: SystemTime,
    ) -> Result<ModerationAction, MaterialError> {
        revision.ensure_material(self)?;
        revision.ensure_state(RevisionState::PendingModeration)?;
        if !policy::can_moderate_revision(actor, self, revision) {
            return Err(MaterialError::PermissionDenied);
        }
        revision.ensure_current_base(self)?;
        revision.ensure_timestamp(at)?;
        if self
            .public_updated_at()
            .is_some_and(|previous| at < previous)
        {
            return Err(MaterialError::InvalidTimestamp);
        }

        let action = ModerationAction::new(
            actor.user.id,
            revision,
            ModerationDecision::Approved,
            None,
            at,
        );
        revision.state = RevisionState::Published;
        revision.published_at = Some(at);
        revision.updated_at = at;
        self.published_revision = Some(PublishedRevision {
            id: revision.id(),
            published_at: at,
        });
        Ok(action)
    }

    pub fn reject_revision(
        &self,
        actor: &Actor<'_>,
        revision: &mut MaterialRevision,
        reason: String,
        at: SystemTime,
    ) -> Result<ModerationAction, MaterialError> {
        revision.ensure_material(self)?;
        revision.ensure_state(RevisionState::PendingModeration)?;
        if !policy::can_moderate_revision(actor, self, revision) {
            return Err(MaterialError::PermissionDenied);
        }
        if reason.trim().is_empty() {
            return Err(MaterialError::InvalidContent(
                super::ContentError::EmptyRejectionReason,
            ));
        }
        revision.ensure_timestamp(at)?;

        let action = ModerationAction::new(
            actor.user.id,
            revision,
            ModerationDecision::Rejected,
            Some(reason),
            at,
        );
        revision.state = RevisionState::Rejected;
        revision.updated_at = at;
        Ok(action)
    }
}
