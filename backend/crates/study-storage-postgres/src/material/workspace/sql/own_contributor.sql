SELECT
    primary_contributor_id
FROM user_profiles
WHERE user_id = $1
