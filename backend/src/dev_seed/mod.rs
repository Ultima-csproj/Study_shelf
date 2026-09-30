mod data;

use anyhow::Result;
use sqlx::SqlitePool;
use study_core::catalog::normalize_search_text;

use self::data::{DISCIPLINES, TEACHERS};

const FORMATS: [(&str, &str, &str); 5] = [
    ("lecture", "Лекция", "Теория"),
    ("book", "Учебное пособие", "Чтение"),
    ("article", "Статья", "Разбор"),
    ("video", "Видео", "Видео"),
    ("lab", "Практическая работа", "Практика"),
];

pub async fn populate(pool: &SqlitePool) -> Result<u64> {
    let mut transaction = pool.begin().await?;
    let mut inserted = 0;

    for (id, name, short_name) in TEACHERS {
        sqlx::query(
            "INSERT INTO teachers (id, name, short_name) VALUES (?, ?, ?) ON CONFLICT(id) DO NOTHING",
        )
        .bind(id)
        .bind(name)
        .bind(short_name)
        .execute(&mut *transaction)
        .await?;
    }

    for discipline in DISCIPLINES {
        let description = format!("Учебные материалы по дисциплине «{}».", discipline.name);
        sqlx::query(
            "INSERT INTO disciplines (id, slug, name, description, course, semester, teacher_id) \
             VALUES (?, ?, ?, ?, ?, ?, ?) ON CONFLICT(id) DO NOTHING",
        )
        .bind(discipline.id)
        .bind(discipline.slug)
        .bind(discipline.name)
        .bind(description)
        .bind(discipline.course)
        .bind(discipline.semester)
        .bind(discipline.teacher_id)
        .execute(&mut *transaction)
        .await?;

        let teacher_name = TEACHERS
            .iter()
            .find(|(id, _, _)| *id == discipline.teacher_id)
            .map(|(_, name, _)| *name)
            .expect("discipline teacher is declared in TEACHERS");

        for (topic_index, &(topic, topic_tag)) in discipline.topics.iter().enumerate() {
            for (format_index, &(material_type, label, format_tag)) in FORMATS.iter().enumerate() {
                let id = discipline.id * 1000 + topic_index as i64 * 10 + format_index as i64 + 1;
                let title = format!("{label}. {topic}");
                let description = format!(
                    "{topic}: учебный материал по дисциплине «{}».",
                    discipline.name
                );
                let year = 2023 + (topic_index as i64 + format_index as i64) % 4;
                let created_at = format!("{year}-09-{:02}", topic_index + 1);
                let tags = [topic_tag, format_tag];
                let search_text = normalize_search_text(&format!(
                    "{title} {description} {} {teacher_name} {}",
                    discipline.name,
                    tags.join(" ")
                ));
                let sort_title = normalize_search_text(&title);
                let duration_seconds =
                    (material_type == "video").then_some(1200 + topic_index as i64 * 90);

                let result = sqlx::query(
                    "INSERT INTO materials \
                     (id, title, description, material_type, discipline_id, teacher_id, course, year, \
                      created_at, duration_seconds, search_text, sort_title) \
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(id) DO NOTHING",
                )
                .bind(id)
                .bind(title)
                .bind(description)
                .bind(material_type)
                .bind(discipline.id)
                .bind(discipline.teacher_id)
                .bind(discipline.course)
                .bind(year)
                .bind(created_at)
                .bind(duration_seconds)
                .bind(search_text)
                .bind(sort_title)
                .execute(&mut *transaction)
                .await?;
                inserted += result.rows_affected();

                for tag in tags {
                    sqlx::query(
                        "INSERT INTO material_tags (material_id, tag) VALUES (?, ?) \
                         ON CONFLICT(material_id, tag) DO NOTHING",
                    )
                    .bind(id)
                    .bind(tag)
                    .execute(&mut *transaction)
                    .await?;
                }
            }
        }
    }

    transaction.commit().await?;
    Ok(inserted)
}
