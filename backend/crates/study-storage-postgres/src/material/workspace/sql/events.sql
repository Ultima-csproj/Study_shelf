SELECT
    event.id,
    event.revision_id,
    account.display_name AS actor_name,
    event.action,
    event.reason,
    event.created_at::text AS created_at
FROM material_events AS event
JOIN users AS account
    ON account.id = event.actor_user_id
WHERE event.material_id = $1
ORDER BY event.id DESC
LIMIT $2
OFFSET $3
