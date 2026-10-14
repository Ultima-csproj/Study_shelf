use super::{PostgresAccountRepository, session};
use crate::{
    administration::{
        access::Access,
        audit::{self, AuditEvent},
    },
    error::map_sqlx_error,
};
use serde_json::json;
use study_core::account::{AccountError, AccountResult};

impl PostgresAccountRepository {
    /// Requests membership for the authenticated account, never for an administrative target.
    pub async fn request_membership(&self, digest: &[u8], institution: i64) -> AccountResult<()> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        // require locks this user; administrative membership writes use the same lock order.
        let applicant = session::require(&mut tx, digest).await?.user_id;
        let exists: bool = sqlx::query_scalar(
            r#"
                SELECT
                    EXISTS (
                        SELECT
                            1
                        FROM
                            institutions
                        WHERE
                            id = $1
                    )
            "#,
        )
        .bind(institution)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if !exists {
            return Err(AccountError::NotFound);
        }
        let previous: Option<String> = sqlx::query_scalar(
            r#"
                SELECT
                    state
                FROM
                    institution_memberships
                WHERE
                    user_id = $1
                    AND institution_id = $2
                FOR UPDATE
            "#,
        )
        .bind(applicant)
        .bind(institution)
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        match previous.as_deref() {
            Some("pending") => return Err(AccountError::MembershipAlreadyPending),
            Some("active") => return Err(AccountError::MembershipAlreadyActive),
            Some("rejected" | "revoked") | None => {}
            Some(_) => return Err(AccountError::Validation("Неизвестное состояние членства")),
        }
        sqlx::query(
            r#"
                INSERT INTO
                    institution_memberships (
                        user_id,
                        institution_id,
                        state,
                        requested_at
                    )
                VALUES
                    ($1, $2, 'pending', clock_timestamp())
                ON CONFLICT (user_id, institution_id) DO UPDATE
                SET
                    state = 'pending',
                    requested_at = clock_timestamp(),
                    decision_reason = NULL,
                    edit_version = institution_memberships.edit_version + 1
            "#,
        )
        .bind(applicant)
        .bind(institution)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let actor = Access {
            user: Some(applicant),
            permissions: Vec::new(),
            local: false,
        };
        audit::write(
            &mut tx,
            &actor,
            AuditEvent {
                kind: "membership",
                id: applicant,
                institution: Some(institution),
                action: "request_membership",
                reason: "Личная заявка на вступление в учреждение",
                before: json!({ "state": previous }),
                after: json!({ "state": "pending" }),
            },
        )
        .await?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
}
