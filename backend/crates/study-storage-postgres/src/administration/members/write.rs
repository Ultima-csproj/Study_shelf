use super::MembershipChange;
use crate::administration::{
    access::Access,
    audit::{self, AuditEvent},
    guard::LastManagers,
};
use crate::error::map_sqlx_error;
use serde_json::json;
use sqlx::PgConnection;
use study_core::{
    account::{AccountError, AccountResult},
    institution::{AcademicRole, InstitutionPermission, MembershipState},
};
pub(super) async fn apply(
    conn: &mut PgConnection,
    actor: &Access,
    user: i64,
    institution: i64,
    expected: Option<i64>,
    change: MembershipChange,
    reason: &str,
) -> AccountResult<()> {
    let active: Option<bool> = sqlx::query_scalar(
        r#"
            SELECT
              state = 'active'
            FROM
              users
            WHERE
              id = $1
            FOR UPDATE
        "#,
    )
    .bind(user)
    .fetch_optional(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    if active.is_none() {
        return Err(AccountError::NotFound);
    }
    if change.state == Some(MembershipState::Active) && active != Some(true) {
        return Err(AccountError::AccountDisabled);
    }
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
    .fetch_one(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    if !exists {
        return Err(AccountError::NotFound);
    }
    let old: Option<(String, i64)> = sqlx::query_as(
        r#"
            SELECT
              state,
              edit_version
            FROM
              institution_memberships
            WHERE
              user_id = $1
              AND institution_id = $2
            FOR UPDATE
        "#,
    )
    .bind(user)
    .bind(institution)
    .fetch_optional(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    if expected.is_some_and(|version| version != old.as_ref().map_or(0, |m| m.1)) {
        return Err(AccountError::VersionConflict);
    }
    let before_roles: Vec<String> = sqlx::query_scalar(
        r#"
            SELECT
              role
            FROM
              membership_academic_roles
            WHERE
              user_id = $1
              AND institution_id = $2
            ORDER BY
              role
        "#,
    )
    .bind(user)
    .bind(institution)
    .fetch_all(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    let before_permissions: Vec<String> = sqlx::query_scalar(
        r#"
            SELECT
              permission
            FROM
              membership_permissions
            WHERE
              user_id = $1
              AND institution_id = $2
            ORDER BY
              permission
        "#,
    )
    .bind(user)
    .bind(institution)
    .fetch_all(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    let last = LastManagers::capture(conn, user).await?;
    let state = match change.state {
        Some(MembershipState::Active) => "active",
        Some(MembershipState::Pending) => "pending",
        Some(MembershipState::Rejected) => "rejected",
        Some(MembershipState::Revoked) => "revoked",
        None => old
            .as_ref()
            .map(|m| m.0.as_str())
            .ok_or(AccountError::NotFound)?,
    };
    let roles = change
        .roles
        .map(|rs| {
            rs.into_iter()
                .map(|r| match r {
                    AcademicRole::Student => "student".to_owned(),
                    AcademicRole::Teacher => "teacher".to_owned(),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| before_roles.clone());
    let permissions = if state != "active" {
        Vec::new()
    } else {
        change
            .permissions
            .map(|ps| {
                ps.into_iter()
                    .map(|p| match p {
                        InstitutionPermission::ManageMembers => "manage_members".to_owned(),
                        InstitutionPermission::ManageDisciplines => "manage_disciplines".to_owned(),
                        InstitutionPermission::CreateMaterials => "create_materials".to_owned(),
                        InstitutionPermission::ManageMaterials => "manage_materials".to_owned(),
                        InstitutionPermission::RestrictMaterials => "restrict_materials".to_owned(),
                        InstitutionPermission::ReviewRestrictedMaterials => {
                            "review_restricted_materials".to_owned()
                        }
                    })
                    .collect()
            })
            .unwrap_or_else(|| before_permissions.clone())
    };
    sqlx::query(
        r#"
            INSERT INTO
              institution_memberships (user_id, institution_id, state, decision_reason)
            VALUES
              ($1, $2, $3, $4)
            ON CONFLICT (user_id, institution_id) DO UPDATE
            SET
              state = $3,
              decision_reason = $4,
              edit_version = institution_memberships.edit_version + 1
        "#,
    )
    .bind(user)
    .bind(institution)
    .bind(state)
    .bind(reason.trim())
    .execute(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    sqlx::query(
        r#"
            DELETE FROM membership_academic_roles
            WHERE
              user_id = $1
              AND institution_id = $2
        "#,
    )
    .bind(user)
    .bind(institution)
    .execute(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    sqlx::query(
        r#"
            INSERT INTO
              membership_academic_roles (user_id, institution_id, role)
            SELECT
              $1,
              $2,
              r
            FROM
              unnest($3::text[]) r
            ON CONFLICT DO NOTHING
        "#,
    )
    .bind(user)
    .bind(institution)
    .bind(&roles)
    .execute(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    sqlx::query(
        r#"
            DELETE FROM membership_permissions
            WHERE
              user_id = $1
              AND institution_id = $2
        "#,
    )
    .bind(user)
    .bind(institution)
    .execute(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    sqlx::query(
        r#"
            INSERT INTO
              membership_permissions (user_id, institution_id, permission)
            SELECT
              $1,
              $2,
              p
            FROM
              unnest($3::text[]) p
            ON CONFLICT DO NOTHING
        "#,
    )
    .bind(user)
    .bind(institution)
    .bind(&permissions)
    .execute(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    last.check(conn).await?;
    audit::write(conn,actor,AuditEvent {kind:"membership",id:user,institution:Some(institution),action:"set_membership",reason,
        before:json!({"state":old.as_ref().map(|m|m.0.as_str()),"roles":before_roles,"permissions":before_permissions}),after:json!({"state":state,"roles":roles,"permissions":permissions})})
        .await?;
    Ok(())
}
