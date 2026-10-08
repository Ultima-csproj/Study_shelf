use super::{PostgresAccountRepository, session};
use crate::error::map_sqlx_error;
use sqlx::FromRow;
use study_core::account::{AccountError, AccountResult};

#[derive(FromRow)]
pub struct MembershipRequest {
    pub institution_id: i64,
    pub name: String,
    pub state: String,
}
#[derive(FromRow)]
pub struct LinkRequest {
    pub contributor_id: i64,
    pub display_name: String,
    pub state: String,
}
pub struct IdentityRequests {
    pub memberships: Vec<MembershipRequest>,
    pub links: Vec<LinkRequest>,
}
#[derive(FromRow)]
pub struct InstitutionReference {
    pub id: i64,
    pub name: String,
}
pub struct InstitutionPage {
    pub items: Vec<InstitutionReference>,
    pub total: i64,
}
impl PostgresAccountRepository {
    pub async fn institutions(&self, page: u32, size: u32) -> AccountResult<InstitutionPage> {
        if page == 0 || !(1..=100).contains(&size) {
            return Err(AccountError::Validation("Некорректная страница"));
        }
        let mut tx = session::snapshot(&self.pool).await?;
        let total = sqlx::query_scalar(
            r#"
                SELECT
                  count(*)
                FROM
                  institutions
            "#,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let items = sqlx::query_as::<_, InstitutionReference>(
            r#"
                SELECT
                  id,
                  name
                FROM
                  institutions
                ORDER BY
                  name,
                  id
                LIMIT
                  $1
                OFFSET
                  $2
            "#,
        )
        .bind(i64::from(size))
        .bind(i64::from(page - 1) * i64::from(size))
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(InstitutionPage { items, total })
    }
    pub async fn request_membership(&self, digest: &[u8], institution: i64) -> AccountResult<()> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let user = session::require(&mut tx, digest).await?.user_id;
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
        sqlx::query(
            r#"
                INSERT INTO
                  institution_memberships (user_id, institution_id, state, requested_at)
                VALUES
                  ($1, $2, 'pending', clock_timestamp())
                ON CONFLICT (user_id, institution_id) DO UPDATE
                SET
                  state = 'pending',
                  requested_at = clock_timestamp(),
                  decision_reason = NULL,
                  edit_version = institution_memberships.edit_version + 1
                WHERE
                  institution_memberships.state IN ('rejected', 'revoked')
            "#,
        )
        .bind(user)
        .bind(institution)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
    pub async fn request_contributor(
        &self,
        digest: &[u8],
        contributor: i64,
        evidence: &str,
    ) -> AccountResult<()> {
        if evidence.trim().chars().count() < 3 || evidence.chars().count() > 2000 {
            return Err(AccountError::Validation(
                "Укажите основание привязки автора: 3–2000 символов",
            ));
        }
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        let user = session::require(&mut tx, digest).await?.user_id;
        let linked: Option<Option<i64>> = sqlx::query_scalar(
            r#"
                SELECT
                  user_id
                FROM
                  contributors
                WHERE
                  id = $1
                  AND kind = 'person'
                FOR SHARE
            "#,
        )
        .bind(contributor)
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        match linked {
            None => return Err(AccountError::NotFound),
            Some(Some(id)) if id != user => return Err(AccountError::ContributorAlreadyLinked),
            Some(Some(_)) => return Ok(()),
            Some(None) => {}
        }
        sqlx::query(
            r#"
                INSERT INTO
                  contributor_link_requests (user_id, contributor_id, state, evidence)
                VALUES
                  ($1, $2, 'pending', $3)
                ON CONFLICT (user_id, contributor_id) DO UPDATE
                SET
                  state = 'pending',
                  evidence = $3,
                  decision_reason = NULL,
                  created_at = clock_timestamp(),
                  edit_version = contributor_link_requests.edit_version + 1
                WHERE
                  contributor_link_requests.state IN ('rejected', 'pending')
            "#,
        )
        .bind(user)
        .bind(contributor)
        .bind(evidence.trim())
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }
    pub async fn identity_requests(&self, digest: &[u8]) -> AccountResult<IdentityRequests> {
        let mut tx = session::snapshot(&self.pool).await?;
        let user = session::read(&mut tx, digest).await?.user_id;
        let memberships = sqlx::query_as::<_, MembershipRequest>(
            r#"
                SELECT
                  m.institution_id,
                  i.name,
                  m.state
                FROM
                  institution_memberships m
                  JOIN institutions i ON i.id = m.institution_id
                WHERE
                  m.user_id = $1
                ORDER BY
                  i.id
                LIMIT
                  100
            "#,
        )
        .bind(user)
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let links = sqlx::query_as::<_, LinkRequest>(
            r#"
                SELECT
                  r.contributor_id,
                  c.display_name,
                  r.state
                FROM
                  contributor_link_requests r
                  JOIN contributors c ON c.id = r.contributor_id
                WHERE
                  r.user_id = $1
                ORDER BY
                  c.id
                LIMIT
                  100
            "#,
        )
        .bind(user)
        .fetch_all(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(IdentityRequests { memberships, links })
    }
}
