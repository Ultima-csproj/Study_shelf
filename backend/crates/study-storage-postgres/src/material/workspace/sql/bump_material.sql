UPDATE materials
SET edit_version = edit_version + 1
WHERE id = $1
