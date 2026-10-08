SELECT EXISTS (
    SELECT 1
    FROM materials AS m
    WHERE m.id = $1
        AND (
            m.owner_institution_id = $2
            OR EXISTS (
                SELECT 1
                FROM discipline_placements AS placement
                WHERE placement.material_id = m.id
                    AND placement.institution_id = $2
            )
        )
)
