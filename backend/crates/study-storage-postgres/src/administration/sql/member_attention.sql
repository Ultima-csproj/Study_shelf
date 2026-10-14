SELECT
    institution.id AS institution_id,
    institution.name AS institution_name,
    count(*) AS pending_count
FROM institution_memberships AS membership
JOIN institutions AS institution
    ON institution.id = membership.institution_id
WHERE membership.state = 'pending'
    AND (
        $1
        OR membership.institution_id = ANY ($2::bigint[])
    )
GROUP BY
    institution.id,
    institution.name
ORDER BY
    pending_count DESC,
    institution.id
LIMIT 10
