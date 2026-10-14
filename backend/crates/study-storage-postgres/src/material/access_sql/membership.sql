SELECT
                  m.state,
                  ARRAY(
                    SELECT
                      role
                    FROM
                      membership_academic_roles
                    WHERE
                      user_id = m.user_id
                      AND institution_id = m.institution_id
                  ) AS roles,
                  ARRAY(
                    SELECT
                      permission
                    FROM
                      membership_permissions
                    WHERE
                      user_id = m.user_id
                      AND institution_id = m.institution_id
                  ) AS permissions
                FROM
                  institution_memberships AS m
                WHERE
                  m.user_id = $1
                  AND m.institution_id = $2
