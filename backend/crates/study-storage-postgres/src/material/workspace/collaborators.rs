use super::super::PostgresMaterialRepository;
use super::{CollaboratorRow, MaterialEvent, Result, audit, locked, material_version, reason};
use crate::{
    account::session,
    administration::{Page, PageRequest},
    error::map_sqlx_error,
};
use serde_json::json;
use study_core::ids::{MaterialId, UserId};
use study_core::material::{MaterialError, MaterialPermission, policy};

impl PostgresMaterialRepository {
    pub async fn collaborators(
        &self,
        digest: &[u8],
        id: MaterialId,
        p: &PageRequest,
    ) -> Result<Page<CollaboratorRow>> {
        p.validate()?;
        let mut tx = session::snapshot(&self.pool).await?;
        let user = UserId(session::read(&mut tx, digest).await?.user_id);
        let material = super::super::load::material(&mut tx, id, false)
            .await?
            .ok_or(super::super::MaterialWriteError::NotFound(
                super::super::write::Entity::Material,
            ))?;
        let actor = super::super::access::load_snapshot(
            &mut tx,
            user,
            Some(id),
            super::super::write::owner_institution(material.owner()),
        )
        .await?;
        if !policy::can_manage_collaborators(&actor.actor(), &material) {
            return Err(MaterialError::PermissionDenied.into());
        }
        let total = sqlx::query_scalar(include_str!("sql/collaborator_count.sql"))
            .bind(id.0)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let items = sqlx::query_as(include_str!("sql/collaborators.sql"))
            .bind(id.0)
            .bind(i64::from(p.size))
            .bind(p.offset())
            .fetch_all(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(p.finish(items, total))
    }

    pub async fn set_collaborator_authenticated(
        &self,
        digest: &[u8],
        id: MaterialId,
        expected: i64,
        change: super::CollaboratorChange,
        cause: &str,
    ) -> Result<()> {
        let super::CollaboratorChange {
            user: target,
            active,
            permissions,
        } = change;
        reason(cause)?;
        let (mut tx, user) = self.begin_session(digest).await?;
        let (material, actor) = locked(&mut tx, user, id).await?;
        material_version(&mut tx, id, expected).await?;
        if !policy::can_manage_collaborators(&actor.actor(), &material) {
            return Err(MaterialError::PermissionDenied.into());
        }
        let state: Option<String> = sqlx::query_scalar(include_str!("sql/target_user.sql"))
            .bind(target.0)
            .fetch_optional(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        if state.is_none() {
            return Err(study_core::account::AccountError::NotFound.into());
        }
        if active && state.as_deref() != Some("active") {
            return Err(study_core::account::AccountError::AccountDisabled.into());
        }
        let mut values: Vec<&str> = if active {
            permissions
                .iter()
                .map(|permission| match permission {
                    MaterialPermission::Read => "read",
                    MaterialPermission::EditRevision => "edit_revision",
                    MaterialPermission::SubmitForModeration => "submit_for_moderation",
                    MaterialPermission::ManageCollaborators => "manage_collaborators",
                })
                .collect()
        } else {
            Vec::new()
        };
        values.sort_unstable();
        values.dedup();
        sqlx::query(include_str!("sql/upsert_collaborator.sql"))
            .bind(id.0)
            .bind(target.0)
            .bind(active)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        sqlx::query(include_str!("sql/clear_collaborator_permissions.sql"))
            .bind(id.0)
            .bind(target.0)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        sqlx::query(include_str!("sql/collaborator_permissions.sql"))
            .bind(id.0)
            .bind(target.0)
            .bind(&values)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        sqlx::query(include_str!("sql/bump_material.sql"))
            .bind(id.0)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        audit::record(
            &mut tx,
            user,
            id,
            None,
            "set_collaborator",
            cause,
            json!({"userId":target.0,"active":active,"permissions":values}),
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }

    pub async fn material_events(
        &self,
        digest: &[u8],
        id: MaterialId,
        p: &PageRequest,
    ) -> Result<Page<MaterialEvent>> {
        // workspace checks the same private access; the actual read remains in one fresh snapshot.
        p.validate()?;
        let mut tx = session::snapshot(&self.pool).await?;
        let user = UserId(session::read(&mut tx, digest).await?.user_id);
        let material = super::super::load::material(&mut tx, id, false)
            .await?
            .ok_or(study_core::account::AccountError::NotFound)?;
        let actor = super::super::access::load_snapshot(
            &mut tx,
            user,
            Some(id),
            super::super::write::owner_institution(material.owner()),
        )
        .await?;
        if !policy::can_read_nonpublic_material(Some(&actor.actor()), &material) {
            return Err(MaterialError::PermissionDenied.into());
        }
        let total = sqlx::query_scalar(include_str!("sql/event_count.sql"))
            .bind(id.0)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let items = sqlx::query_as(include_str!("sql/events.sql"))
            .bind(id.0)
            .bind(i64::from(p.size))
            .bind(p.offset())
            .fetch_all(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(p.finish(items, total))
    }
}
