SELECT
              u.id,
              u.display_name,
              u.state,
              ARRAY(
                SELECT
                  permission
                FROM
                  user_platform_permissions
                WHERE
                  user_id = u.id
              ) AS permissions
            FROM
              users AS u
            WHERE
              u.id = $1
