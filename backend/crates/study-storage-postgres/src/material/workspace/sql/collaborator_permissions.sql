INSERT INTO collaborator_permissions (
    material_id,
    user_id,
    permission
)
SELECT
    $1,
    $2,
    grant_name
FROM unnest($3::text[]) AS grant_name
