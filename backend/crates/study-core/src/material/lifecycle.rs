use super::{Actor, Material, MaterialError, MaterialVisibility, policy};
use crate::ids::InstitutionId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialLifecycle {
    Active,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationState {
    Published,
    Unpublished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestrictionScope {
    Global,
    Institution(InstitutionId),
}

impl Material {
    pub fn lifecycle(&self) -> MaterialLifecycle {
        self.lifecycle
    }

    pub fn publication(&self) -> PublicationState {
        self.publication
    }

    pub fn set_lifecycle(
        &mut self,
        actor: &Actor<'_>,
        lifecycle: MaterialLifecycle,
    ) -> Result<(), MaterialError> {
        if !policy::can_manage_material(actor, self) {
            return Err(MaterialError::PermissionDenied);
        }
        self.lifecycle = lifecycle;
        Ok(())
    }

    pub fn set_publication(
        &mut self,
        actor: &Actor<'_>,
        publication: PublicationState,
    ) -> Result<(), MaterialError> {
        if !policy::can_manage_material(actor, self) {
            return Err(MaterialError::PermissionDenied);
        }
        if publication == PublicationState::Published {
            if self.published_revision_id().is_none() {
                return Err(MaterialError::NoPublishedRevision);
            }
            // Making a formerly private publication public requires review of a new revision.
            if self.visibility() != MaterialVisibility::Public {
                return Err(MaterialError::MaterialIsPrivate);
            }
        }
        self.publication = publication;
        Ok(())
    }
}
