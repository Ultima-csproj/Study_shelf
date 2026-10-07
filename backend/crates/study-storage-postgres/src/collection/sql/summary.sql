SELECT
  c.id,
  c.slug,
  c.title,
  c.short_description,
  c.description,
  c.purpose,
  c.target_audience,
  c.discipline_id,
  d.slug AS discipline_slug,
  d.name AS discipline_name,
  c.course,
  c.difficulty,
  c.estimated_minutes,
  c.created_at,
  c.updated_at,
  counts.step_count,
  counts.unique_material_count
FROM
  collections c
  LEFT JOIN disciplines d ON d.id = c.discipline_id
  LEFT JOIN LATERAL (
    SELECT
      count(*) AS step_count,
      count(DISTINCT e.material_id) AS unique_material_count
    FROM
      collection_entries e
      JOIN public_materials m ON m.id = e.material_id
    WHERE
      e.collection_id = c.id
  ) counts ON TRUE
