SELECT
    'type'::text AS kind,
    NULL::bigint AS value,
    material_type AS name,
    NULL::text AS slug,
    count(*) AS material_count
FROM public_materials
GROUP BY material_type

UNION ALL

SELECT
    'year',
    source_year::bigint,
    NULL,
    NULL,
    count(*)
FROM public_materials
WHERE source_year IS NOT NULL
GROUP BY source_year

UNION ALL

SELECT
    'discipline',
    d.id,
    d.name,
    d.slug,
    count(DISTINCT p.id)
FROM public_materials p
JOIN discipline_placements x
    ON x.material_id = p.id
JOIN disciplines d
    ON d.id = x.discipline_id
GROUP BY d.id

UNION ALL

SELECT
    'teacher',
    u.id,
    u.display_name,
    NULL,
    count(DISTINCT p.id)
FROM public_materials p
JOIN discipline_placements x
    ON x.material_id = p.id
JOIN users u
    ON u.id = x.responsible_user_id
GROUP BY u.id

UNION ALL

SELECT
    'course',
    x.course::bigint,
    NULL,
    NULL,
    count(DISTINCT p.id)
FROM public_materials p
JOIN discipline_placements x
    ON x.material_id = p.id
WHERE x.course IS NOT NULL
GROUP BY x.course

ORDER BY kind, value, name;
