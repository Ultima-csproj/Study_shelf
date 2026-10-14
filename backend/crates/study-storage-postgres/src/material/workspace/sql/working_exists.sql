SELECT EXISTS (
    SELECT 1
    FROM material_revisions
    WHERE material_id = $1
        AND state IN ('draft', 'pending_moderation')
)
