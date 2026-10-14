use super::super::{SeedError, identities::World};
use sqlx::PgPool;
use study_core::collection::{CollectionOwner, CollectionVisibility};

pub(super) async fn populate(pool: &PgPool, world: &World) -> Result<u64, SeedError> {
    let materials: Vec<(i64, bool)> = sqlx::query_as(
        r#"
            SELECT
              m.id,
              (
                m.visibility = 'public'
                AND m.published_revision_id IS NOT NULL
              ) AS available
            FROM
              materials m
              JOIN LATERAL (
                SELECT
                  material_type
                FROM
                  material_revisions r
                WHERE
                  r.material_id = m.id
                ORDER BY
                  r.id
                LIMIT
                  1
              ) r ON TRUE
            WHERE
              r.material_type <> 'video'
            ORDER BY
              m.id
        "#,
    )
    .fetch_all(pool)
    .await?;
    if materials.len() < 36 {
        return Err(SeedError::MissingLearningMaterials);
    }
    let hidden: Vec<i64> = materials
        .iter()
        .filter(|(_, public)| !public)
        .map(|(id, _)| *id)
        .collect();
    let mut inserted = 0;
    for index in 0..48_usize {
        let key = format!("collection:v3:{index}");
        let mut tx = pool.begin().await?;
        sqlx::query(
            r#"
            SELECT
              pg_advisory_xact_lock(hashtextextended ($1, 0))
        "#,
        )
        .bind(&key)
        .execute(&mut *tx)
        .await?;
        let creator = world.users[index % 6];
        let owner = if index % 3 == 0 {
            CollectionOwner::Institution(world.institutions[index % 3])
        } else {
            CollectionOwner::User(creator)
        };
        let visibility = if index % 13 == 0 {
            CollectionVisibility::Private
        } else {
            CollectionVisibility::Public
        };
        let (owner_user, owner_institution) = match owner {
            CollectionOwner::User(id) => (Some(id.0), None),
            CollectionOwner::Institution(id) => (None, Some(id.0)),
        };
        let topics = [
            "Базы данных",
            "Rust для начинающих",
            "Алгоритмы",
            "Математика",
            "Проектирование приложений",
            "Подготовка к экзамену",
            "Самостоятельная практика",
            "Повторение основ",
        ];
        let title = format!("{} · маршрут {}", topics[index % topics.len()], index + 1);
        let discipline = if index % 4 == 0 {
            None
        } else {
            Some(world.disciplines[index % world.disciplines.len()].0)
        };
        let course = if index % 4 == 0 {
            None
        } else {
            Some((index % 3 + 1) as i16)
        };
        let id=sqlx::query_scalar::<_,i64>(
            r#"
                INSERT INTO
                  collections (
                    fixture_key,
                    slug,
                    title,
                    short_description,
                    description,
                    purpose,
                    target_audience,
                    owner_user_id,
                    owner_institution_id,
                    created_by_user_id,
                    visibility,
                    discipline_id,
                    course,
                    difficulty,
                    estimated_minutes,
                    created_at,
                    updated_at
                  )
                VALUES
                  (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6,
                    $7,
                    $8,
                    $9,
                    $10,
                    $11,
                    $12,
                    $13,
                    $14,
                    $15,
                    TIMESTAMPTZ '2026-10-01T00:00:00Z' + $16::integer * INTERVAL '1 hour',
                    TIMESTAMPTZ '2026-10-01T00:00:00Z' + $16::integer * INTERVAL '1 hour'
                  )
                ON CONFLICT (fixture_key) DO NOTHING
                RETURNING
                  id
            "#
        ).bind(&key).bind(format!("learning-route-{}",index+1)).bind(&title)
         .bind("Последовательность материалов для изучения и практики.")
         .bind("Подборка объединяет материалы разных учебных контекстов и самостоятельные публикации. Содержание открывается в текущей опубликованной редакции.")
         .bind("Разобраться в теме и закрепить знания практикой.")
         .bind("Студенты и самостоятельные читатели.")
         .bind(owner_user).bind(owner_institution).bind(creator.0)
         .bind(match visibility{CollectionVisibility::Private=>"private",CollectionVisibility::Public=>"public"})
         .bind(discipline).bind(course).bind(["beginner","intermediate","advanced"][index%3])
         .bind(90_i32+(index%5) as i32*30).bind(index as i32).fetch_optional(&mut *tx).await?;
        let Some(id) = id else {
            tx.commit().await?;
            continue;
        };
        for (position, (contributor, role)) in [
            (index % 8, "author"),
            (8 + index % 2, "curator"),
            (10, "contributor"),
        ]
        .into_iter()
        .enumerate()
        {
            let (person, name) = &world.contributors[contributor];
            sqlx::query(
                r#"
                    INSERT INTO
                      collection_participants (
                        collection_id,
                        contributor_id,
                        display_name,
                        role,
                        position
                      )
                    VALUES
                      ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(id)
            .bind(person.0)
            .bind(name)
            .bind(role)
            .bind((position + 1) as i32)
            .execute(&mut *tx)
            .await?;
        }
        for (section_index, title) in [
            "Начните здесь",
            "Разберитесь в деталях",
            "Закрепите на практике",
        ]
        .into_iter()
        .enumerate()
        {
            let section = sqlx::query_scalar::<_, i64>(
                r#"
                    INSERT INTO
                      collection_sections (collection_id, title, description, position)
                    VALUES
                      ($1, $2, $3, $4)
                    RETURNING
                      id
                "#,
            )
            .bind(id)
            .bind(title)
            .bind("Последовательность учебных шагов.")
            .bind((section_index + 1) as i32)
            .fetch_one(&mut *tx)
            .await?;
            let mut ids = Vec::with_capacity(18);
            for step in 0..18 {
                let offset = (index * 11 + section_index * 17 + step) % materials.len();
                let material = if index % 11 == 0 && section_index == 2 && !hidden.is_empty() {
                    hidden[(index + step) % hidden.len()]
                } else if section_index > 0 && step == 0 {
                    materials[(index * 11) % materials.len()].0
                } else {
                    materials[offset].0
                };
                if !ids.contains(&material) {
                    ids.push(material);
                }
            }
            for (position, material) in ids.into_iter().enumerate() {
                sqlx::query(
                    r#"
                        INSERT INTO
                          collection_entries (collection_id, section_id, material_id, position)
                        VALUES
                          ($1, $2, $3, $4)
                    "#,
                )
                .bind(id)
                .bind(section)
                .bind(material)
                .bind((position + 1) as i32)
                .execute(&mut *tx)
                .await?;
            }
        }
        tx.commit().await?;
        inserted += 1;
    }
    Ok(inserted)
}
