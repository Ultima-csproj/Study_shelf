use super::Result;
use crate::error::map_sqlx_error;
use serde_json::Value;
use sqlx::PgConnection;
use study_core::ids::{MaterialId, MaterialRevisionId, UserId};

pub(super) async fn record(
    tx: &mut PgConnection,
    user: UserId,
    material: MaterialId,
    revision: Option<MaterialRevisionId>,
    action: &str,
    reason: &str,
    details: Value,
) -> Result<()> {
    sqlx::query(include_str!("sql/event.sql"))
        .bind(material.0)
        .bind(revision.map(|id| id.0))
        .bind(user.0)
        .bind(action)
        .bind(reason.trim())
        .bind(details)
        .execute(tx)
        .await
        .map_err(map_sqlx_error)?;
    Ok(())
}
