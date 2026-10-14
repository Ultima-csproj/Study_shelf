SELECT
    id,
    display_name
FROM contributors
WHERE id = ANY($1::bigint[])
