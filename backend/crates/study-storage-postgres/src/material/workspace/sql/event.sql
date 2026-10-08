INSERT INTO material_events (
    material_id,
    revision_id,
    actor_user_id,
    action,
    reason,
    details
)
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6
)
