use super::{PostgresAccountRepository, owned, session};
use crate::error::map_sqlx_error;
use study_core::account::{AccountError, AccountResult};

pub struct CollectionInfo {
    pub title: String,
    pub short_description: String,
    pub description: String,
    pub purpose: String,
    pub target_audience: String,
}
impl CollectionInfo {
    pub fn validate(&self) -> AccountResult<()> {
        if self.title.trim().is_empty()
            || self.title.chars().count() > 160
            || self.short_description.chars().count() > 500
            || self.description.chars().count() > 10000
            || self.purpose.chars().count() > 2000
            || self.target_audience.chars().count() > 2000
        {
            return Err(AccountError::Validation("Некорректные сведения подборки"));
        }
        Ok(())
    }
}
pub enum CollectionEdit {
    Info(CollectionInfo),
    AddSection {
        title: String,
        description: String,
    },
    UpdateSection {
        id: i64,
        title: String,
        description: String,
    },
    DeleteSection {
        id: i64,
    },
    AddEntry {
        section: i64,
        material: i64,
    },
    DeleteEntry {
        section: i64,
        id: i64,
    },
    ReorderSections {
        ids: Vec<i64>,
    },
    ReorderEntries {
        section: i64,
        ids: Vec<i64>,
    },
}
impl PostgresAccountRepository {
    pub async fn edit_collection(
        &self,
        digest: &[u8],
        id: i64,
        expected: i64,
        edit: CollectionEdit,
    ) -> AccountResult<i64> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let user = session::require(&mut tx, digest).await?.user_id;
        let collection = owned(&mut tx, user, id).await?;
        if collection.visibility != "private" {
            return Err(AccountError::Forbidden);
        }
        if collection.edit_version != expected {
            return Err(AccountError::VersionConflict);
        }
        match edit {
            CollectionEdit::Info(info) => {
                info.validate()?;
                sqlx::query(
                    r#"
                    UPDATE collections
                    SET
                      title = $1,
                      short_description = $2,
                      description = $3,
                      purpose = $4,
                      target_audience = $5
                    WHERE
                      id = $6
                "#,
                )
                .bind(info.title.trim())
                .bind(info.short_description)
                .bind(info.description)
                .bind(info.purpose)
                .bind(info.target_audience)
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
            }
            CollectionEdit::AddSection { title, description } => {
                validate_section(&title, &description)?;
                let count: i64 = sqlx::query_scalar(
                    r#"
                        SELECT
                          count(*)
                        FROM
                          collection_sections
                        WHERE
                          collection_id = $1
                    "#,
                )
                .bind(id)
                .fetch_one(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
                if count >= 100 {
                    return Err(AccountError::Validation("Не более 100 разделов"));
                }
                sqlx::query(
                    r#"
                        INSERT INTO
                          collection_sections (collection_id, title, description, position)
                        SELECT
                          $1,
                          $2,
                          $3,
                          COALESCE(max(position), 0) + 1
                        FROM
                          collection_sections
                        WHERE
                          collection_id = $1
                    "#,
                )
                .bind(id)
                .bind(title.trim())
                .bind(description)
                .execute(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
            }
            CollectionEdit::UpdateSection {
                id: section,
                title,
                description,
            } => {
                validate_section(&title, &description)?;
                let n = sqlx::query(
                    r#"
                    UPDATE collection_sections
                    SET
                      title = $1,
                      description = $2
                    WHERE
                      id = $3
                      AND collection_id = $4
                "#,
                )
                .bind(title.trim())
                .bind(description)
                .bind(section)
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(map_sqlx_error)?
                .rows_affected();
                if n != 1 {
                    return Err(AccountError::NotFound);
                }
            }
            CollectionEdit::DeleteSection { id: section } => {
                sqlx::query(
                    r#"
                        DELETE FROM collection_entries
                        WHERE
                          section_id = $1
                          AND collection_id = $2
                    "#,
                )
                .bind(section)
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
                let n = sqlx::query(
                    r#"
                        DELETE FROM collection_sections
                        WHERE
                          id = $1
                          AND collection_id = $2
                    "#,
                )
                .bind(section)
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(map_sqlx_error)?
                .rows_affected();
                if n != 1 {
                    return Err(AccountError::NotFound);
                }
            }
            CollectionEdit::AddEntry { section, material } => {
                if super::super::library::materials(&mut tx, user, &[material])
                    .await?
                    .is_empty()
                {
                    return Err(AccountError::NotFound);
                }
                let valid: bool = sqlx::query_scalar(
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
                if !valid {
                    return Err(AccountError::NotFound);
                }
                let count: i64 = sqlx::query_scalar(
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
                if count >= 1000 {
                    return Err(AccountError::Validation(
                        "Не более 1000 материалов в разделе",
                    ));
                }
                sqlx::query(
                    r#"
                        INSERT INTO
                          collection_entries (collection_id, section_id, material_id, position)
                        SELECT
                          $1,
                          $2,
                          $3,
                          COALESCE(max(position), 0) + 1
                        FROM
                          collection_entries
                        WHERE
                          section_id = $2
                        ON CONFLICT (section_id, material_id) DO NOTHING
                    "#,
                )
                .bind(id)
                .bind(section)
                .bind(material)
                .execute(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
            }
            CollectionEdit::DeleteEntry { section, id: entry } => {
                let n = sqlx::query(
                    r#"
                    DELETE FROM collection_entries
                    WHERE
                      id = $1
                      AND section_id = $2
                      AND collection_id = $3
                "#,
                )
                .bind(entry)
                .bind(section)
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(map_sqlx_error)?
                .rows_affected();
                if n != 1 {
                    return Err(AccountError::NotFound);
                }
            }
            CollectionEdit::ReorderSections { ids } => {
                let current: Vec<i64> = sqlx::query_scalar(
                    r#"
                        SELECT
                          id
                        FROM
                          collection_sections
                        WHERE
                          collection_id = $1
                        ORDER BY
                          id
                    "#,
                )
                .bind(id)
                .fetch_all(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
                validate_order(&current, &ids)?;
                sqlx::query(
                    r#"
            UPDATE collection_sections s
            SET position=x.position::integer
            FROM unnest($1::bigint[]) WITH ORDINALITY x(id,position)
            WHERE s.id=x.id
                AND s.collection_id=$2
        "#,
                )
                .bind(ids)
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
            }
            CollectionEdit::ReorderEntries { section, ids } => {
                let valid: bool = sqlx::query_scalar(
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
                if !valid {
                    return Err(AccountError::NotFound);
                }
                let current: Vec<i64> = sqlx::query_scalar(
                    r#"
                    SELECT
                      id
                    FROM
                      collection_entries
                    WHERE
                      section_id = $1
                      AND collection_id = $2
                    ORDER BY
                      id
                "#,
                )
                .bind(section)
                .bind(id)
                .fetch_all(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
                validate_order(&current, &ids)?;
                sqlx::query(
                    r#"
            UPDATE collection_entries e
            SET position=x.position::integer
            FROM unnest($1::bigint[]) WITH ORDINALITY x(id,position)
            WHERE e.id=x.id
                AND e.section_id=$2
                AND e.collection_id=$3
        "#,
                )
                .bind(ids)
                .bind(section)
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
            }
        }
        let version = sqlx::query_scalar(
            r#"
            UPDATE collections
            SET
              edit_version = edit_version + 1,
              updated_at = clock_timestamp()
            WHERE
              id = $1
            RETURNING
              edit_version
        "#,
        )
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(version)
    }
}
fn validate_section(title: &str, description: &str) -> AccountResult<()> {
    if title.trim().is_empty() || title.chars().count() > 160 || description.chars().count() > 2000
    {
        return Err(AccountError::Validation("Некорректный раздел"));
    }
    Ok(())
}
fn validate_order(current: &[i64], requested: &[i64]) -> AccountResult<()> {
    if requested.len() > 1000 || current.len() != requested.len() {
        return Err(AccountError::Validation(
            "Передайте полный порядок элементов",
        ));
    }
    let mut sorted = requested.to_vec();
    sorted.sort_unstable();
    if sorted != current {
        return Err(AccountError::Validation(
            "Порядок содержит чужие или повторяющиеся элементы",
        ));
    }
    Ok(())
}
