use super::super::{MaterialWriteError, PostgresMaterialRepository};
use super::{MaterialStateChange, Result, audit, locked, material_version, reason};
use crate::error::map_sqlx_error;
use serde_json::json;
use study_core::ids::MaterialId;
use study_core::material::{
    MaterialError, MaterialLifecycle, MaterialVisibility, PublicationState,
};

impl PostgresMaterialRepository {
    pub async fn change_state_authenticated(
        &self,
        digest: &[u8],
        id: MaterialId,
        expected: i64,
        change: MaterialStateChange,
        cause: &str,
    ) -> Result<()> {
        reason(cause)?;
        let (mut tx, user) = self.begin_session(digest).await?;
        let (mut material, actor) = locked(&mut tx, user, id).await?;
        material_version(&mut tx, id, expected).await?;
        if !study_core::material::policy::can_manage_material(&actor.actor(), &material) {
            return Err(MaterialError::PermissionDenied.into());
        }
        let pending: bool = sqlx::query_scalar(include_str!("sql/pending_exists.sql"))
            .bind(id.0)
            .fetch_one(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        if pending {
            return Err(MaterialWriteError::PendingReviewActive);
        }
        let action = match change {
            MaterialStateChange::Lifecycle(value) => {
                material.set_lifecycle(&actor.actor(), value)?;
                if value == MaterialLifecycle::Archived {
                    "archive_material"
                } else {
                    "restore_material"
                }
            }
            MaterialStateChange::Publication(value) => {
                material.set_publication(&actor.actor(), value)?;
                if value == PublicationState::Published {
                    "republish_material"
                } else {
                    "unpublish_material"
                }
            }
            MaterialStateChange::Private => {
                material.set_visibility(&actor.actor(), MaterialVisibility::Private)?;
                "make_material_private"
            }
        };
        let lifecycle = match material.lifecycle() {
            MaterialLifecycle::Active => "active",
            MaterialLifecycle::Archived => "archived",
        };
        let publication = match material.publication() {
            PublicationState::Published => "published",
            PublicationState::Unpublished => "unpublished",
        };
        sqlx::query(include_str!("sql/update_state.sql"))
            .bind(lifecycle)
            .bind(publication)
            .bind(super::super::mapping::visibility_name(
                material.visibility(),
            ))
            .bind(id.0)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        audit::record(
            &mut tx,
            user,
            id,
            None,
            action,
            cause,
            json!({"lifecycle":lifecycle,"publication":publication}),
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
