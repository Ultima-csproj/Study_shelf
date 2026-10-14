SELECT
    m.id,
    CASE
        WHEN $1::text = 'pending' THEN working.title
        WHEN $1::text = 'restricted' AND m.visibility = 'private'
            THEN 'Ограниченный материал #' || m.id::text
        WHEN $1::text = 'author' THEN COALESCE(working.title, published.title, latest.title)
        ELSE COALESCE(published.title, 'Материал #' || m.id::text)
    END AS title,
    m.lifecycle,
    m.publication_state AS publication,
    m.visibility,
    m.edit_version AS version,
    m.owner_user_id,
    m.owner_institution_id,
    COALESCE(owner_user.display_name, owner_institution.name) AS owner_name,
    CASE
        WHEN $1::text = 'restricted' AND m.visibility = 'private' THEN NULL
        ELSE m.published_revision_id
    END AS published_revision_id,
    CASE
        WHEN $1::text IN ('restricted', 'public') THEN NULL
        ELSE working.id
    END AS working_revision_id,
    CASE
        WHEN $1::text IN ('restricted', 'public') THEN NULL
        ELSE working.state
    END AS working_revision_state,
    content.material_type,
    content.source_year,
    left(content.description, 300) AS description,
    content.updated_at::text AS updated_at,
    CASE
        WHEN $1::text = 'restricted' AND m.visibility = 'private' THEN NULL
        WHEN published.id IS NULL THEN NULL
        ELSE (
            SELECT
                count(*)
            FROM material_revisions AS numbered
            WHERE numbered.material_id = m.id
                AND numbered.id <= published.id
        )
    END AS published_revision_number
FROM materials AS m
LEFT JOIN users AS owner_user
    ON owner_user.id = m.owner_user_id
LEFT JOIN institutions AS owner_institution
    ON owner_institution.id = m.owner_institution_id
LEFT JOIN material_revisions AS published
    ON published.id = m.published_revision_id
LEFT JOIN LATERAL (
    SELECT
        r.id,
        r.title,
        r.state,
        r.material_type,
        r.source_year,
        r.description,
        r.updated_at
    FROM material_revisions AS r
    WHERE r.material_id = m.id
        AND r.state IN ('draft', 'pending_moderation')
    ORDER BY r.id DESC
    LIMIT 1
) AS working ON TRUE
LEFT JOIN LATERAL (
    SELECT
        r.title,
        r.material_type,
        r.source_year,
        r.description,
        r.updated_at
    FROM material_revisions AS r
    WHERE r.material_id = m.id
    ORDER BY r.id DESC
    LIMIT 1
) AS latest ON $1::text = 'author'
LEFT JOIN LATERAL (
    SELECT
        CASE
            WHEN $1::text = 'author' THEN
                COALESCE(working.material_type, published.material_type, latest.material_type)
            WHEN $1::text = 'pending' THEN working.material_type
            ELSE published.material_type
        END AS material_type,
        CASE
            WHEN $1::text = 'author' THEN CASE
                WHEN working.id IS NOT NULL THEN working.source_year
                WHEN published.id IS NOT NULL THEN published.source_year
                ELSE latest.source_year
            END
            WHEN $1::text = 'pending' THEN working.source_year
            ELSE published.source_year
        END AS source_year,
        CASE
            WHEN $1::text = 'author' THEN
                COALESCE(working.description, published.description, latest.description)
            WHEN $1::text = 'pending' THEN working.description
            ELSE published.description
        END AS description,
        CASE
            WHEN $1::text = 'author' THEN
                COALESCE(working.updated_at, published.updated_at, latest.updated_at)
            WHEN $1::text = 'pending' THEN working.updated_at
            ELSE published.updated_at
        END AS updated_at
    WHERE NOT ($1::text = 'restricted' AND m.visibility = 'private')
) AS content ON TRUE
