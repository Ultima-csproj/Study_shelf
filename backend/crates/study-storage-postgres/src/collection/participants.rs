use crate::error::map_sqlx_error;
use futures_util::TryStreamExt;
use sqlx::PgConnection;
use std::collections::HashMap;
use study_core::collection::{CollectionParticipant, CollectionRole, CollectionSummary};
use study_core::error::RepositoryResult;
use study_core::ids::ContributorId;

pub(super) async fn populate(
    conn: &mut PgConnection,
    items: &mut [CollectionSummary],
) -> RepositoryResult<()> {
    if items.is_empty() {
        return Ok(());
    }
    let ids: Vec<i64> = items.iter().map(|item| item.id.0).collect();
    let positions: HashMap<i64, usize> = items
        .iter()
        .enumerate()
        .map(|(index, item)| (item.id.0, index))
        .collect();
    let mut rows = sqlx::query_as::<_, (i64, i64, String, String, i32)>(
        r#"
            SELECT
              collection_id,
              contributor_id,
              display_name,
              role,
              position
            FROM
              collection_participants
            WHERE
              collection_id = ANY ($1)
            ORDER BY
              collection_id,
              position
        "#,
    )
    .bind(&ids)
    .fetch(conn);
    while let Some((parent, id, name, role, position)) =
        rows.try_next().await.map_err(map_sqlx_error)?
    {
        let role = match role.as_str() {
            "author" => CollectionRole::Author,
            "curator" => CollectionRole::Curator,
            "contributor" => CollectionRole::Contributor,
            value => return Err(crate::error::unknown("collection participant role", value)),
        };
        let index = *positions
            .get(&parent)
            .ok_or_else(|| crate::error::invalid_relation("collection participant parent"))?;
        items[index].contributors.push(CollectionParticipant {
            contributor_id: ContributorId(id),
            display_name: name,
            role,
            position: crate::learning::count(i64::from(position))?,
        });
    }
    Ok(())
}
