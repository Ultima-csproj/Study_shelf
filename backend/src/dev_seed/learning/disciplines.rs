use super::super::{SeedError, identities::World};
use sqlx::PgPool;

pub(super) async fn populate(pool: &PgPool, world: &World) -> Result<(), SeedError> {
    for (index, discipline) in world.disciplines.iter().enumerate() {
        let mut tx = pool.begin().await?;
        sqlx::query(
            r#"
            SELECT
              pg_advisory_xact_lock(hashtextextended ($1, 0))
        "#,
        )
        .bind(format!("discipline-structure:v3:{}", discipline.0))
        .execute(&mut *tx)
        .await?;
        for (position, (title, description, types)) in [
            (
                "Начните здесь",
                "Введение и основные понятия.",
                vec!["book", "article"],
            ),
            (
                "Лекции и объяснения",
                "Последовательное изучение темы.",
                vec!["lecture"],
            ),
            (
                "Практика",
                "Задания для самостоятельной работы.",
                vec!["lab"],
            ),
        ]
        .into_iter()
        .enumerate()
        {
            let key = format!("discipline-section:v3:{}:{position}", discipline.0);
            let section = sqlx::query_scalar::<_, i64>(
                r#"
                    INSERT INTO
                      discipline_sections (
                        discipline_id,
                        fixture_key,
                        title,
                        description,
                        position
                      )
                    VALUES
                      ($1, $2, $3, $4, $5)
                    ON CONFLICT (fixture_key) DO NOTHING
                    RETURNING
                      id
                "#,
            )
            .bind(discipline.0)
            .bind(&key)
            .bind(title)
            .bind(description)
            .bind((position + 1) as i32)
            .fetch_optional(&mut *tx)
            .await?;
            let Some(section) = section else {
                continue;
            };
            if position == 0 {
                sqlx::query(
                    r#"
                        UPDATE disciplines
                        SET
                          description = $1,
                          summary = $2
                        WHERE
                          id = $3
                          AND description = ''
                    "#
                ).bind("Материалы для изучения дисциплины: введение, объяснения и практические задания. Учебные контексты могут отличаться курсом и ответственным участником.")
                 .bind("Введение, теория и практика в одном учебном маршруте.")
                 .bind(discipline.0).execute(&mut *tx).await?;
                let material = sqlx::query_scalar::<_, i64>(
                    r#"
                        SELECT
                          p.material_id
                        FROM
                          discipline_placements p
                          JOIN public_materials m ON m.id = p.material_id
                        WHERE
                          p.discipline_id = $1
                          AND m.material_type = 'book'
                        ORDER BY
                          p.material_id
                        LIMIT
                          1
                    "#,
                )
                .bind(discipline.0)
                .fetch_optional(&mut *tx)
                .await?;
                if let Some(material) = material {
                    let institution = sqlx::query_scalar::<_, i64>(
                        r#"
                            SELECT
                              institution_id
                            FROM
                              disciplines
                            WHERE
                              id = $1
                        "#,
                    )
                    .bind(discipline.0)
                    .fetch_one(&mut *tx)
                    .await?;
                    for (course, user) in [
                        (2_i16, world.users[index % 6]),
                        (3_i16, world.users[(index + 1) % 6]),
                    ] {
                        sqlx::query(
                            r#"
                                INSERT INTO
                                  discipline_placements (
                                    material_id,
                                    discipline_id,
                                    institution_id,
                                    responsible_user_id,
                                    course
                                  )
                                VALUES
                                  ($1, $2, $3, $4, $5)
                                ON CONFLICT ON CONSTRAINT placement_context_unique DO NOTHING
                            "#,
                        )
                        .bind(material)
                        .bind(discipline.0)
                        .bind(institution)
                        .bind(user.0)
                        .bind(course)
                        .execute(&mut *tx)
                        .await?;
                    }
                }
            }
            sqlx::query(
                r#"
                    INSERT INTO
                      discipline_section_entries (discipline_id, section_id, placement_id, position)
                    SELECT
                      $1,
                      $2,
                      p.id,
                      (
                        row_number() OVER (
                          ORDER BY
                            p.material_id,
                            p.id
                        )
                      )::integer
                    FROM
                      discipline_placements p
                      JOIN LATERAL (
                        SELECT
                          r.material_type
                        FROM
                          material_revisions r
                        WHERE
                          r.material_id = p.material_id
                        ORDER BY
                          r.id
                        LIMIT
                          1
                      ) r ON TRUE
                    WHERE
                      p.discipline_id = $1
                      AND r.material_type = ANY ($3::text[])
                "#,
            )
            .bind(discipline.0)
            .bind(section)
            .bind(types)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
    }
    Ok(())
}
