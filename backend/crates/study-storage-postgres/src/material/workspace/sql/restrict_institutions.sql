SELECT DISTINCT
    membership.institution_id
FROM institution_memberships AS membership
JOIN membership_permissions AS permission
    USING (user_id, institution_id)
WHERE membership.user_id = $1
    AND membership.state = 'active'
    AND permission.permission = 'restrict_materials'
ORDER BY membership.institution_id
