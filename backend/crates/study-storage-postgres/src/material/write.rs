use super::{access, children, load, mapping};
use crate::error::map_sqlx_error;
use sqlx::{PgConnection, PgPool};
use std::time::SystemTime;
use study_core::error::RepositoryError;
use study_core::ids::{InstitutionId, MaterialId, MaterialRevisionId, UserId};
use study_core::material::{
    Material, MaterialError, MaterialOwner, MaterialRevision, MaterialVisibility, RevisionContent,
};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum Entity {
    User,
    Material,
    Revision,
}

#[derive(Debug, Error)]
pub enum MaterialWriteError {
    #[error("invalid development fixture")]
    InvalidFixture,
    #[error("draft edit conflict: expected {expected}, current {current}")]
    DraftConflict { expected: i64, current: i64 },
    #[error("{0:?} not found")]
    NotFound(Entity),
    #[error("revision conflict: expected {expected:?}, current {current:?}")]
    RevisionConflict {
        expected: Option<MaterialRevisionId>,
        current: Option<MaterialRevisionId>,
    },
    #[error("material command rejected")]
    Domain(#[source] MaterialError),
    #[error("invalid command timestamp")]
    Timestamp(#[from] study_core::timestamp::TimestampError),
    #[error("material storage failed")]
    Storage(#[from] RepositoryError),
}
impl From<MaterialError> for MaterialWriteError {
    fn from(error: MaterialError) -> Self {
        match error {
            MaterialError::OutdatedBase {
                base_revision_id,
                published_revision_id,
            } => Self::RevisionConflict {
                expected: base_revision_id,
                current: published_revision_id,
            },
            error => Self::Domain(error),
        }
    }
}
#[derive(Clone)]
pub struct CreateMaterial {
    pub owner: MaterialOwner,
    pub visibility: MaterialVisibility,
    pub content: RevisionContent,
    /// Explicit dev fixtures only; regular commands use None.
    pub fixture_key: Option<String>,
}
#[derive(Clone)]
pub struct PostgresMaterialRepository {
    pub(super) pool: PgPool,
}
impl PostgresMaterialRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn create(
        &self,
        user_id: UserId,
        command: CreateMaterial,
        at: SystemTime,
    ) -> Result<(Material, MaterialRevision), MaterialWriteError> {
        super::retry::run(|| {
            let command = command.clone();
            async move {
                let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
                let result = create_in(&mut tx, user_id, command, at).await?;
                tx.commit().await.map_err(map_sqlx_error)?;
                Ok(result)
            }
        })
        .await
    }

    /// Internal persistence read, not a public authorization boundary.
    pub async fn load_material(&self, id: MaterialId) -> Result<Option<Material>, RepositoryError> {
        let mut tx = crate::connection::read_snapshot(&self.pool).await?;
        let material = load::material(&mut tx, id, false).await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(material)
    }
    pub async fn load_revision(
        &self,
        id: MaterialRevisionId,
    ) -> Result<Option<super::StoredRevision>, RepositoryError> {
        let mut tx = crate::connection::read_snapshot(&self.pool).await?;
        let revision = load::revision(&mut tx, id, false).await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(revision)
    }
    pub async fn set_visibility(
        &self,
        user: UserId,
        id: MaterialId,
        visibility: MaterialVisibility,
    ) -> Result<(), MaterialWriteError> {
        super::retry::run(|| async move {
            let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
            let mut material = load::material(&mut tx, id, true)
                .await?
                .ok_or(MaterialWriteError::NotFound(Entity::Material))?;
            let actor =
                access::load(&mut tx, user, Some(id), owner_institution(material.owner())).await?;
            material.set_visibility(&actor.actor(), visibility)?;
            sqlx::query("UPDATE materials SET visibility=$1 WHERE id=$2")
                .bind(mapping::visibility_name(visibility))
                .bind(id.0)
                .execute(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
            tx.commit().await.map_err(map_sqlx_error)?;
            Ok(())
        })
        .await
    }
}
pub(super) fn owner_institution(owner: MaterialOwner) -> Option<InstitutionId> {
    match owner {
        MaterialOwner::Institution(id) => Some(id),
        MaterialOwner::User(_) => None,
    }
}
pub(super) async fn next_id(
    conn: &mut PgConnection,
    sql: &'static str,
) -> Result<i64, RepositoryError> {
    sqlx::query_scalar(sql)
        .fetch_one(conn)
        .await
        .map_err(map_sqlx_error)
}

pub(super) async fn create_in(
    conn: &mut PgConnection,
    user_id: UserId,
    command: CreateMaterial,
    at: SystemTime,
) -> Result<(Material, MaterialRevision), MaterialWriteError> {
    let at = study_core::timestamp::normalize(at)?;
    let actor = access::load(&mut *conn, user_id, None, owner_institution(command.owner)).await?;
    let id = MaterialId(
        next_id(
            &mut *conn,
            "SELECT nextval(pg_get_serial_sequence('materials','id'))",
        )
        .await?,
    );
    let material = Material::new(&actor.actor(), id, command.owner, command.visibility, at)?;
    let (owner_user_id, owner_institution_id) = match command.owner {
        MaterialOwner::User(id) => (Some(id.0), None),
        MaterialOwner::Institution(id) => (None, Some(id.0)),
    };
    sqlx::query(
        r#"
            INSERT INTO materials (
                id,
                fixture_key,
                owner_user_id,
                owner_institution_id,
                created_by_user_id,
                created_at,
                visibility
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(id.0)
    .bind(command.fixture_key.as_deref())
    .bind(owner_user_id)
    .bind(owner_institution_id)
    .bind(user_id.0)
    .bind(study_core::timestamp::to_utc(at)?)
    .bind(mapping::visibility_name(command.visibility))
    .execute(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    let revision_id = MaterialRevisionId(
        next_id(
            &mut *conn,
            "SELECT nextval(pg_get_serial_sequence('material_revisions','id'))",
        )
        .await?,
    );
    let revision =
        MaterialRevision::new_draft(&actor.actor(), &material, revision_id, command.content, at)?;
    children::insert_draft(&mut *conn, &revision).await?;
    Ok((material, revision))
}
