UPDATE material_restrictions
SET
    revoked_by_user_id = $2,
    revoked_at = clock_timestamp(),
    revoke_reason = $3
WHERE id = $1
