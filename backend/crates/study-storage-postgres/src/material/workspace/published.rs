use super::super::{MaterialWriteError, PostgresMaterialRepository, load, write::Entity};
use super::{MaterialContext, PublishedHistory, PublishedView, Result, RevisionView, read};
use crate::{account::session, administration::PageRequest, error::map_sqlx_error};
use sqlx::{PgConnection, Postgres, Transaction};
use study_core::ids::{MaterialId, MaterialRevisionId};
use study_core::material::RevisionState;

pub(super) async fn context(
    tx: &mut PgConnection,
    id: MaterialId,
    user: Option<i64>,
    context: &MaterialContext,
) -> Result<Option<i64>> {
    if usize::from(context.discipline_id.is_some())
        + usize::from(context.collection_id.is_some())
        + usize::from(context.restriction_id.is_some())
        > 1
    {
        return Err(MaterialWriteError::ScopeMismatch);
    }
    if let Some(restriction) = context.restriction_id {
        return sqlx::query_scalar::<_, Option<i64>>(include_str!("sql/context_restriction.sql"))
            .bind(restriction)
            .bind(id.0)
            .fetch_optional(tx)
            .await
            .map_err(map_sqlx_error)?
            .ok_or(MaterialWriteError::ScopeMismatch);
    }
    if let Some(discipline) = context.discipline_id {
        return sqlx::query_scalar(include_str!("sql/context_discipline.sql"))
            .bind(discipline)
            .bind(id.0)
            .fetch_optional(tx)
            .await
            .map_err(map_sqlx_error)?
            .map(Some)
            .ok_or(MaterialWriteError::ScopeMismatch);
    }
    if let Some(collection) = context.collection_id {
        return sqlx::query_scalar::<_, Option<i64>>(include_str!("sql/context_collection.sql"))
            .bind(collection)
            .bind(id.0)
            .bind(user)
            .fetch_optional(tx)
            .await
            .map_err(map_sqlx_error)?
            .ok_or(MaterialWriteError::ScopeMismatch);
    }
    Ok(None)
}

impl PostgresMaterialRepository {
    async fn publication_snapshot(
        &self,
        digest: Option<&[u8]>,
        id: MaterialId,
        context_input: &MaterialContext,
    ) -> Result<(
        Transaction<'_, Postgres>,
        study_core::material::Material,
        Option<i64>,
    )> {
        let mut tx = crate::connection::read_snapshot(&self.pool).await?;
        let user = if let Some(digest) = digest {
            Some(session::read(&mut tx, digest).await?.user_id)
        } else {
            None
        };
        let institution = context(&mut tx, id, user, context_input).await?;
        let allowed: bool = sqlx::query_scalar(include_str!("sql/published_access.sql"))
            .bind(id.0)
            .bind(user)
            .bind(institution)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        if !allowed {
            return Err(MaterialWriteError::NotFound(Entity::Material));
        }
        let material = load::material(&mut tx, id, false)
            .await?
            .ok_or(MaterialWriteError::NotFound(Entity::Material))?;
        Ok((tx, material, institution))
    }

    pub async fn published_detail(
        &self,
        digest: Option<&[u8]>,
        id: MaterialId,
        context: &MaterialContext,
    ) -> Result<PublishedView> {
        let (mut tx, material, institution) =
            self.publication_snapshot(digest, id, context).await?;
        let row = sqlx::query_as::<_, crate::catalog::row::SummaryRow>(include_str!(
            "sql/published_summary.sql"
        ))
        .bind(id.0)
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?
        .ok_or(MaterialWriteError::NotFound(Entity::Revision))?;
        let mut items = vec![row.into_summary()?];
        crate::catalog::children::populate(&mut tx, &mut items).await?;
        let mut restrictions = read::restrictions(&mut tx, id).await?;
        restrictions
            .retain(|row| row.institution_id.is_none() || row.institution_id == institution);
        let lifecycle = match material.lifecycle() {
            study_core::material::MaterialLifecycle::Active => "active",
            study_core::material::MaterialLifecycle::Archived => "archived",
        }
        .to_owned();
        let publication = match material.publication() {
            study_core::material::PublicationState::Published => "published",
            study_core::material::PublicationState::Unpublished => "unpublished",
        }
        .to_owned();
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(PublishedView {
            summary: items.remove(0),
            lifecycle,
            publication,
            restrictions,
        })
    }

    pub async fn published_history(
        &self,
        digest: Option<&[u8]>,
        id: MaterialId,
        context: &MaterialContext,
        p: &PageRequest,
    ) -> Result<PublishedHistory> {
        let (mut tx, material, institution) =
            self.publication_snapshot(digest, id, context).await?;
        let _ = institution;
        let revisions = read::revision_headers(&mut tx, id, p, true).await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(PublishedHistory {
            material_id: id.0,
            lifecycle: match material.lifecycle() {
                study_core::material::MaterialLifecycle::Active => "active",
                study_core::material::MaterialLifecycle::Archived => "archived",
            }
            .to_owned(),
            current_revision_id: material.published_revision_id().map(|id| id.0),
            revisions,
        })
    }

    pub async fn published_revision(
        &self,
        digest: Option<&[u8]>,
        id: MaterialId,
        revision: MaterialRevisionId,
        context: &MaterialContext,
    ) -> Result<RevisionView> {
        let (mut tx, _material, _institution) =
            self.publication_snapshot(digest, id, context).await?;
        let stored = load::revision(&mut tx, revision, false)
            .await?
            .ok_or(MaterialWriteError::NotFound(Entity::Revision))?;
        if stored.revision.material_id() != id
            || stored.revision.state() != RevisionState::Published
        {
            return Err(MaterialWriteError::NotFound(Entity::Revision));
        }
        let header = read::header(&mut tx, id, "public").await?;
        let number = read::revision_number(&mut tx, id, revision).await?;
        // Only public history is returned. Header must not disclose an unrelated working draft.
        let mut header = header;
        header.working_revision_id = None;
        header.working_revision_state = None;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(RevisionView {
            material: header,
            stored,
            outdated: false,
            self_review: false,
            can_moderate: false,
            number,
        })
    }
}
