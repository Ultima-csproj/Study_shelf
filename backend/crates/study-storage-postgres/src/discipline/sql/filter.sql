WHERE
  (
    $1::text = ''
    OR NOT EXISTS (
      SELECT
        1
      FROM
        unnest(string_to_array($1::text, ' ')) AS words (word)
      WHERE
        word <> ''
        AND strpos(
          replace(lower(d.name || ' ' || d.description), 'ё', 'е'),
          word
        ) = 0
    )
  )
  AND (
    $2::bigint IS NULL
    OR d.institution_id = $2
  )
