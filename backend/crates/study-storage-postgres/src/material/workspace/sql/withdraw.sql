UPDATE material_revisions
SET
    state = 'draft',
    submitted_by_user_id = NULL,
    submitted_at = NULL,
    updated_at = $1,
    edit_version = edit_version + 1
WHERE id = $2
