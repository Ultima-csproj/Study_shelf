SELECT DISTINCT
    membership.institution_id
FROM institution_memberships AS membership
JOIN membership_permissions AS permission
    USING (user_id, institution_id)
JOIN materials AS material
    ON material.id = $1
WHERE membership.user_id = $2
    AND membership.state = 'active'
    AND permission.permission = 'restrict_materials'
    AND (
        material.owner_institution_id = membership.institution_id
        OR EXISTS (
            SELECT 1
            FROM discipline_placements
            WHERE material_id = material.id
                AND institution_id = membership.institution_id
        )
    )
ORDER BY membership.institution_id
