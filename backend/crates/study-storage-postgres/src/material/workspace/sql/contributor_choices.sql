SELECT
    contributor.id,
    contributor.display_name
FROM contributors AS contributor
WHERE (
    contributor.user_id = $1
    OR EXISTS (
        SELECT
            1
        FROM revision_credits AS credit
        JOIN materials AS material
            ON material.published_revision_id = credit.revision_id
        WHERE credit.contributor_id = contributor.id
            AND material.visibility = 'public'
            AND material.publication_state = 'published'
            AND material.lifecycle = 'active'
            AND NOT EXISTS (
                SELECT
                    1
                FROM material_restrictions AS restriction
                WHERE restriction.material_id = material.id
                    AND restriction.institution_id IS NULL
                    AND restriction.revoked_at IS NULL
            )
    )
)
AND (
    contributor.display_name ILIKE '%' || $3 || '%'
    OR contributor.id::text = $2
)
ORDER BY
    (contributor.user_id = $1) DESC NULLS LAST,
    contributor.display_name,
    contributor.id
LIMIT 20
