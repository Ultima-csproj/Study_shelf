use super::super::{
    CreateMaterial, MaterialWriteError, PostgresMaterialRepository, children, edit,
    write::{create_in, next_id},
};
use super::{
    ContentInput, ContributorInput, NewMaterialInput, Result, audit, reason, working_available,
};
use crate::error::map_sqlx_error;
use serde_json::json;
use sqlx::PgConnection;
use std::{collections::HashMap, time::SystemTime};
use study_core::ids::{ContributorId, MaterialId, MaterialRevisionId, UserId};
use study_core::material::{
    ContributorRole, MaterialOwner, MaterialRevision, MaterialVisibility, RevisionContent,
    RevisionCredit,
};

async fn content(
    tx: &mut PgConnection,
    user: UserId,
    mut input: ContentInput,
) -> Result<RevisionContent> {
    if input.title.chars().count() > 300
        || input.description.chars().count() > 20_000
        || input.credits.len() > 100
        || input.tags.len() > 100
    {
        return Err(study_core::account::AccountError::Validation(
            "Слишком большой текст или список участников",
        )
        .into());
    }
    if input.credits.is_empty() {
        let own: i64 = sqlx::query_scalar(include_str!("sql/own_contributor.sql"))
            .bind(user.0)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        input.credits.push(ContributorInput {
            id: own,
            role: ContributorRole::Author,
        });
    }
    let ids: Vec<i64> = input.credits.iter().map(|credit| credit.id).collect();
    let names: HashMap<i64, String> =
        sqlx::query_as::<_, (i64, String)>(include_str!("sql/contributor_names.sql"))
            .bind(&ids)
            .fetch_all(&mut *tx)
            .await
            .map_err(map_sqlx_error)?
            .into_iter()
            .collect();
    let credits = input
        .credits
        .into_iter()
        .map(|credit| {
            let name =
                names
                    .get(&credit.id)
                    .ok_or(study_core::account::AccountError::Validation(
                        "Указанный автор не найден",
                    ))?;
            Ok(RevisionCredit {
                contributor_id: ContributorId(credit.id),
                display_name: name.clone(),
                role: credit.role,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(RevisionContent {
        title: input.title.trim().to_owned(),
        description: input.description,
        material_type: input.material_type,
        metadata: input.metadata,
        tags: input.tags,
        credits,
    })
}

impl PostgresMaterialRepository {
    pub async fn create_authenticated(
        &self,
        digest: &[u8],
        input: NewMaterialInput,
        cause: &str,
    ) -> Result<(i64, i64)> {
        reason(cause)?;
        let (mut tx, user) = self.begin_session(digest).await?;
        let content = content(&mut tx, user, input.content).await?;
        let owner = input
            .institution
            .map(MaterialOwner::Institution)
            .unwrap_or(MaterialOwner::User(user));
        let (material, revision) = create_in(
            &mut tx,
            user,
            CreateMaterial {
                owner,
                visibility: MaterialVisibility::Private,
                content,
                fixture_key: None,
            },
            SystemTime::now(),
        )
        .await?;
        audit::record(
            &mut tx,
            user,
            material.id(),
            Some(revision.id()),
            "create_material",
            cause,
            json!({}),
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok((material.id().0, revision.id().0))
    }

    pub async fn fork_authenticated(
        &self,
        digest: &[u8],
        id: MaterialId,
        expected: i64,
        source: Option<MaterialRevisionId>,
        cause: &str,
    ) -> Result<i64> {
        reason(cause)?;
        let (mut tx, user) = self.begin_session(digest).await?;
        let (material, actor) = super::locked(&mut tx, user, id).await?;
        super::material_version(&mut tx, id, expected).await?;
        working_available(&mut tx, id).await?;
        if !study_core::material::policy::can_create_revision(&actor.actor(), &material) {
            return Err(study_core::material::MaterialError::PermissionDenied.into());
        }
        let source =
            source
                .or(material.published_revision_id())
                .ok_or(MaterialWriteError::NotFound(
                    super::super::write::Entity::Revision,
                ))?;
        let old = super::super::load::revision(&mut tx, source, false)
            .await?
            .ok_or(MaterialWriteError::NotFound(
                super::super::write::Entity::Revision,
            ))?;
        if old.revision.material_id() != id {
            return Err(MaterialWriteError::ScopeMismatch);
        }
        let next =
            MaterialRevisionId(next_id(&mut tx, include_str!("sql/next_revision.sql")).await?);
        let revision = MaterialRevision::new_draft(
            &actor.actor(),
            &material,
            next,
            old.revision.content().clone(),
            SystemTime::now(),
        )?;
        children::insert_draft(&mut tx, &revision).await?;
        audit::record(
            &mut tx,
            user,
            id,
            Some(next),
            "create_draft",
            cause,
            json!({
                "copiedFrom": source.0,
                "baseRevisionId": revision.base_revision_id().map(|id| id.0),
            }),
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(next.0)
    }

    pub async fn save_authenticated(
        &self,
        digest: &[u8],
        material: MaterialId,
        revision: MaterialRevisionId,
        expected: i64,
        input: ContentInput,
        cause: &str,
    ) -> Result<i64> {
        reason(cause)?;
        let (mut tx, user) = self.begin_session(digest).await?;
        let content = content(&mut tx, user, input).await?;
        let stored = edit::replace_in(
            &mut tx,
            user,
            material,
            revision,
            expected,
            content,
            SystemTime::now(),
        )
        .await?;
        audit::record(
            &mut tx,
            user,
            material,
            Some(revision),
            "save_draft",
            cause,
            json!({"version":stored.edit_version}),
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(stored.edit_version)
    }
}
