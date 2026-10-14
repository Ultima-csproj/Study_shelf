SELECT
    count(*)
FROM material_revisions
WHERE material_id = $1
    AND id <= $2
