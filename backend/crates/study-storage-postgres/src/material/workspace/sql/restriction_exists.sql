SELECT EXISTS (
    SELECT 1
    FROM material_restrictions
    WHERE material_id = $1
        AND institution_id IS NOT DISTINCT FROM $2
        AND revoked_at IS NULL
)
