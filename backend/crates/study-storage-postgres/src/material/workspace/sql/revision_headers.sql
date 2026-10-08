SELECT
    id,
    title,
    state,
    edit_version AS version,
    base_revision_id,
    created_at::text AS created_at,
    published_at::text AS published_at
FROM material_revisions
WHERE material_id = $1
    AND ($2::text IS NULL OR state = $2)
ORDER BY id DESC
LIMIT $3
OFFSET $4
