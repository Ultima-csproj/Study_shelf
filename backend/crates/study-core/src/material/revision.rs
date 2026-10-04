use std::time::SystemTime;

use crate::ids::{MaterialId, MaterialRevisionId, UserId};

use super::{Material, MaterialError, RevisionContent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevisionState {
    Draft,
    PendingModeration,
    Published,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct MaterialRevision {
    pub(super) id: MaterialRevisionId,
    pub(super) material_id: MaterialId,
    pub(super) base_revision_id: Option<MaterialRevisionId>,
    pub(super) created_by_user_id: UserId,
    pub(super) created_at: SystemTime,
    pub(super) updated_at: SystemTime,
    pub(super) state: RevisionState,
    pub(super) content: RevisionContent,
    pub(super) changed_by: Vec<UserId>,
    pub(super) submitted_by_user_id: Option<UserId>,
    pub(super) submitted_at: Option<SystemTime>,
    pub(super) published_at: Option<SystemTime>,
}

impl MaterialRevision {
    pub fn id(&self) -> MaterialRevisionId {
        self.id
    }
    pub fn material_id(&self) -> MaterialId {
        self.material_id
    }
    pub fn base_revision_id(&self) -> Option<MaterialRevisionId> {
        self.base_revision_id
    }
    pub fn created_by_user_id(&self) -> UserId {
        self.created_by_user_id
    }
    pub fn created_at(&self) -> SystemTime {
        self.created_at
    }
    pub fn updated_at(&self) -> SystemTime {
        self.updated_at
    }
    pub fn state(&self) -> RevisionState {
        self.state
    }
    pub fn content(&self) -> &RevisionContent {
        &self.content
    }
    pub fn changed_by(&self) -> &[UserId] {
        &self.changed_by
    }
    pub fn submitted_by_user_id(&self) -> Option<UserId> {
        self.submitted_by_user_id
    }
    pub fn submitted_at(&self) -> Option<SystemTime> {
        self.submitted_at
    }
    pub fn published_at(&self) -> Option<SystemTime> {
        self.published_at
    }

    pub fn has_participated(&self, user_id: UserId) -> bool {
        self.created_by_user_id == user_id
            || self.changed_by.contains(&user_id)
            || self.submitted_by_user_id == Some(user_id)
    }

    /// Staleness applies to unpublished work; a published base is historical provenance.
    pub fn is_outdated_for(&self, material: &Material) -> Result<bool, MaterialError> {
        self.ensure_material(material)?;
        Ok(self.state != RevisionState::Published
            && self.base_revision_id != material.published_revision_id())
    }

    pub(super) fn ensure_material(&self, material: &Material) -> Result<(), MaterialError> {
        if self.material_id != material.id() {
            return Err(MaterialError::RevisionMaterialMismatch {
                material_id: material.id(),
                revision_material_id: self.material_id,
            });
        }
        Ok(())
    }

    pub(super) fn ensure_state(&self, expected: RevisionState) -> Result<(), MaterialError> {
        if self.state != expected {
            return Err(MaterialError::InvalidRevisionState {
                expected,
                actual: self.state,
            });
        }
        Ok(())
    }

    pub(super) fn ensure_current_base(&self, material: &Material) -> Result<(), MaterialError> {
        self.ensure_material(material)?;
        if self.base_revision_id != material.published_revision_id() {
            return Err(MaterialError::OutdatedBase {
                base_revision_id: self.base_revision_id,
                published_revision_id: material.published_revision_id(),
            });
        }
        Ok(())
    }

    pub(super) fn ensure_timestamp(&self, at: SystemTime) -> Result<(), MaterialError> {
        if at < self.updated_at {
            return Err(MaterialError::InvalidTimestamp);
        }
        Ok(())
    }
}
