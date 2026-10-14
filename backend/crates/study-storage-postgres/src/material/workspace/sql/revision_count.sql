SELECT
    count(*)
FROM material_revisions
WHERE material_id = $1
    AND ($2::text IS NULL OR state = $2)
