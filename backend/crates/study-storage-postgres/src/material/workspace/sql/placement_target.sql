SELECT
    institution_id
FROM discipline_placements
WHERE id = $1
    AND material_id = $2
FOR UPDATE
