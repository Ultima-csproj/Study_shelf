SELECT
    m.id,
    r.id AS published_revision_id,
    r.title,
    r.description,
    r.material_type,
    r.source_year,
    r.preview_url,
    r.duration_seconds,
    r.file_size_bytes,
    m.first_published_at,
    r.published_at
FROM materials AS m
JOIN material_revisions AS r
    ON r.id = m.published_revision_id
    AND r.material_id = m.id
WHERE m.id = $1
    AND r.state = 'published'
