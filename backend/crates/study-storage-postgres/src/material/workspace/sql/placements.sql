SELECT
    placement.id,
    discipline.id AS discipline_id,
    discipline.name AS discipline_name,
    institution.id AS institution_id,
    institution.name AS institution_name,
    (
        SELECT count(*)
        FROM discipline_section_entries
        WHERE placement_id = placement.id
    ) AS entry_count,
    EXISTS (
        SELECT 1
        FROM institution_memberships AS membership
        JOIN membership_permissions AS permission
            USING (user_id, institution_id)
        WHERE membership.user_id = $2
            AND membership.institution_id = placement.institution_id
            AND membership.state = 'active'
            AND permission.permission = 'manage_disciplines'
    ) AS can_detach
FROM discipline_placements AS placement
JOIN disciplines AS discipline
    ON discipline.id = placement.discipline_id
JOIN institutions AS institution
    ON institution.id = placement.institution_id
WHERE placement.material_id = $1
ORDER BY placement.id
LIMIT 100
