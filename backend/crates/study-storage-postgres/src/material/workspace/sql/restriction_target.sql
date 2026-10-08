SELECT
    institution_id
FROM material_restrictions
WHERE id = $1
    AND material_id = $2
    AND revoked_at IS NULL
FOR UPDATE
