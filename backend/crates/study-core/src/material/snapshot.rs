use super::record::PublishedRevision;
use super::{
    Material, MaterialOwner, MaterialRevision, MaterialVisibility, RevisionContent, RevisionState,
};
use crate::ids::{MaterialId, MaterialRevisionId, UserId};
use std::collections::HashSet;
use std::time::SystemTime;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SnapshotError {
    #[error("invalid stored identifier")]
    InvalidIdentifier,
    #[error("invalid stored timestamps")]
    InvalidTimestamps,
    #[error("state and lifecycle fields disagree")]
    InvalidLifecycle,
    #[error("revision cannot use itself as its base")]
    SelfReference,
    #[error("duplicate revision participant")]
    DuplicateParticipant,
    #[error("invalid stored revision content")]
    InvalidContent(#[source] super::MaterialError),
}

pub struct MaterialSnapshot {
    pub id: MaterialId,
    pub owner: MaterialOwner,
    pub created_by_user_id: UserId,
    pub created_at: SystemTime,
    pub visibility: MaterialVisibility,
    pub lifecycle: super::MaterialLifecycle,
    pub publication: super::PublicationState,
    pub published_revision: Option<(MaterialRevisionId, SystemTime)>,
}

impl Material {
    pub fn try_restore(snapshot: MaterialSnapshot) -> Result<Self, SnapshotError> {
        let owner_id = match snapshot.owner {
            MaterialOwner::User(id) => id.0,
            MaterialOwner::Institution(id) => id.0,
        };
        if snapshot.id.0 <= 0 || snapshot.created_by_user_id.0 <= 0 || owner_id <= 0 {
            return Err(SnapshotError::InvalidIdentifier);
        }
        if let Some((id, at)) = snapshot.published_revision {
            if id.0 <= 0 {
                return Err(SnapshotError::InvalidIdentifier);
            }
            if at < snapshot.created_at {
                return Err(SnapshotError::InvalidTimestamps);
            }
        }
        if snapshot.publication == super::PublicationState::Published
            && snapshot.published_revision.is_none()
        {
            return Err(SnapshotError::InvalidLifecycle);
        }
        Ok(Self {
            id: snapshot.id,
            owner: snapshot.owner,
            created_by_user_id: snapshot.created_by_user_id,
            created_at: snapshot.created_at,
            visibility: snapshot.visibility,
            lifecycle: snapshot.lifecycle,
            publication: snapshot.publication,
            published_revision: snapshot
                .published_revision
                .map(|(id, published_at)| PublishedRevision { id, published_at }),
        })
    }
}

pub struct RevisionSnapshot {
    pub id: MaterialRevisionId,
    pub material_id: MaterialId,
    pub base_revision_id: Option<MaterialRevisionId>,
    pub created_by_user_id: UserId,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub state: RevisionState,
    pub content: RevisionContent,
    pub changed_by: Vec<UserId>,
    pub submitted_by_user_id: Option<UserId>,
    pub submitted_at: Option<SystemTime>,
    pub published_at: Option<SystemTime>,
}

impl MaterialRevision {
    pub fn try_restore(snapshot: RevisionSnapshot) -> Result<Self, SnapshotError> {
        if snapshot.id.0 <= 0
            || snapshot.material_id.0 <= 0
            || snapshot.created_by_user_id.0 <= 0
            || snapshot.base_revision_id.is_some_and(|id| id.0 <= 0)
            || snapshot.submitted_by_user_id.is_some_and(|id| id.0 <= 0)
            || snapshot.changed_by.iter().any(|id| id.0 <= 0)
        {
            return Err(SnapshotError::InvalidIdentifier);
        }
        if snapshot.base_revision_id == Some(snapshot.id) {
            return Err(SnapshotError::SelfReference);
        }
        let unique: HashSet<_> = snapshot.changed_by.iter().collect();
        if unique.len() != snapshot.changed_by.len() {
            return Err(SnapshotError::DuplicateParticipant);
        }
        if snapshot.updated_at < snapshot.created_at
            || snapshot
                .submitted_at
                .is_some_and(|at| at < snapshot.created_at || at > snapshot.updated_at)
            || snapshot.published_at.is_some_and(|at| {
                snapshot.submitted_at.is_none_or(|submitted| at < submitted)
                    || at != snapshot.updated_at
            })
        {
            return Err(SnapshotError::InvalidTimestamps);
        }
        let submitted = snapshot.submitted_at.is_some() && snapshot.submitted_by_user_id.is_some();
        let valid = match snapshot.state {
            RevisionState::Draft | RevisionState::Discarded => {
                snapshot.submitted_at.is_none()
                    && snapshot.submitted_by_user_id.is_none()
                    && snapshot.published_at.is_none()
            }
            RevisionState::PendingModeration | RevisionState::Rejected => {
                submitted && snapshot.published_at.is_none()
            }
            RevisionState::Published => submitted && snapshot.published_at.is_some(),
        };
        if !valid {
            return Err(SnapshotError::InvalidLifecycle);
        }
        snapshot
            .content
            .validate()
            .map_err(SnapshotError::InvalidContent)?;
        Ok(Self {
            id: snapshot.id,
            material_id: snapshot.material_id,
            base_revision_id: snapshot.base_revision_id,
            created_by_user_id: snapshot.created_by_user_id,
            created_at: snapshot.created_at,
            updated_at: snapshot.updated_at,
            state: snapshot.state,
            content: snapshot.content,
            changed_by: snapshot.changed_by,
            submitted_by_user_id: snapshot.submitted_by_user_id,
            submitted_at: snapshot.submitted_at,
            published_at: snapshot.published_at,
        })
    }
}
