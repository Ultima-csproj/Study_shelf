SELECT
    m.id,
    CASE
        WHEN $1::text = 'pending' THEN working.title
        WHEN $1::text = 'restricted' AND m.visibility = 'private'
            THEN 'Ограниченный материал #' || m.id::text
        ELSE COALESCE(published.title, working.title, 'Материал #' || m.id::text)
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
    CASE WHEN $1::text = 'restricted' THEN NULL ELSE working.id END AS working_revision_id,
    CASE WHEN $1::text = 'restricted' THEN NULL ELSE working.state END AS working_revision_state
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
        r.state
    FROM material_revisions AS r
    WHERE r.material_id = m.id
        AND r.state IN ('draft', 'pending_moderation')
    ORDER BY r.id DESC
    LIMIT 1
) AS working ON TRUE
