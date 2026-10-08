use std::time::SystemTime;

use crate::ids::MaterialRevisionId;

use super::{
    Actor, Material, MaterialError, MaterialRevision, RevisionContent, RevisionState, policy,
};

impl MaterialRevision {
    pub fn new_draft(
        actor: &Actor<'_>,
        material: &Material,
        id: MaterialRevisionId,
        content: RevisionContent,
        created_at: SystemTime,
    ) -> Result<Self, MaterialError> {
        if id.0 <= 0 {
            return Err(MaterialError::InvalidIdentifier("revision"));
        }
        if Some(id) == material.published_revision_id() {
            return Err(MaterialError::RevisionIdReused);
        }
        if !policy::can_create_revision(actor, material) {
            return Err(MaterialError::PermissionDenied);
        }
        if created_at < material.created_at()
            || material
                .public_updated_at()
                .is_some_and(|published_at| created_at < published_at)
        {
            return Err(MaterialError::InvalidTimestamp);
        }
        content.validate()?;

        Ok(Self {
            id,
            material_id: material.id(),
            base_revision_id: material.published_revision_id(),
            created_by_user_id: actor.user.id,
            created_at,
            updated_at: created_at,
            state: RevisionState::Draft,
            content,
            changed_by: Vec::new(),
            submitted_by_user_id: None,
            submitted_at: None,
            published_at: None,
        })
    }

    pub fn replace_content(
        &mut self,
        actor: &Actor<'_>,
        material: &Material,
        content: RevisionContent,
        at: SystemTime,
    ) -> Result<(), MaterialError> {
        self.ensure_material(material)?;
        self.ensure_state(RevisionState::Draft)?;
        if !policy::can_edit_revision(actor, material, self) {
            return Err(MaterialError::PermissionDenied);
        }
        self.ensure_timestamp(at)?;
        content.validate()?;

        if self.content != content {
            if !self.changed_by.contains(&actor.user.id) {
                self.changed_by.push(actor.user.id);
            }
            self.content = content;
            self.updated_at = at;
        }
        Ok(())
    }

    pub fn submit_for_moderation(
        &mut self,
        actor: &Actor<'_>,
        material: &Material,
        at: SystemTime,
    ) -> Result<(), MaterialError> {
        self.ensure_material(material)?;
        self.ensure_state(RevisionState::Draft)?;
        if !policy::can_submit_revision(actor, material, self) {
            return Err(MaterialError::PermissionDenied);
        }
        if material.lifecycle() != super::MaterialLifecycle::Active {
            return Err(MaterialError::MaterialArchived);
        }
        self.ensure_current_base(material)?;
        self.ensure_timestamp(at)?;

        self.submitted_by_user_id = Some(actor.user.id);
        self.submitted_at = Some(at);
        self.updated_at = at;
        self.state = RevisionState::PendingModeration;
        Ok(())
    }

    pub fn withdraw(
        &mut self,
        actor: &Actor<'_>,
        material: &Material,
        at: SystemTime,
    ) -> Result<(), MaterialError> {
        self.ensure_material(material)?;
        self.ensure_state(RevisionState::PendingModeration)?;
        if !policy::can_create_revision(actor, material) {
            return Err(MaterialError::PermissionDenied);
        }
        self.ensure_timestamp(at)?;
        self.state = RevisionState::Draft;
        self.submitted_by_user_id = None;
        self.submitted_at = None;
        self.updated_at = at;
        Ok(())
    }

    pub fn discard(
        &mut self,
        actor: &Actor<'_>,
        material: &Material,
        at: SystemTime,
    ) -> Result<(), MaterialError> {
        self.ensure_material(material)?;
        self.ensure_state(RevisionState::Draft)?;
        if !policy::can_edit_revision(actor, material, self) {
            return Err(MaterialError::PermissionDenied);
        }
        self.ensure_timestamp(at)?;
        self.state = RevisionState::Discarded;
        self.updated_at = at;
        Ok(())
    }
}
