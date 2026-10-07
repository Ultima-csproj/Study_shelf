use super::{PostgresAccountRepository, session};
use crate::error::map_sqlx_error;
use futures_util::TryStreamExt;
use sqlx::{FromRow, PgConnection};
use std::sync::LazyLock;
use study_core::account::{AccountError, AccountResult};
use study_core::catalog::MaterialSummary;

pub(crate) static MATERIAL_ACCESS: &str = include_str!("material_access.sql");
static MATERIALS: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"
            SELECT
              m.id,
              r.id AS published_revision_id,
              r.title,
              r.description,
              r.material_type,
              r.source_year,
              r.preview_url,
              r.duration_seconds,
              r.file_size_bytes,
              m.first_published_at,
              r.published_at
            FROM
              materials m
              JOIN material_revisions r ON r.id = m.published_revision_id
              AND r.material_id = m.id
            WHERE
              r.state = 'published'
              AND m.id = ANY ($2)
              AND ({MATERIAL_ACCESS})
            ORDER BY
              m.id
        "#
    )
});
pub(crate) async fn materials(
    conn: &mut PgConnection,
    user: i64,
    ids: &[i64],
) -> AccountResult<Vec<MaterialSummary>> {
    let mut items = Vec::with_capacity(ids.len());
    if ids.is_empty() {
        return Ok(items);
    }
    {
        let mut rows = sqlx::query_as::<_, crate::catalog::row::SummaryRow>(&**MATERIALS)
            .bind(user)
            .bind(ids)
            .fetch(&mut *conn);
        while let Some(row) = rows.try_next().await.map_err(map_sqlx_error)? {
            items.push(row.into_summary()?);
        }
    }
    crate::catalog::children::populate(conn, &mut items).await?;
    Ok(items)
}
pub enum SavedKind {
    Material,
    Collection,
}
pub struct SavedMaterials {
    pub ids: Vec<i64>,
    pub materials: Vec<MaterialSummary>,
    pub total: i64,
}
#[derive(FromRow)]
pub struct SavedCollection {
    pub id: i64,
    pub title: Option<String>,
    pub short_description: Option<String>,
    pub owned: bool,
}
pub struct SavedCollections {
    pub items: Vec<SavedCollection>,
    pub total: i64,
}
fn page(page: u32, size: u32) -> AccountResult<i64> {
    if page == 0 || !(1..=100).contains(&size) {
        return Err(AccountError::Validation("Некорректная страница"));
    }
    Ok(i64::from(page - 1) * i64::from(size))
}
impl PostgresAccountRepository {
    pub async fn contributed_collections(
        &self,
        digest: &[u8],
        number: u32,
        size: u32,
    ) -> AccountResult<SavedCollections> {
        let offset = page(number, size)?;
        let mut tx = session::snapshot(&self.pool).await?;
        let user = session::read(&mut tx, digest).await?.user_id;
        let total = sqlx::query_scalar(
            r#"
                SELECT
                  count(*)
                FROM
                  collections c
                WHERE
                  (
                    c.visibility = 'public'
                    OR c.owner_user_id = $1
                  )
                  AND EXISTS (
                    SELECT
                      1
                    FROM
                      collection_participants p
                      JOIN contributors a ON a.id = p.contributor_id
                    WHERE
                      p.collection_id = c.id
                      AND a.user_id = $1
                  )
            "#,
        )
        .bind(user)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let items = sqlx::query_as::<_, SavedCollection>(
            r#"
                SELECT
                  c.id,
                  c.title,
                  c.short_description,
                  COALESCE(c.owner_user_id = $1, false) AS owned
                FROM
                  collections c
                WHERE
                  (
                    c.visibility = 'public'
                    OR c.owner_user_id = $1
                  )
                  AND EXISTS (
                    SELECT
                      1
                    FROM
                      collection_participants p
                      JOIN contributors a ON a.id = p.contributor_id
                    WHERE
                      p.collection_id = c.id
                      AND a.user_id = $1
                  )
                ORDER BY
                  c.updated_at DESC,
                  c.id
                LIMIT
                  $2
                OFFSET
                  $3
            "#,
        )
        .bind(user)
        .bind(i64::from(size))
        .bind(offset)
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(SavedCollections { items, total })
    }
    pub async fn saved_materials(
        &self,
        digest: &[u8],
        number: u32,
        size: u32,
    ) -> AccountResult<SavedMaterials> {
        let offset = page(number, size)?;
        let mut tx = session::snapshot(&self.pool).await?;
        let user = session::read(&mut tx, digest).await?.user_id;
        let total: i64 = sqlx::query_scalar(
            r#"
                SELECT
                  count(*)
                FROM
                  saved_materials
                WHERE
                  user_id = $1
            "#,
        )
        .bind(user)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let ids: Vec<i64> = sqlx::query_scalar(
            r#"
            SELECT
              material_id
            FROM
              saved_materials
            WHERE
              user_id = $1
            ORDER BY
              saved_at DESC,
              material_id
            LIMIT
              $2
            OFFSET
              $3
        "#,
        )
        .bind(user)
        .bind(i64::from(size))
        .bind(offset)
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let materials = materials(&mut tx, user, &ids).await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(SavedMaterials {
            ids,
            materials,
            total,
        })
    }
    pub async fn saved_collections(
        &self,
        digest: &[u8],
        number: u32,
        size: u32,
    ) -> AccountResult<SavedCollections> {
        let offset = page(number, size)?;
        let mut tx = session::snapshot(&self.pool).await?;
        let user = session::read(&mut tx, digest).await?.user_id;
        let total = sqlx::query_scalar(
            r#"
            SELECT
              count(*)
            FROM
              saved_collections
            WHERE
              user_id = $1
        "#,
        )
        .bind(user)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let items = sqlx::query_as::<_, SavedCollection>(
            r#"
                SELECT
                  s.collection_id AS id,
                  c.title,
                  c.short_description,
                  COALESCE(c.owner_user_id = $1, false) AS owned
                FROM
                  saved_collections s
                  LEFT JOIN collections c ON c.id = s.collection_id
                  AND (
                    c.visibility = 'public'
                    OR c.owner_user_id = $1
                  )
                WHERE
                  s.user_id = $1
                ORDER BY
                  s.saved_at DESC,
                  s.collection_id
                LIMIT
                  $2
                OFFSET
                  $3
            "#,
        )
        .bind(user)
        .bind(i64::from(size))
        .bind(offset)
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(SavedCollections { items, total })
    }
    pub async fn saved_flags(
        &self,
        digest: &[u8],
        material_ids: &[i64],
        collection_ids: &[i64],
    ) -> AccountResult<(Vec<i64>, Vec<i64>)> {
        if material_ids.len() > 100 || collection_ids.len() > 100 {
            return Err(AccountError::Validation("Слишком много идентификаторов"));
        }
        let mut tx = session::snapshot(&self.pool).await?;
        let user = session::read(&mut tx, digest).await?.user_id;
        let materials = sqlx::query_scalar(
            r#"
                SELECT
                  material_id
                FROM
                  saved_materials
                WHERE
                  user_id = $1
                  AND material_id = ANY ($2)
            "#,
        )
        .bind(user)
        .bind(material_ids)
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let collections = sqlx::query_scalar(
            r#"
            SELECT
              collection_id
            FROM
              saved_collections
            WHERE
              user_id = $1
              AND collection_id = ANY ($2)
        "#,
        )
        .bind(user)
        .bind(collection_ids)
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok((materials, collections))
    }
    pub async fn save(
        &self,
        digest: &[u8],
        kind: SavedKind,
        id: i64,
        save: bool,
    ) -> AccountResult<()> {
        if id <= 0 {
            return Err(AccountError::Validation("Некорректный идентификатор"));
        }
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let user = session::require(&mut tx, digest).await?.user_id;
        if save {
            let allowed = match kind {
                SavedKind::Material => !materials(&mut tx, user, &[id]).await?.is_empty(),
                SavedKind::Collection => sqlx::query_scalar::<_, bool>(
                    r#"
                    SELECT
                      EXISTS (
                        SELECT
                          1
                        FROM
                          collections
                        WHERE
                          id = $2
                          AND (
                            visibility = 'public'
                            OR owner_user_id = $1
                          )
                      )
                "#,
                )
                .bind(user)
                .bind(id)
                .fetch_one(&mut *tx)
                .await
                .map_err(map_sqlx_error)?,
            };
            if !allowed {
                return Err(AccountError::NotFound);
            }
        }
        let sql = match (kind, save) {
            (SavedKind::Material, true) => {
                r#"
                    INSERT INTO
                      saved_materials (user_id, material_id)
                    VALUES
                      ($1, $2)
                    ON CONFLICT DO NOTHING
                "#
            }
            (SavedKind::Material, false) => {
                r#"
                    DELETE FROM saved_materials
                    WHERE
                      user_id = $1
                      AND material_id = $2
                "#
            }
            (SavedKind::Collection, true) => {
                r#"
                    INSERT INTO
                      saved_collections (user_id, collection_id)
                    VALUES
                      ($1, $2)
                    ON CONFLICT DO NOTHING
                "#
            }
            (SavedKind::Collection, false) => {
                r#"
                    DELETE FROM saved_collections
                    WHERE
                      user_id = $1
                      AND collection_id = $2
                "#
            }
        };
        sqlx::query(sql)
            .bind(user)
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
    pub async fn material(&self, digest: &[u8], id: i64) -> AccountResult<Option<MaterialSummary>> {
        let mut tx = session::snapshot(&self.pool).await?;
        let user = session::read(&mut tx, digest).await?.user_id;
        let mut items = materials(&mut tx, user, &[id]).await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(items.pop())
    }
}
