use super::super::{
    PostgresMaterialRepository, access, load,
    write::{Entity, owner_institution},
};
use super::*;
use crate::{
    account::session,
    administration::{Page, PageRequest},
    error::map_sqlx_error,
};
use sqlx::PgConnection;
use std::sync::LazyLock;
use study_core::identity::PlatformPermission;
use study_core::material::policy;

const HEADER: &str = include_str!("sql/header.sql");
const FILTER: &str = include_str!("sql/list_filter.sql");
static ONE: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"
        {HEADER}
        WHERE m.id = $2
    "#
    )
});
static COUNT: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"
        WITH headers AS (
            {HEADER}
        )
        SELECT
            count(*)
        FROM headers AS h
        JOIN materials AS m
            ON m.id = h.id
        {FILTER}
    "#
    )
});
static LIST: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"
        WITH headers AS (
            {HEADER}
        )
        SELECT
            h.*
        FROM headers AS h
        JOIN materials AS m
            ON m.id = h.id
        {FILTER}
        ORDER BY h.id DESC
        LIMIT $5
        OFFSET $6
    "#
    )
});

pub(super) async fn header(
    tx: &mut PgConnection,
    id: MaterialId,
    mode: &str,
) -> Result<MaterialHeader> {
    sqlx::query_as(&**ONE)
        .bind(mode)
        .bind(id.0)
        .fetch_optional(tx)
        .await
        .map_err(map_sqlx_error)?
        .ok_or(MaterialWriteError::NotFound(Entity::Material))
}

pub(super) async fn revision_headers(
    tx: &mut PgConnection,
    id: MaterialId,
    p: &PageRequest,
    published_only: bool,
) -> Result<Page<RevisionHeader>> {
    p.validate()?;
    let state = if published_only {
        Some("published")
    } else {
        None
    };
    let total = sqlx::query_scalar(include_str!("sql/revision_count.sql"))
        .bind(id.0)
        .bind(state)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
    let items = sqlx::query_as(include_str!("sql/revision_headers.sql"))
        .bind(id.0)
        .bind(state)
        .bind(i64::from(p.size))
        .bind(p.offset())
        .fetch_all(tx)
        .await
        .map_err(map_sqlx_error)?;
    Ok(p.finish(items, total))
}

pub(super) async fn restrictions(
    tx: &mut PgConnection,
    id: MaterialId,
) -> Result<Vec<RestrictionRow>> {
    Ok(sqlx::query_as(include_str!("sql/restrictions.sql"))
        .bind(id.0)
        .fetch_all(tx)
        .await
        .map_err(map_sqlx_error)?)
}

impl PostgresMaterialRepository {
    pub async fn workspace_list(
        &self,
        digest: &[u8],
        kind: MaterialListKind,
        p: &PageRequest,
    ) -> Result<Page<MaterialHeader>> {
        p.validate()?;
        if p.state
            .as_deref()
            .is_some_and(|value| !matches!(value, "active" | "archived"))
        {
            return Err(study_core::account::AccountError::Validation(
                "Неизвестное состояние материала",
            )
            .into());
        }
        let mut tx = session::snapshot(&self.pool).await?;
        let user = session::read(&mut tx, digest).await?.user_id;
        if matches!(kind, MaterialListKind::Restricted) {
            let allowed: bool =
                sqlx::query_scalar(include_str!("sql/restriction_queue_access.sql"))
                    .bind(user)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(map_sqlx_error)?;
            if !allowed {
                return Err(MaterialError::PermissionDenied.into());
            }
        }
        let mode = match kind {
            MaterialListKind::Author => "author",
            MaterialListKind::Pending => "pending",
            MaterialListKind::Restricted => "restricted",
        };
        if matches!(kind, MaterialListKind::Pending) {
            let actor = access::load_snapshot(&mut tx, UserId(user), None, None).await?;
            if !actor
                .actor()
                .user
                .has_permission(PlatformPermission::ModeratePublicMaterials)
            {
                return Err(MaterialError::PermissionDenied.into());
            }
        }
        let total = sqlx::query_scalar(&**COUNT)
            .bind(mode)
            .bind(user)
            .bind(&p.query)
            .bind(&p.state)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let items = sqlx::query_as(&**LIST)
            .bind(mode)
            .bind(user)
            .bind(&p.query)
            .bind(&p.state)
            .bind(i64::from(p.size))
            .bind(p.offset())
            .fetch_all(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(p.finish(items, total))
    }

    pub async fn workspace(
        &self,
        digest: &[u8],
        id: MaterialId,
        p: &PageRequest,
    ) -> Result<Workspace> {
        let mut tx = session::snapshot(&self.pool).await?;
        let user = UserId(session::read(&mut tx, digest).await?.user_id);
        let material = load::material(&mut tx, id, false)
            .await?
            .ok_or(MaterialWriteError::NotFound(Entity::Material))?;
        let actor =
            access::load_snapshot(&mut tx, user, Some(id), owner_institution(material.owner()))
                .await?;
        if !policy::can_read_nonpublic_material(Some(&actor.actor()), &material) {
            return Err(MaterialError::PermissionDenied.into());
        }
        let result = header(&mut tx, id, "author").await?;
        let revisions = revision_headers(&mut tx, id, p, false).await?;
        let restrictions = restrictions(&mut tx, id).await?;
        let placements = sqlx::query_as(include_str!("sql/placements.sql"))
            .bind(id.0)
            .bind(user.0)
            .fetch_all(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let institutions = sqlx::query_scalar(include_str!("sql/restrict_institutions.sql"))
            .bind(user.0)
            .fetch_all(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let capabilities = MaterialCapabilities {
            manage: policy::can_manage_material(&actor.actor(), &material),
            edit: policy::can_create_revision(&actor.actor(), &material),
            submit: policy::can_submit_material(&actor.actor(), &material),
            collaborators: policy::can_manage_collaborators(&actor.actor(), &material),
            restrict_global: actor
                .actor()
                .user
                .has_permission(PlatformPermission::RestrictPublicMaterials),
            restrict_institutions: institutions,
        };
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(Workspace {
            material: result,
            revisions,
            restrictions,
            placements,
            capabilities,
        })
    }

    pub async fn authenticated_revision(
        &self,
        digest: &[u8],
        material_id: MaterialId,
        id: study_core::ids::MaterialRevisionId,
    ) -> Result<RevisionView> {
        let mut tx = session::snapshot(&self.pool).await?;
        let user = UserId(session::read(&mut tx, digest).await?.user_id);
        let material = load::material(&mut tx, material_id, false)
            .await?
            .ok_or(MaterialWriteError::NotFound(Entity::Material))?;
        let stored = load::revision(&mut tx, id, false)
            .await?
            .ok_or(MaterialWriteError::NotFound(Entity::Revision))?;
        let actor = access::load_snapshot(
            &mut tx,
            user,
            Some(material_id),
            owner_institution(material.owner()),
        )
        .await?;
        let moderate = policy::can_moderate_revision(&actor.actor(), &material, &stored.revision);
        let author_access = policy::can_read_nonpublic_material(Some(&actor.actor()), &material);
        let published_access =
            if stored.revision.state() == study_core::material::RevisionState::Published {
                sqlx::query_scalar::<_, bool>(include_str!("sql/published_access.sql"))
                    .bind(material_id.0)
                    .bind(Some(user.0))
                    .bind(None::<i64>)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(map_sqlx_error)?
            } else {
                false
            };
        if stored.revision.material_id() != material_id
            || (!author_access && !moderate && !published_access)
        {
            return Err(MaterialError::PermissionDenied.into());
        }
        let mut header = header(
            &mut tx,
            material_id,
            if moderate { "pending" } else { "author" },
        )
        .await?;
        if !author_access && !moderate {
            header.working_revision_id = None;
            header.working_revision_state = None;
        }
        let outdated = matches!(
            stored.revision.state(),
            study_core::material::RevisionState::Draft
                | study_core::material::RevisionState::PendingModeration
        ) && stored.revision.is_outdated_for(&material)?;
        let self_review = stored.revision.has_participated(user);
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(RevisionView {
            material: header,
            stored,
            outdated,
            self_review,
            can_moderate: moderate,
        })
    }
}
