use super::mapping;
use super::row::{MaterialRow, RevisionRow};
use crate::error::map_sqlx_error;
use futures_util::TryStreamExt;
use sqlx::PgConnection;
use study_core::error::RepositoryResult;
use study_core::ids::{ContributorId, MaterialId, MaterialRevisionId, UserId};
use study_core::material::{Material, RevisionCredit};

pub(super) async fn material(
    conn: &mut PgConnection,
    id: MaterialId,
    lock: bool,
) -> RepositoryResult<Option<Material>> {
    let sql = if lock {
        r#"
            SELECT
              m.*,
              r.published_at,
              r.state AS published_state
            FROM
              materials AS m
              LEFT JOIN material_revisions AS r ON r.id = m.published_revision_id
            WHERE
              m.id = $1
            FOR UPDATE OF
              m
        "#
    } else {
        r#"
            SELECT
              m.*,
              r.published_at,
              r.state AS published_state
            FROM
              materials AS m
              LEFT JOIN material_revisions AS r ON r.id = m.published_revision_id
            WHERE
              m.id = $1
        "#
    };
    sqlx::query_as::<_, MaterialRow>(sql)
        .bind(id.0)
        .fetch_optional(conn)
        .await
        .map_err(map_sqlx_error)?
        .map(MaterialRow::into_domain)
        .transpose()
}

pub(super) async fn revision(
    conn: &mut PgConnection,
    id: MaterialRevisionId,
    lock: bool,
) -> RepositoryResult<Option<super::StoredRevision>> {
    let sql = if lock {
        r#"
            SELECT
              *
            FROM
              material_revisions
            WHERE
              id = $1
            FOR UPDATE
        "#
    } else {
        r#"
            SELECT
              *
            FROM
              material_revisions
            WHERE
              id = $1
        "#
    };
    let Some(row) = sqlx::query_as::<_, RevisionRow>(sql)
        .bind(id.0)
        .fetch_optional(&mut *conn)
        .await
        .map_err(map_sqlx_error)?
    else {
        return Ok(None);
    };
    let mut tags = Vec::new();
    {
        let mut rows = sqlx::query_scalar::<_, String>(
            r#"
                SELECT
                  tag
                FROM
                  revision_tags
                WHERE
                  revision_id = $1
                ORDER BY
                  position
            "#,
        )
        .bind(id.0)
        .fetch(&mut *conn);
        while let Some(tag) = rows.try_next().await.map_err(map_sqlx_error)? {
            tags.push(tag);
        }
    }
    let mut credits = Vec::new();
    {
        let mut rows = sqlx::query_as::<_, (i64, String, String)>(
            r#"
                SELECT
                  contributor_id,
                  display_name,
                  role
                FROM
                  revision_credits
                WHERE
                  revision_id = $1
                ORDER BY
                  position
            "#,
        )
        .bind(id.0)
        .fetch(&mut *conn);
        while let Some((id, name, role)) = rows.try_next().await.map_err(map_sqlx_error)? {
            credits.push(RevisionCredit {
                contributor_id: ContributorId(id),
                display_name: name,
                role: mapping::role(&role)?,
            });
        }
    }
    let mut editors = Vec::new();
    {
        let mut rows = sqlx::query_scalar::<_, i64>(
            r#"
                SELECT
                  user_id
                FROM
                  revision_editors
                WHERE
                  revision_id = $1
                ORDER BY
                  user_id
            "#,
        )
        .bind(id.0)
        .fetch(&mut *conn);
        while let Some(id) = rows.try_next().await.map_err(map_sqlx_error)? {
            editors.push(UserId(id));
        }
    }
    let edit_version = row.edit_version;
    if edit_version < 0 {
        return Err(crate::error::invalid_relation("draft edit version"));
    }
    row.into_domain(tags, credits, editors).map(|revision| {
        Some(super::StoredRevision {
            revision,
            edit_version,
        })
    })
}
