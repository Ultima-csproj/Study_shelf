SELECT
    restriction.id,
    restriction.institution_id,
    institution.name AS institution_name,
    restriction.reason,
    restriction.created_at::text AS created_at
FROM material_restrictions AS restriction
LEFT JOIN institutions AS institution
    ON institution.id = restriction.institution_id
WHERE restriction.material_id = $1
    AND restriction.revoked_at IS NULL
ORDER BY restriction.id
