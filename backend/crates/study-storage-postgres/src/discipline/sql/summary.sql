SELECT
  d.id,
  d.slug,
  d.name,
  d.description,
  d.summary,
  d.institution_id,
  i.name AS institution_name,
  placements.material_count,
  placements.courses,
  entries.step_count
FROM
  disciplines d
  JOIN institutions i ON i.id = d.institution_id
  LEFT JOIN LATERAL (
    SELECT
      count(DISTINCT p.material_id) AS material_count,
      COALESCE(
        array_agg(
          DISTINCT p.course
          ORDER BY
            p.course
        ) FILTER (
          WHERE
            p.course IS NOT NULL
        ),
        ARRAY[]::smallint[]
      ) AS courses
    FROM
      discipline_placements p
      JOIN public_materials m ON m.id = p.material_id
        AND m.lifecycle = 'active'
        AND material_publicly_readable(m.id, d.institution_id)
    WHERE
      p.discipline_id = d.id
  ) placements ON TRUE
  LEFT JOIN LATERAL (
    SELECT
      count(*) AS step_count
    FROM
      discipline_section_entries e
      JOIN discipline_placements p ON p.id = e.placement_id
      JOIN public_materials m ON m.id = p.material_id
        AND m.lifecycle = 'active'
        AND material_publicly_readable(m.id, d.institution_id)
    WHERE
      e.discipline_id = d.id
  ) entries ON TRUE
