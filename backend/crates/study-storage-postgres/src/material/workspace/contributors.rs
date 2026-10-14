use super::{PostgresMaterialRepository, Result};
use crate::{account::session, error::map_sqlx_error};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ContributorChoice {
    pub id: i64,
    pub display_name: String,
}

impl PostgresMaterialRepository {
    /// Only public credit names and the viewer's own contributor are searchable here.
    pub async fn contributor_choices(
        &self,
        digest: &[u8],
        query: &str,
    ) -> Result<Vec<ContributorChoice>> {
        let query = query.trim();
        if query.chars().count() > 100 {
            return Err(study_core::account::AccountError::Validation(
                "Слишком длинное имя автора",
            )
            .into());
        }
        let mut tx = session::snapshot(&self.pool).await?;
        let user = session::read(&mut tx, digest).await?.user_id;
        let pattern = query
            .replace('\\', "\\\\")
            .replace('%', "\\%")
            .replace('_', "\\_");
        let result = sqlx::query_as(include_str!("sql/contributor_choices.sql"))
            .bind(user)
            .bind(query)
            .bind(pattern)
            .fetch_all(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(result)
    }
}
