FROM public_materials p
LEFT JOIN material_search s
    ON s.material_id = p.id
    AND s.revision_id = p.published_revision_id
WHERE (
    $1::text = ''
    OR NOT EXISTS (
        SELECT 1
        FROM unnest(string_to_array($1::text, ' ')) AS words(word)
        WHERE word <> ''
            AND strpos(COALESCE(s.search_text, ''), word) = 0
    )
)
AND (
    cardinality($2::text[]) = 0
    OR p.material_type = ANY($2)
)
AND (
    (
        cardinality($3::bigint[]) = 0
        AND cardinality($4::bigint[]) = 0
        AND cardinality($5::smallint[]) = 0
    )
    OR EXISTS (
        SELECT 1
        FROM discipline_placements x
        WHERE x.material_id = p.id
            AND (
                cardinality($3::bigint[]) = 0
                OR x.discipline_id = ANY($3)
            )
            AND (
                cardinality($4::bigint[]) = 0
                OR x.responsible_user_id = ANY($4)
            )
            AND (
                cardinality($5::smallint[]) = 0
                OR x.course = ANY($5)
            )
    )
)
AND (
    cardinality($6::integer[]) = 0
    OR p.source_year = ANY($6)
)
AND NOT EXISTS (
    SELECT 1
    FROM unnest($7::text[]) AS requested(wanted)
    WHERE NOT EXISTS (
        SELECT 1
        FROM revision_tags t
        WHERE t.revision_id = p.published_revision_id
            AND t.tag = wanted
    )
)
