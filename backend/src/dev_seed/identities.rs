use super::SeedError;
use super::data::{DISCIPLINES, TEACHERS};
use sqlx::{PgConnection, PgPool};
use study_core::ids::{ContributorId, DisciplineId, InstitutionId, UserId};

pub(super) struct World {
    pub users: Vec<UserId>,
    pub institutions: Vec<InstitutionId>,
    pub contributors: Vec<(ContributorId, String)>,
    pub disciplines: Vec<DisciplineId>,
}
pub(super) async fn populate(pool: &PgPool) -> Result<World, SeedError> {
    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"
            SELECT pg_advisory_xact_lock(
                hashtextextended('study-seed-identities-v2', 0)
            )
        "#,
    )
    .execute(&mut *tx)
    .await?;
    let mut institutions = Vec::with_capacity(3);
    for (index, name) in [
        "Колледж цифровых технологий",
        "Институт прикладных наук",
        "Открытая учебная академия",
    ]
    .iter()
    .enumerate()
    {
        let key = format!("institution:{index}");
        let id = sqlx::query_scalar::<_, i64>(
            r#"
                INSERT INTO institutions (
                    fixture_key,
                    name
                )
                VALUES ($1, $2)
                ON CONFLICT (fixture_key) DO NOTHING
                RETURNING id
            "#,
        )
        .bind(&key)
        .bind(name)
        .fetch_optional(&mut *tx)
        .await?;
        let id = match id {
            Some(id) => id,
            None => {
                sqlx::query_scalar("SELECT id FROM institutions WHERE fixture_key=$1")
                    .bind(&key)
                    .fetch_one(&mut *tx)
                    .await?
            }
        };
        institutions.push(InstitutionId(id));
    }
    let mut users = Vec::with_capacity(8);
    let mut contributors = Vec::with_capacity(11);
    let names = TEACHERS
        .iter()
        .map(|(_, name, _)| *name)
        .chain(["Алексеева Полина — модератор", "Морозов Никита — студент"]);
    for (index, name) in names.enumerate() {
        let key = format!("user:{index}");
        let inserted = sqlx::query_scalar::<_, i64>(
            r#"
                INSERT INTO users (
                    fixture_key,
                    display_name,
                    state
                )
                VALUES ($1, $2, 'active')
                ON CONFLICT (fixture_key) DO NOTHING
                RETURNING id
            "#,
        )
        .bind(&key)
        .bind(name)
        .fetch_optional(&mut *tx)
        .await?;
        let id = match inserted {
            Some(id) => id,
            None => {
                sqlx::query_scalar("SELECT id FROM users WHERE fixture_key=$1")
                    .bind(&key)
                    .fetch_one(&mut *tx)
                    .await?
            }
        };
        if inserted.is_some() && index == 6 {
            sqlx::query(
                r#"
                    INSERT INTO user_platform_permissions (
                        user_id,
                        permission
                    )
                    VALUES ($1, 'moderate_public_materials')
                "#,
            )
            .bind(id)
            .execute(&mut *tx)
            .await?;
        }
        for institution in &institutions {
            let inserted = sqlx::query_scalar::<_, i64>(
                r#"
                    INSERT INTO institution_memberships (
                        user_id,
                        institution_id,
                        state
                    )
                    VALUES ($1, $2, 'active')
                    ON CONFLICT DO NOTHING
                    RETURNING user_id
                "#,
            )
            .bind(id)
            .bind(institution.0)
            .fetch_optional(&mut *tx)
            .await?;
            if inserted.is_some() {
                let role = if index < 6 { "teacher" } else { "student" };
                sqlx::query(
                    r#"
                        INSERT INTO membership_academic_roles (
                            user_id,
                            institution_id,
                            role
                        )
                        VALUES ($1, $2, $3)
                    "#,
                )
                .bind(id)
                .bind(institution.0)
                .bind(role)
                .execute(&mut *tx)
                .await?;
                if index < 6 {
                    sqlx::query(
                        r#"
                            INSERT INTO membership_permissions (
                                user_id,
                                institution_id,
                                permission
                            )
                            SELECT
                                $1,
                                $2,
                                unnest(ARRAY[
                                    'create_materials',
                                    'manage_materials',
                                    'manage_disciplines'
                                ])
                        "#,
                    )
                    .bind(id)
                    .bind(institution.0)
                    .execute(&mut *tx)
                    .await?;
                }
            }
        }
        users.push(UserId(id));
        let contributor = contributor(
            &mut tx,
            &format!("contributor:user:{index}"),
            name,
            Some(id),
            None,
        )
        .await?;
        contributors.push((contributor, name.to_owned()));
    }
    for (key, name) in [
        ("external:0", "Внешний автор: Дарья Соколова"),
        ("external:1", "Внешний редактор: Роман Лебедев"),
    ] {
        contributors.push((
            contributor(&mut tx, key, name, None, None).await?,
            name.to_owned(),
        ));
    }
    contributors.push((
        contributor(
            &mut tx,
            "organization:0",
            "Открытая учебная академия",
            None,
            Some(institutions[2].0),
        )
        .await?,
        "Открытая учебная академия".to_owned(),
    ));
    let mut disciplines = Vec::with_capacity(DISCIPLINES.len());
    for (index, discipline) in DISCIPLINES.iter().enumerate() {
        let key = format!("discipline:{index}");
        let inserted = sqlx::query_scalar::<_, i64>(
            r#"
                INSERT INTO disciplines (
                    fixture_key,
                    institution_id,
                    slug,
                    name
                )
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (fixture_key) DO NOTHING
                RETURNING id
            "#,
        )
        .bind(&key)
        .bind(institutions[index % 3].0)
        .bind(discipline.slug)
        .bind(discipline.name)
        .fetch_optional(&mut *tx)
        .await?;
        let id = match inserted {
            Some(id) => id,
            None => {
                sqlx::query_scalar("SELECT id FROM disciplines WHERE fixture_key=$1")
                    .bind(&key)
                    .fetch_one(&mut *tx)
                    .await?
            }
        };
        disciplines.push(DisciplineId(id));
    }
    tx.commit().await?;
    Ok(World {
        users,
        institutions,
        contributors,
        disciplines,
    })
}
async fn contributor(
    conn: &mut PgConnection,
    key: &str,
    name: &str,
    user: Option<i64>,
    institution: Option<i64>,
) -> Result<ContributorId, SeedError> {
    let kind = if institution.is_some() {
        "organization"
    } else {
        "person"
    };
    let inserted = sqlx::query_scalar::<_, i64>(
        r#"
            INSERT INTO contributors (
                fixture_key,
                kind,
                display_name,
                user_id,
                institution_id
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (fixture_key) DO NOTHING
            RETURNING id
        "#,
    )
    .bind(key)
    .bind(kind)
    .bind(name)
    .bind(user)
    .bind(institution)
    .fetch_optional(&mut *conn)
    .await?;
    let id = match inserted {
        Some(id) => id,
        None => {
            sqlx::query_scalar("SELECT id FROM contributors WHERE fixture_key=$1")
                .bind(key)
                .fetch_one(conn)
                .await?
        }
    };
    Ok(ContributorId(id))
}
