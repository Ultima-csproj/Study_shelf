use super::{SeedError, data::DISCIPLINES, identities::World};
use sqlx::PgPool;
use std::time::Duration;
use study_core::catalog::MaterialType;
use study_core::material::{
    ContributorRole, MaterialOwner, MaterialVisibility, RevisionContent, RevisionCredit,
    RevisionMetadata,
};
use study_storage_postgres::material::{
    CreateMaterial, FixtureMode, FixturePlacement, MaterialFixture, PostgresMaterialRepository,
};

pub(super) async fn populate(pool: &PgPool, world: &World) -> Result<u64, SeedError> {
    let repository = PostgresMaterialRepository::new(pool.clone());
    let baseline = study_core::timestamp::from_utc(time::OffsetDateTime::parse(
        "2026-10-04T00:00:00Z",
        &time::format_description::well_known::Rfc3339,
    )?)?;
    let types = [
        MaterialType::Lecture,
        MaterialType::Book,
        MaterialType::Article,
        MaterialType::Video,
        MaterialType::Lab,
    ];
    let labels = ["Лекция", "Книга", "Статья", "Видео", "Практикум"];
    let mut inserted = 0;
    for (discipline_index, discipline) in DISCIPLINES.iter().enumerate() {
        for (topic_index, (topic, tag)) in discipline.topics.iter().enumerate() {
            for edition in 0..2 {
                for (type_index, kind) in types.iter().enumerate() {
                    let index =
                        (((discipline_index * 12 + topic_index) * 2 + edition) * 5) + type_index;
                    let creator_index = if index % 19 == 0 {
                        7
                    } else if index % 23 == 0 {
                        6
                    } else {
                        (discipline.teacher_id - 1) as usize
                    };
                    let creator = world.users[creator_index];
                    let owner = if index % 3 == 0 && creator_index < 6 {
                        MaterialOwner::Institution(world.institutions[discipline_index % 3])
                    } else {
                        MaterialOwner::User(creator)
                    };
                    let credited = if index % 9 == 0 {
                        &world.contributors[8]
                    } else {
                        &world.contributors[creator_index]
                    };
                    let mut credits = vec![RevisionCredit {
                        contributor_id: credited.0,
                        display_name: credited.1.clone(),
                        role: ContributorRole::Author,
                    }];
                    if index % 4 == 0 {
                        let editor = &world.contributors[9];
                        credits.push(RevisionCredit {
                            contributor_id: editor.0,
                            display_name: editor.1.clone(),
                            role: ContributorRole::Editor,
                        });
                    }
                    if index % 11 == 0 {
                        let author = &world.contributors[10];
                        credits.push(RevisionCredit {
                            contributor_id: author.0,
                            display_name: author.1.clone(),
                            role: ContributorRole::CoAuthor,
                        });
                    }
                    let visibility = if index % 17 == 0 {
                        MaterialVisibility::Private
                    } else {
                        MaterialVisibility::Public
                    };
                    let mode = if index % 13 == 0 {
                        FixtureMode::Draft
                    } else if index % 7 == 0 {
                        FixtureMode::PublishedWithDraft
                    } else if index % 5 == 0 {
                        FixtureMode::Updated
                    } else {
                        FixtureMode::Published
                    };
                    let mut placements = Vec::with_capacity(2);
                    if index % 10 != 0 {
                        placements.push(FixturePlacement {
                            discipline_id: world.disciplines[discipline_index],
                            institution_id: world.institutions[discipline_index % 3],
                            responsible_user_id: Some(
                                world.users[(discipline.teacher_id - 1) as usize],
                            ),
                            course: Some(discipline.course as u8),
                        });
                        if index % 8 == 0 {
                            let next = (discipline_index + 1) % world.disciplines.len();
                            placements.push(FixturePlacement {
                                discipline_id: world.disciplines[next],
                                institution_id: world.institutions[next % 3],
                                responsible_user_id: None,
                                course: None,
                            });
                        }
                    }
                    let at = baseline
                        .checked_sub(Duration::from_secs((index % 365) as u64 * 86400))
                        .ok_or(study_core::timestamp::TimestampError::OutOfRange)?;
                    let content = RevisionContent {
                        title: format!(
                            "{}: {} · {}",
                            labels[type_index],
                            topic,
                            if edition == 0 {
                                "основы"
                            } else {
                                "углублённый курс"
                            }
                        ),
                        description: format!(
                            "{topic}. Материал по направлению «{}»: объяснение, примеры и вопросы для самостоятельной работы. Вымышленный демонстрационный набор.",
                            discipline.name
                        ),
                        material_type: *kind,
                        metadata: RevisionMetadata {
                            source_year: Some(2023 + (index % 4) as i32),
                            preview_url: preview(*kind),
                            duration_seconds: if *kind == MaterialType::Video {
                                Some(1200 + topic_index as u32 * 90)
                            } else {
                                None
                            },
                            file_size_bytes: if *kind != MaterialType::Video {
                                Some(240000 + index as i64 * 1300)
                            } else {
                                None
                            },
                        },
                        tags: vec![
                            tag.to_string(),
                            if edition == 0 {
                                "Для начинающих".to_owned()
                            } else {
                                "Практика".to_owned()
                            },
                        ],
                        credits,
                    };
                    let fixture = MaterialFixture {
                        creator,
                        moderator: world.users[6],
                        material: CreateMaterial {
                            owner,
                            visibility,
                            content,
                            fixture_key: Some(format!("material:v2:{index}")),
                        },
                        mode,
                        placements,
                        at,
                    };
                    if repository.create_fixture(fixture).await?.is_some() {
                        inserted += 1;
                    }
                }
            }
        }
    }
    Ok(inserted)
}
fn preview(kind: MaterialType) -> Option<String> {
    match kind {
        MaterialType::Video => Some("/previews/transactions-video.webp".to_owned()),
        MaterialType::Book => Some("/previews/database-book.webp".to_owned()),
        MaterialType::Article => Some("/previews/database-article.webp".to_owned()),
        _ => None,
    }
}
