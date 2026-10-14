SELECT DISTINCT
    discipline.institution_id
FROM disciplines AS discipline
JOIN discipline_placements AS placement
    ON placement.discipline_id = discipline.id
WHERE discipline.id = $1
    AND placement.material_id = $2
