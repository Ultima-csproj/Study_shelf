INSERT INTO material_collaborators (
    material_id,
    user_id,
    active
)
VALUES (
    $1,
    $2,
    $3
)
ON CONFLICT (material_id, user_id)
DO UPDATE SET active = $3
