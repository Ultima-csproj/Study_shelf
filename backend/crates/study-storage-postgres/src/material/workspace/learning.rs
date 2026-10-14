use super::super::{MaterialWriteError, PostgresMaterialRepository, access};
use super::{Result, audit, material_version, reason};
use crate::error::map_sqlx_error;
use serde_json::json;
use study_core::{
    ids::{InstitutionId, MaterialId},
    institution::InstitutionPermission,
    material::MaterialError,
};

impl PostgresMaterialRepository {
    pub async fn placement_entries(
        &self,
        digest: &[u8],
        id: MaterialId,
        placement: i64,
        p: &crate::administration::PageRequest,
    ) -> Result<crate::administration::Page<super::PlacementEntryRow>> {
        p.validate()?;
        let mut tx = crate::account::session::snapshot(&self.pool).await?;
        let user = study_core::ids::UserId(
            crate::account::session::read(&mut tx, digest)
                .await?
                .user_id,
        );
        let material = super::super::load::material(&mut tx, id, false)
            .await?
            .ok_or(MaterialWriteError::NotFound(
                super::super::write::Entity::Material,
            ))?;
        let _institution: i64 = sqlx::query_scalar(include_str!("sql/placement_context.sql"))
            .bind(placement)
            .bind(id.0)
            .fetch_optional(&mut *tx)
            .await
            .map_err(map_sqlx_error)?
            .ok_or(MaterialWriteError::ScopeMismatch)?;
        let actor = access::load_snapshot(
            &mut tx,
            user,
            Some(id),
            super::super::write::owner_institution(material.owner()),
        )
        .await?;
        if !study_core::material::policy::can_read_nonpublic_material(
            Some(&actor.actor()),
            &material,
        ) {
            return Err(MaterialError::PermissionDenied.into());
        }
        let total = sqlx::query_scalar(include_str!("sql/placement_entry_count.sql"))
            .bind(placement)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let items = sqlx::query_as(include_str!("sql/placement_entries.sql"))
            .bind(placement)
            .bind(i64::from(p.size))
            .bind(p.offset())
            .fetch_all(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(p.finish(items, total))
    }

    pub async fn detach_placement_authenticated(
        &self,
        digest: &[u8],
        id: MaterialId,
        expected: i64,
        placement: i64,
        cause: &str,
    ) -> Result<u64> {
        reason(cause)?;
        let (mut tx, user) = self.begin_session(digest).await?;
        super::locked(&mut tx, user, id).await?;
        material_version(&mut tx, id, expected).await?;
        let institution: Option<i64> = sqlx::query_scalar(include_str!("sql/placement_target.sql"))
            .bind(placement)
            .bind(id.0)
            .fetch_optional(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let institution = institution.ok_or(MaterialWriteError::ScopeMismatch)?;
        let actor = access::load(&mut tx, user, Some(id), Some(InstitutionId(institution))).await?;
        if !actor.actor().has_institution_permission(
            InstitutionId(institution),
            InstitutionPermission::ManageDisciplines,
        ) {
            return Err(MaterialError::PermissionDenied.into());
        }
        let removed = sqlx::query(include_str!("sql/detach_placement_entries.sql"))
            .bind(placement)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?
            .rows_affected();
        sqlx::query(include_str!("sql/detach_placement.sql"))
            .bind(placement)
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
            "detach_placement",
            cause,
            json!({"placementId":placement,"removedEntries":removed}),
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(removed)
    }

    pub async fn detach_entry_authenticated(
        &self,
        digest: &[u8],
        id: MaterialId,
        expected: i64,
        entry: i64,
        cause: &str,
    ) -> Result<()> {
        reason(cause)?;
        let (mut tx, user) = self.begin_session(digest).await?;
        super::locked(&mut tx, user, id).await?;
        material_version(&mut tx, id, expected).await?;
        let institution: Option<i64> = sqlx::query_scalar(include_str!("sql/entry_target.sql"))
            .bind(entry)
            .bind(id.0)
            .fetch_optional(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let institution = institution.ok_or(MaterialWriteError::ScopeMismatch)?;
        let actor = access::load(&mut tx, user, Some(id), Some(InstitutionId(institution))).await?;
        if !actor.actor().has_institution_permission(
            InstitutionId(institution),
            InstitutionPermission::ManageDisciplines,
        ) {
            return Err(MaterialError::PermissionDenied.into());
        }
        sqlx::query(include_str!("sql/detach_entry.sql"))
            .bind(entry)
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
            "detach_entry",
            cause,
            json!({"entryId":entry}),
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
