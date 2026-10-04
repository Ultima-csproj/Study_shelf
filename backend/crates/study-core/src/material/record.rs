use std::time::SystemTime;

use crate::ids::{MaterialId, MaterialRevisionId, UserId};

use super::{Actor, MaterialError, MaterialOwner, policy};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialVisibility {
    Private,
    Public,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct PublishedRevision {
    pub id: MaterialRevisionId,
    pub published_at: SystemTime,
}

/// Stable identity. Public metadata is read from its published revision.
#[derive(Debug, Clone)]
pub struct Material {
    pub(super) id: MaterialId,
    pub(super) owner: MaterialOwner,
    pub(super) created_by_user_id: UserId,
    pub(super) created_at: SystemTime,
    pub(super) visibility: MaterialVisibility,
    pub(super) published_revision: Option<PublishedRevision>,
}

impl Material {
    pub fn new(
        actor: &Actor<'_>,
        id: MaterialId,
        owner: MaterialOwner,
        visibility: MaterialVisibility,
        created_at: SystemTime,
    ) -> Result<Self, MaterialError> {
        if id.0 <= 0 {
            return Err(MaterialError::InvalidIdentifier("material"));
        }
        if !policy::can_create_material(actor, owner) {
            return Err(MaterialError::PermissionDenied);
        }
        Ok(Self {
            id,
            owner,
            created_by_user_id: actor.user.id,
            created_at,
            visibility,
            published_revision: None,
        })
    }

    pub fn id(&self) -> MaterialId {
        self.id
    }

    pub fn owner(&self) -> MaterialOwner {
        self.owner
    }

    pub fn created_by_user_id(&self) -> UserId {
        self.created_by_user_id
    }

    pub fn created_at(&self) -> SystemTime {
        self.created_at
    }

    pub fn visibility(&self) -> MaterialVisibility {
        self.visibility
    }

    pub fn published_revision_id(&self) -> Option<MaterialRevisionId> {
        self.published_revision.map(|revision| revision.id)
    }

    pub fn public_updated_at(&self) -> Option<SystemTime> {
        self.published_revision
            .map(|revision| revision.published_at)
    }

    pub fn is_publicly_available(&self) -> bool {
        self.visibility == MaterialVisibility::Public && self.published_revision.is_some()
    }

    pub fn set_visibility(
        &mut self,
        actor: &Actor<'_>,
        visibility: MaterialVisibility,
    ) -> Result<(), MaterialError> {
        if !policy::can_manage_material(actor, self) {
            return Err(MaterialError::PermissionDenied);
        }
        self.visibility = visibility;
        Ok(())
    }
}
