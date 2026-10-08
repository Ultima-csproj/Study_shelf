SELECT
    count(*)
FROM material_events
WHERE material_id = $1
