use super::super::{MaterialWriteError, PostgresMaterialRepository, access};
use super::{Result, audit, locked, material_version, reason};
use crate::error::map_sqlx_error;
use serde_json::json;
use study_core::identity::PlatformPermission;
use study_core::ids::{InstitutionId, MaterialId};
use study_core::institution::InstitutionPermission;
use study_core::material::{MaterialError, MaterialVisibility, RestrictionScope};

async fn allowed(
    tx: &mut sqlx::PgConnection,
    user: study_core::ids::UserId,
    material: &study_core::material::Material,
    institution: Option<i64>,
) -> Result<()> {
    let scope = institution
        .map(|id| RestrictionScope::Institution(InstitutionId(id)))
        .unwrap_or(RestrictionScope::Global);
    let actor = access::load(
        tx,
        user,
        Some(material.id()),
        institution.map(InstitutionId),
    )
    .await?;
    let allowed = match scope {
        RestrictionScope::Global => actor
            .actor()
            .user
            .has_permission(PlatformPermission::RestrictPublicMaterials),
        RestrictionScope::Institution(id) => actor
            .actor()
            .has_institution_permission(id, InstitutionPermission::RestrictMaterials),
    };
    if !allowed {
        return Err(MaterialError::PermissionDenied.into());
    }
    Ok(())
}

impl PostgresMaterialRepository {
    pub async fn restriction_target(
        &self,
        digest: &[u8],
        id: MaterialId,
    ) -> Result<super::RestrictionTarget> {
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
        if material.visibility() != MaterialVisibility::Public
            || material.published_revision_id().is_none()
        {
            return Err(MaterialError::PermissionDenied.into());
        }
        let actor = access::load_snapshot(&mut tx, user, None, None).await?;
        let global = actor
            .actor()
            .user
            .has_permission(PlatformPermission::RestrictPublicMaterials);
        let institutions: Vec<i64> =
            sqlx::query_scalar(include_str!("sql/restriction_target_scopes.sql"))
                .bind(id.0)
                .bind(user.0)
                .fetch_all(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
        if !global && institutions.is_empty() {
            return Err(MaterialError::PermissionDenied.into());
        }
        let header = super::read::header(&mut tx, id, "restricted").await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(super::RestrictionTarget {
            material: header,
            global,
            institutions,
        })
    }

    pub async fn restrict_authenticated(
        &self,
        digest: &[u8],
        id: MaterialId,
        expected: i64,
        institution: Option<i64>,
        cause: &str,
    ) -> Result<()> {
        reason(cause)?;
        let (mut tx, user) = self.begin_session(digest).await?;
        let (material, _) = locked(&mut tx, user, id).await?;
        material_version(&mut tx, id, expected).await?;
        allowed(&mut tx, user, &material, institution).await?;
        if material.visibility() != MaterialVisibility::Public
            || material.published_revision_id().is_none()
        {
            return Err(MaterialError::PermissionDenied.into());
        }
        if let Some(institution) = institution {
            let attached: bool = sqlx::query_scalar(include_str!("sql/restriction_scope.sql"))
                .bind(id.0)
                .bind(institution)
                .fetch_one(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
            if !attached {
                return Err(MaterialWriteError::ScopeMismatch);
            }
        }
        let exists: bool = sqlx::query_scalar(include_str!("sql/restriction_exists.sql"))
            .bind(id.0)
            .bind(institution)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        if exists {
            return Err(MaterialWriteError::RestrictionExists);
        }
        let restriction: i64 = sqlx::query_scalar(include_str!("sql/restrict.sql"))
            .bind(id.0)
            .bind(institution)
            .bind(user.0)
            .bind(cause.trim())
            .fetch_one(&mut *tx)
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
            "restrict_material",
            cause,
            json!({"restrictionId":restriction,"institutionId":institution}),
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }

    pub async fn unrestrict_authenticated(
        &self,
        digest: &[u8],
        id: MaterialId,
        expected: i64,
        restriction: i64,
        cause: &str,
    ) -> Result<()> {
        reason(cause)?;
        let (mut tx, user) = self.begin_session(digest).await?;
        let (material, _) = locked(&mut tx, user, id).await?;
        material_version(&mut tx, id, expected).await?;
        let scope: Option<Option<i64>> =
            sqlx::query_scalar(include_str!("sql/restriction_target.sql"))
                .bind(restriction)
                .bind(id.0)
                .fetch_optional(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
        let institution = scope.ok_or(MaterialWriteError::NotFound(
            super::super::write::Entity::Material,
        ))?;
        allowed(&mut tx, user, &material, institution).await?;
        sqlx::query(include_str!("sql/unrestrict.sql"))
            .bind(restriction)
            .bind(user.0)
            .bind(cause.trim())
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
            "unrestrict_material",
            cause,
            json!({"restrictionId":restriction,"institutionId":institution}),
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
}

impl PostgresMaterialRepository {
    /// Restriction metadata does not grant access to private content or drafts.
    pub async fn restriction_metadata(
        &self,
        digest: &[u8],
        id: MaterialId,
    ) -> Result<Vec<super::RestrictionRow>> {
        let mut tx = crate::account::session::snapshot(&self.pool).await?;
        let user = crate::account::session::read(&mut tx, digest)
            .await?
            .user_id;
        let rows: Vec<super::RestrictionRow> =
            sqlx::query_as(include_str!("sql/restriction_metadata.sql"))
                .bind(id.0)
                .bind(user)
                .fetch_all(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
        if rows.is_empty() {
            return Err(study_core::account::AccountError::NotFound.into());
        }
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(rows)
    }
}
