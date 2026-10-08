INSERT INTO material_restrictions (
    material_id,
    institution_id,
    created_by_user_id,
    reason
)
VALUES (
    $1,
    $2,
    $3,
    $4
)
RETURNING id
