SELECT
    entry.id,
    section.id AS section_id,
    section.title AS section_title,
    entry.position
FROM discipline_section_entries AS entry
JOIN discipline_sections AS section
    ON section.id = entry.section_id
WHERE entry.placement_id = $1
ORDER BY
    section.position,
    entry.position,
    entry.id
LIMIT $2
OFFSET $3
