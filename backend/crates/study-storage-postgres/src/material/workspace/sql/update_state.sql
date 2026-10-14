UPDATE materials
SET
    lifecycle = $1,
    publication_state = $2,
    visibility = $3,
    edit_version = edit_version + 1
WHERE id = $4
