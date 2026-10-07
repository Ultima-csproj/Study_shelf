WHERE
  c.visibility = 'public'
  AND (
    $1::text = ''
    OR NOT EXISTS (
      SELECT
        1
      FROM
        unnest(string_to_array($1::text, ' ')) AS words (word)
      WHERE
        word <> ''
        AND strpos(
          replace(
            lower(
              c.title || ' ' || c.description || ' ' || c.purpose
            ),
            'ё',
            'е'
          ),
          word
        ) = 0
    )
  )
  AND (
    cardinality($2::bigint[]) = 0
    OR c.discipline_id = ANY ($2)
  )
  AND (
    cardinality($3::smallint[]) = 0
    OR c.course = ANY ($3)
  )
  AND (
    $4::bigint IS NULL
    OR c.owner_institution_id = $4
  )
