SELECT
    collaborator.user_id,
    account.display_name AS name,
    collaborator.active,
    ARRAY(
        SELECT permission
        FROM collaborator_permissions
        WHERE material_id = collaborator.material_id
            AND user_id = collaborator.user_id
        ORDER BY permission
    ) AS permissions
FROM material_collaborators AS collaborator
JOIN users AS account
    ON account.id = collaborator.user_id
WHERE collaborator.material_id = $1
ORDER BY collaborator.user_id
LIMIT $2
OFFSET $3
