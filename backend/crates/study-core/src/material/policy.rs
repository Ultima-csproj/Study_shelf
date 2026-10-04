use crate::identity::PlatformPermission;
use crate::institution::InstitutionPermission;

use super::{Actor, Material, MaterialOwner, MaterialPermission, MaterialRevision, RevisionState};

pub fn can_create_material(actor: &Actor<'_>, owner: MaterialOwner) -> bool {
    if !actor.user.is_active() {
        return false;
    }
    match owner {
        MaterialOwner::User(user_id) => actor.user.id == user_id,
        MaterialOwner::Institution(institution_id) => {
            institution_id.0 > 0
                && actor.has_institution_permission(
                    institution_id,
                    InstitutionPermission::CreateMaterials,
                )
        }
    }
}

pub fn can_manage_material(actor: &Actor<'_>, material: &Material) -> bool {
    if !actor.user.is_active() {
        return false;
    }
    match material.owner() {
        MaterialOwner::User(user_id) => actor.user.id == user_id,
        MaterialOwner::Institution(institution_id) => {
            actor.has_institution_permission(institution_id, InstitutionPermission::ManageMaterials)
        }
    }
}

pub fn can_create_revision(actor: &Actor<'_>, material: &Material) -> bool {
    can_manage_material(actor, material)
        || actor.has_material_permission(material.id(), MaterialPermission::EditRevision)
}

pub fn can_edit_revision(
    actor: &Actor<'_>,
    material: &Material,
    revision: &MaterialRevision,
) -> bool {
    revision.material_id() == material.id()
        && revision.state() == RevisionState::Draft
        && can_create_revision(actor, material)
}

pub fn can_manage_collaborators(actor: &Actor<'_>, material: &Material) -> bool {
    can_manage_material(actor, material)
        || actor.has_material_permission(material.id(), MaterialPermission::ManageCollaborators)
}

/// Acceptance remains an owner operation; a proposal workflow is added later.
pub fn can_accept_proposal(actor: &Actor<'_>, material: &Material) -> bool {
    can_manage_material(actor, material)
}

pub fn can_submit_revision(
    actor: &Actor<'_>,
    material: &Material,
    revision: &MaterialRevision,
) -> bool {
    revision.material_id() == material.id()
        && revision.state() == RevisionState::Draft
        && (can_manage_material(actor, material)
            || actor
                .has_material_permission(material.id(), MaterialPermission::SubmitForModeration))
}

pub fn can_moderate_revision(
    actor: &Actor<'_>,
    material: &Material,
    revision: &MaterialRevision,
) -> bool {
    revision.material_id() == material.id()
        && revision.state() == RevisionState::PendingModeration
        && material.visibility() == super::MaterialVisibility::Public
        && actor
            .user
            .has_permission(PlatformPermission::ModeratePublicMaterials)
}

pub fn can_read_material(actor: Option<&Actor<'_>>, material: &Material) -> bool {
    if material.is_publicly_available() {
        return true;
    }
    can_read_nonpublic_material(actor, material)
}

/// Public material access must not expose its draft or a historical revision by ID.
pub fn can_read_revision(
    actor: Option<&Actor<'_>>,
    material: &Material,
    revision: &MaterialRevision,
) -> bool {
    if revision.material_id() != material.id() {
        return false;
    }
    if material.is_publicly_available()
        && revision.state() == RevisionState::Published
        && material.published_revision_id() == Some(revision.id())
    {
        return true;
    }
    can_read_nonpublic_material(actor, material)
        || actor.is_some_and(|actor| can_moderate_revision(actor, material, revision))
}

fn can_read_nonpublic_material(actor: Option<&Actor<'_>>, material: &Material) -> bool {
    let Some(actor) = actor else {
        return false;
    };
    can_manage_material(actor, material)
        || [
            MaterialPermission::Read,
            MaterialPermission::EditRevision,
            MaterialPermission::SubmitForModeration,
            MaterialPermission::ManageCollaborators,
        ]
        .into_iter()
        .any(|permission| actor.has_material_permission(material.id(), permission))
}
