DELETE FROM collaborator_permissions
WHERE material_id = $1
    AND user_id = $2
