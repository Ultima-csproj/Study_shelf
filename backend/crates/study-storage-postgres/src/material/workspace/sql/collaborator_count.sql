SELECT
    count(*)
FROM material_collaborators
WHERE material_id = $1
