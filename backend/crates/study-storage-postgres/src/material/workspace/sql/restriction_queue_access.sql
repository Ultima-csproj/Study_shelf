SELECT
    EXISTS (
        SELECT 1
        FROM user_platform_permissions
        WHERE user_id = $1
            AND permission IN ('restrict_public_materials', 'review_restricted_materials')
    )
    OR EXISTS (
        SELECT 1
        FROM institution_memberships AS membership
        JOIN membership_permissions AS permission
            USING (user_id, institution_id)
        WHERE membership.user_id = $1
            AND membership.state = 'active'
            AND permission.permission IN ('restrict_materials', 'review_restricted_materials')
    )
