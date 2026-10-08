WHERE (
    (
        $1::text = 'author'
        AND (
            m.owner_user_id = $2
            OR EXISTS (
                SELECT 1
                FROM institution_memberships AS membership
                JOIN membership_permissions AS permission
                    USING (user_id, institution_id)
                WHERE membership.user_id = $2
                    AND membership.institution_id = m.owner_institution_id
                    AND membership.state = 'active'
                    AND (
                        permission.permission = 'manage_materials'
                        OR (
                            m.created_by_user_id = $2
                            AND permission.permission = 'create_materials'
                        )
                    )
            )
            OR EXISTS (
                SELECT 1
                FROM material_collaborators AS collaborator
                JOIN collaborator_permissions AS permission
                    USING (user_id, material_id)
                WHERE collaborator.material_id = m.id
                    AND collaborator.user_id = $2
                    AND collaborator.active
            )
        )
    )
    OR (
        $1::text = 'pending'
        AND h.working_revision_state = 'pending_moderation'
        AND m.lifecycle = 'active'
        AND EXISTS (
            SELECT 1
            FROM user_platform_permissions
            WHERE user_id = $2
                AND permission = 'moderate_public_materials'
        )
    )
    OR (
        $1::text = 'restricted'
        AND EXISTS (
            SELECT 1
            FROM material_restrictions AS restriction
            WHERE restriction.material_id = m.id
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
        )
    )
)
AND ($3 = '' OR h.title ILIKE '%' || $3 || '%' OR h.id::text = $3)
AND ($4::text IS NULL OR h.lifecycle = $4)
