use super::{mapping, write::MaterialWriteError};
use crate::error::map_sqlx_error;
use sqlx::{PgConnection, Postgres, QueryBuilder};
use study_core::material::MaterialRevision;

pub(super) async fn insert_draft(
    conn: &mut PgConnection,
    revision: &MaterialRevision,
) -> Result<(), MaterialWriteError> {
    let content = revision.content();
    let meta = &content.metadata;
    sqlx::query(
        r#"
            INSERT INTO
              material_revisions (
                id,
                material_id,
                base_revision_id,
                created_by_user_id,
                created_at,
                updated_at,
                state,
                title,
                description,
                material_type,
                source_year,
                preview_url,
                duration_seconds,
                file_size_bytes
              )
            VALUES
              (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                'draft',
                $7,
                $8,
                $9,
                $10,
                $11,
                $12,
                $13
              )
        "#,
    )
    .bind(revision.id().0)
    .bind(revision.material_id().0)
    .bind(revision.base_revision_id().map(|id| id.0))
    .bind(revision.created_by_user_id().0)
    .bind(study_core::timestamp::to_utc(revision.created_at())?)
    .bind(study_core::timestamp::to_utc(revision.updated_at())?)
    .bind(&content.title)
    .bind(&content.description)
    .bind(content.material_type.as_db_value())
    .bind(meta.source_year)
    .bind(meta.preview_url.as_deref())
    .bind(meta.duration_seconds.map(i64::from))
    .bind(meta.file_size_bytes)
    .execute(&mut *conn)
    .await
    .map_err(map_sqlx_error)?;
    save_children(conn, revision).await
}

pub(super) async fn save_children(
    conn: &mut PgConnection,
    revision: &MaterialRevision,
) -> Result<(), MaterialWriteError> {
    let id = revision.id().0;
    for sql in [
        r#"
            DELETE FROM revision_tags
            WHERE
              revision_id = $1
        "#,
        r#"
            DELETE FROM revision_credits
            WHERE
              revision_id = $1
        "#,
        r#"
            DELETE FROM revision_editors
            WHERE
              revision_id = $1
        "#,
    ] {
        sqlx::query(sql)
            .bind(id)
            .execute(&mut *conn)
            .await
            .map_err(map_sqlx_error)?;
    }
    if !revision.content().tags.is_empty() {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"
                INSERT INTO
                  revision_tags (revision_id, tag, position)
            "#,
        );
        query.push_values(
            revision.content().tags.iter().enumerate(),
            |mut row, (position, tag)| {
                row.push_bind(id).push_bind(tag).push_bind(position as i32);
            },
        );
        query
            .build()
            .execute(&mut *conn)
            .await
            .map_err(map_sqlx_error)?;
    }
    if !revision.content().credits.is_empty() {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"
                INSERT INTO
                  revision_credits (
                    revision_id,
                    contributor_id,
                    display_name,
                    role,
                    position
                  )
            "#,
        );
        query.push_values(
            revision.content().credits.iter().enumerate(),
            |mut row, (position, credit)| {
                row.push_bind(id)
                    .push_bind(credit.contributor_id.0)
                    .push_bind(&credit.display_name)
                    .push_bind(mapping::role_name(credit.role))
                    .push_bind(position as i32);
            },
        );
        query
            .build()
            .execute(&mut *conn)
            .await
            .map_err(map_sqlx_error)?;
    }
    if !revision.changed_by().is_empty() {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"
                INSERT INTO
                  revision_editors (revision_id, user_id)
            "#,
        );
        query.push_values(revision.changed_by(), |mut row, user| {
            row.push_bind(id).push_bind(user.0);
        });
        query.build().execute(conn).await.map_err(map_sqlx_error)?;
    }
    Ok(())
}
