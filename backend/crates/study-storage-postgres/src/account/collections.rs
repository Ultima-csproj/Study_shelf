mod edit;
use super::{PostgresAccountRepository, session};
use crate::error::map_sqlx_error;
pub use edit::{CollectionEdit, CollectionInfo};
use sqlx::{FromRow, PgConnection};
use study_core::account::{AccountError, AccountResult};
use study_core::catalog::MaterialSummary;

#[derive(FromRow)]
pub struct OwnedCollection {
    pub id: i64,
    pub title: String,
    pub short_description: String,
    pub description: String,
    pub purpose: String,
    pub target_audience: String,
    pub visibility: String,
    pub edit_version: i64,
}
#[derive(FromRow)]
pub struct OwnedSection {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub position: i32,
    pub entry_count: i64,
}
#[derive(FromRow)]
pub struct OwnedEntry {
    pub id: i64,
    pub material_id: i64,
    pub position: i32,
}
pub struct OwnedDetail {
    pub collection: OwnedCollection,
    pub sections: Vec<OwnedSection>,
}
pub struct OwnedEntries {
    pub entries: Vec<OwnedEntry>,
    pub materials: Vec<MaterialSummary>,
    pub total: i64,
}
pub struct OwnedPage {
    pub items: Vec<OwnedCollection>,
    pub total: i64,
}

pub(super) async fn owned(
    conn: &mut PgConnection,
    user: i64,
    id: i64,
) -> AccountResult<OwnedCollection> {
    sqlx::query_as::<_, OwnedCollection>(
        r#"
        SELECT
          id,
          title,
          short_description,
          description,
          purpose,
          target_audience,
          visibility,
          edit_version
        FROM
          collections
        WHERE
          id = $1
          AND owner_user_id = $2
        FOR UPDATE
    "#,
    )
    .bind(id)
    .bind(user)
    .fetch_optional(conn)
    .await
    .map_err(map_sqlx_error)?
    .ok_or(AccountError::NotFound)
}
async fn owned_read(conn: &mut PgConnection, user: i64, id: i64) -> AccountResult<OwnedCollection> {
    sqlx::query_as::<_, OwnedCollection>(
        r#"
        SELECT
          id,
          title,
          short_description,
          description,
          purpose,
          target_audience,
          visibility,
          edit_version
        FROM
          collections
        WHERE
          id = $1
          AND owner_user_id = $2
    "#,
    )
    .bind(id)
    .bind(user)
    .fetch_optional(conn)
    .await
    .map_err(map_sqlx_error)?
    .ok_or(AccountError::NotFound)
}
impl PostgresAccountRepository {
    pub async fn owned_collections(
        &self,
        digest: &[u8],
        number: u32,
        size: u32,
    ) -> AccountResult<OwnedPage> {
        if number == 0 || !(1..=100).contains(&size) {
            return Err(AccountError::Validation("Некорректная страница"));
        }
        let mut tx = session::snapshot(&self.pool).await?;
        let user = session::read(&mut tx, digest).await?.user_id;
        let total = sqlx::query_scalar(
            r#"
            SELECT
              count(*)
            FROM
              collections
            WHERE
              owner_user_id = $1
        "#,
        )
        .bind(user)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let items = sqlx::query_as::<_, OwnedCollection>(
            r#"
            SELECT
              id,
              title,
              short_description,
              description,
              purpose,
              target_audience,
              visibility,
              edit_version
            FROM
              collections
            WHERE
              owner_user_id = $1
            ORDER BY
              updated_at DESC,
              id
            LIMIT
              $2
            OFFSET
              $3
        "#,
        )
        .bind(user)
        .bind(i64::from(size))
        .bind(i64::from(number - 1) * i64::from(size))
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(OwnedPage { items, total })
    }
    pub async fn create_collection(
        &self,
        digest: &[u8],
        info: &CollectionInfo,
        slug: &str,
    ) -> AccountResult<i64> {
        info.validate()?;
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let user = session::require(&mut tx, digest).await?.user_id;
        let id = sqlx::query_scalar(
            r#"
                INSERT INTO
                  collections (
                    slug,
                    title,
                    short_description,
                    description,
                    purpose,
                    target_audience,
                    owner_user_id,
                    created_by_user_id,
                    visibility,
                    created_at,
                    updated_at
                  )
                VALUES
                  (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6,
                    $7,
                    $7,
                    'private',
                    clock_timestamp(),
                    clock_timestamp()
                  )
                RETURNING
                  id
            "#,
        )
        .bind(slug)
        .bind(info.title.trim())
        .bind(&info.short_description)
        .bind(&info.description)
        .bind(&info.purpose)
        .bind(&info.target_audience)
        .bind(user)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        sqlx::query(
            r#"
                INSERT INTO
                  collection_participants (
                    collection_id,
                    contributor_id,
                    display_name,
                    role,
                    position
                  )
                SELECT
                  $1,
                  p.primary_contributor_id,
                  c.display_name,
                  'author',
                  1
                FROM
                  user_profiles p
                  JOIN contributors c ON c.id = p.primary_contributor_id
                WHERE
                  p.user_id = $2
            "#,
        )
        .bind(id)
        .bind(user)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(id)
    }
    pub async fn owned_detail(&self, digest: &[u8], id: i64) -> AccountResult<OwnedDetail> {
        let mut tx = session::snapshot(&self.pool).await?;
        let user = session::read(&mut tx, digest).await?.user_id;
        let collection = owned_read(&mut tx, user, id).await?;
        let sections = sqlx::query_as::<_, OwnedSection>(
            r#"
                SELECT
                  s.id,
                  s.title,
                  s.description,
                  s.position,
                  (
                    SELECT
                      count(*)
                    FROM
                      collection_entries
                    WHERE
                      section_id = s.id
                  ) AS entry_count
                FROM
                  collection_sections s
                WHERE
                  collection_id = $1
                ORDER BY
                  position,
                  id
                LIMIT
                  100
            "#,
        )
        .bind(id)
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(OwnedDetail {
            collection,
            sections,
        })
    }
    pub async fn owned_entries(
        &self,
        digest: &[u8],
        id: i64,
        section: i64,
        number: u32,
        size: u32,
    ) -> AccountResult<OwnedEntries> {
        if number == 0 || !(1..=100).contains(&size) {
            return Err(AccountError::Validation("Некорректная страница"));
        }
        let mut tx = session::snapshot(&self.pool).await?;
        let user = session::read(&mut tx, digest).await?.user_id;
        owned_read(&mut tx, user, id).await?;
        let exists: bool = sqlx::query_scalar(
            r#"
                SELECT
                  EXISTS (
                    SELECT
                      1
                    FROM
                      collection_sections
                    WHERE
                      id = $1
                      AND collection_id = $2
                  )
            "#,
        )
        .bind(section)
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if !exists {
            return Err(AccountError::NotFound);
        }
        let total = sqlx::query_scalar(
            r#"
                SELECT
                  count(*)
                FROM
                  collection_entries
                WHERE
                  section_id = $1
            "#,
        )
        .bind(section)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let entries = sqlx::query_as::<_, OwnedEntry>(
            r#"
            SELECT
              id,
              material_id,
              position
            FROM
              collection_entries
            WHERE
              section_id = $1
            ORDER BY
              position,
              id
            LIMIT
              $2
            OFFSET
              $3
        "#,
        )
        .bind(section)
        .bind(i64::from(size))
        .bind(i64::from(number - 1) * i64::from(size))
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let ids: Vec<i64> = entries.iter().map(|e| e.material_id).collect();
        let materials = super::library::materials(&mut tx, user, &ids).await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(OwnedEntries {
            entries,
            materials,
            total,
        })
    }
}
