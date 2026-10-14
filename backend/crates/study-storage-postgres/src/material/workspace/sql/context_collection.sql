SELECT
    collection.owner_institution_id
FROM collections AS collection
WHERE collection.id = $1
    AND (collection.visibility = 'public' OR collection.owner_user_id = $3)
    AND EXISTS (
        SELECT 1
        FROM collection_entries
        WHERE collection_id = collection.id
            AND material_id = $2
    )
