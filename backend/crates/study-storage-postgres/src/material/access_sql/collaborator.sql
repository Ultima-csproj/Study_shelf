SELECT
                  c.active,
                  ARRAY(
                    SELECT
                      permission
                    FROM
                      collaborator_permissions
                    WHERE
                      user_id = c.user_id
                      AND material_id = c.material_id
                  ) AS permissions
                FROM
                  material_collaborators AS c
                WHERE
                  c.user_id = $1
                  AND c.material_id = $2
