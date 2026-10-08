SELECT
    state
FROM users
WHERE id = $1
FOR UPDATE
