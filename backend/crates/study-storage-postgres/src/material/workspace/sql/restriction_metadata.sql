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
    AND (
        EXISTS (
            SELECT 1
            FROM user_platform_permissions
            WHERE user_id = $2
                AND permission IN (
                    'restrict_public_materials',
                    'review_restricted_materials'
                )
        )
        OR EXISTS (
            SELECT 1
            FROM institution_memberships AS membership
            JOIN membership_permissions AS permission
                USING (user_id, institution_id)
            WHERE membership.user_id = $2
                AND membership.institution_id = restriction.institution_id
                AND membership.state = 'active'
                AND permission.permission IN (
                    'restrict_materials',
                    'review_restricted_materials'
                )
        )
    )
ORDER BY restriction.id
LIMIT 100
