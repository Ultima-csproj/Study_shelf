mod audit;
mod collaborators;
mod contributors;
mod drafts;
mod learning;
mod published;
mod read;
mod restrictions;
mod state;
mod transitions;
mod types;
use super::{
    MaterialWriteError, PostgresMaterialRepository, access, load,
    write::{Entity, owner_institution},
};
use crate::{account::session, error::map_sqlx_error};
pub use contributors::ContributorChoice;
use sqlx::{Postgres, Transaction};
use study_core::ids::{MaterialId, UserId};
use study_core::material::{Material, MaterialError};
pub use types::*;
pub(crate) type Result<T> = std::result::Result<T, MaterialWriteError>;

impl PostgresMaterialRepository {
    async fn begin_session(&self, digest: &[u8]) -> Result<(Transaction<'_, Postgres>, UserId)> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let user = session::require(&mut tx, digest)
            .await
            .map_err(account_error)?
            .user_id;
        Ok((tx, UserId(user)))
    }
}

fn account_error(error: study_core::account::AccountError) -> MaterialWriteError {
    MaterialWriteError::Account(error)
}

fn reason(value: &str) -> Result<()> {
    if !(3..=2000).contains(&value.trim().chars().count()) {
        return Err(MaterialWriteError::InvalidReason);
    }
    Ok(())
}

async fn locked(
    tx: &mut sqlx::PgConnection,
    user: UserId,
    id: MaterialId,
) -> Result<(Material, access::ActorData)> {
    let material = load::material(tx, id, true)
        .await?
        .ok_or(MaterialWriteError::NotFound(Entity::Material))?;
    let actor = access::load(tx, user, Some(id), owner_institution(material.owner())).await?;
    Ok((material, actor))
}

async fn material_version(
    tx: &mut sqlx::PgConnection,
    id: MaterialId,
    expected: i64,
) -> Result<()> {
    let version: i64 = sqlx::query_scalar(include_str!("sql/material_version.sql"))
        .bind(id.0)
        .fetch_one(tx)
        .await
        .map_err(map_sqlx_error)?;
    if version != expected {
        return Err(MaterialWriteError::MaterialVersionConflict);
    }
    Ok(())
}

async fn revision_version(
    tx: &mut sqlx::PgConnection,
    material: &Material,
    id: study_core::ids::MaterialRevisionId,
    expected: i64,
) -> Result<super::StoredRevision> {
    let stored = load::revision(tx, id, true)
        .await?
        .ok_or(MaterialWriteError::NotFound(Entity::Revision))?;
    if stored.revision.material_id() != material.id() {
        return Err(MaterialError::RevisionMaterialMismatch {
            material_id: material.id(),
            revision_material_id: stored.revision.material_id(),
        }
        .into());
    }
    if stored.edit_version != expected {
        return Err(MaterialWriteError::DraftConflict {
            expected,
            current: stored.edit_version,
        });
    }
    Ok(stored)
}

async fn working_available(tx: &mut sqlx::PgConnection, material: MaterialId) -> Result<()> {
    let exists: bool = sqlx::query_scalar(include_str!("sql/working_exists.sql"))
        .bind(material.0)
        .fetch_one(tx)
        .await
        .map_err(map_sqlx_error)?;
    if exists {
        return Err(MaterialWriteError::WorkingRevisionExists);
    }
    Ok(())
}
