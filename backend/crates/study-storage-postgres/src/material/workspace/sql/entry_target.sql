SELECT
    placement.institution_id
FROM discipline_section_entries AS entry
JOIN discipline_placements AS placement
    ON placement.id = entry.placement_id
WHERE entry.id = $1
    AND placement.material_id = $2
FOR UPDATE OF entry, placement
