UPDATE material_revisions
SET
    state = 'discarded',
    updated_at = $1,
    edit_version = edit_version + 1
WHERE id = $2
