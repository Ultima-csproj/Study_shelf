//! Explicit dev fixtures. No public HTTP route calls this module.
use super::{
    access, children,
    draft::submit_in,
    load,
    publication::approve_in,
    write::{
        CreateMaterial, MaterialWriteError, PostgresMaterialRepository, create_in, next_id,
        owner_institution,
    },
};
use crate::error::map_sqlx_error;
use sqlx::FromRow;
use std::time::{Duration, SystemTime};
use study_core::ids::{DisciplineId, InstitutionId, MaterialId, MaterialRevisionId, UserId};
use study_core::material::{MaterialRevision, MaterialVisibility};

#[derive(Clone, Copy)]
pub enum FixtureMode {
    Draft,
    Published,
    PublishedWithDraft,
    Updated,
}
pub struct FixturePlacement {
    pub discipline_id: DisciplineId,
    pub institution_id: InstitutionId,
    pub responsible_user_id: Option<UserId>,
    pub course: Option<u8>,
}
pub struct MaterialFixture {
    pub creator: UserId,
    pub moderator: UserId,
    pub material: CreateMaterial,
    pub mode: FixtureMode,
    pub placements: Vec<FixturePlacement>,
    pub at: SystemTime,
}
#[derive(FromRow)]
struct Member {
    state: String,
    teacher: bool,
}

impl PostgresMaterialRepository {
    /// One fixture commits as a whole. Existing fixtures are never rewritten.
    pub async fn create_fixture(
        &self,
        mut fixture: MaterialFixture,
    ) -> Result<Option<MaterialId>, MaterialWriteError> {
        let key = fixture
            .material
            .fixture_key
            .as_deref()
            .ok_or(MaterialWriteError::InvalidFixture)?;
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        sqlx::query(
            r#"
                SELECT pg_advisory_xact_lock(
                    hashtextextended($1, 0)
                )
            "#,
        )
        .bind(key)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        let existing = sqlx::query_scalar::<_, i64>(
            r#"
                    SELECT id
                    FROM materials
                    WHERE fixture_key = $1
                "#,
        )
        .bind(key)
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;
        if existing.is_some() {
            tx.commit().await.map_err(map_sqlx_error)?;
            return Ok(None);
        }
        let target_visibility = fixture.material.visibility;
        if !matches!(fixture.mode, FixtureMode::Draft) {
            fixture.material.visibility = MaterialVisibility::Public;
        }
        let mut next_content = fixture.material.content.clone();
        let (mut material, revision) =
            create_in(&mut tx, fixture.creator, fixture.material, fixture.at).await?;
        for placement in fixture.placements {
            if placement.course == Some(0) {
                return Err(MaterialWriteError::InvalidFixture);
            }
            if let Some(user) = placement.responsible_user_id {
                let member = sqlx::query_as::<_, Member>(
                    r#"
                        SELECT
                            m.state,
                            EXISTS (
                                SELECT 1
                                FROM membership_academic_roles AS a
                                WHERE a.user_id = m.user_id
                                    AND a.institution_id = m.institution_id
                                    AND a.role = 'teacher'
                            ) AS teacher
                        FROM institution_memberships AS m
                        WHERE m.user_id = $1
                            AND m.institution_id = $2
                        FOR SHARE OF m
                    "#,
                )
                .bind(user.0)
                .bind(placement.institution_id.0)
                .fetch_optional(&mut *tx)
                .await
                .map_err(map_sqlx_error)?;
                if member.is_none_or(|member| member.state != "active" || !member.teacher) {
                    return Err(MaterialWriteError::InvalidFixture);
                }
            }
            sqlx::query(
                r#"
                    INSERT INTO discipline_placements (
                        material_id,
                        discipline_id,
                        institution_id,
                        responsible_user_id,
                        course
                    )
                    VALUES ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(material.id().0)
            .bind(placement.discipline_id.0)
            .bind(placement.institution_id.0)
            .bind(placement.responsible_user_id.map(|id| id.0))
            .bind(placement.course.map(i16::from))
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        }
        if !matches!(fixture.mode, FixtureMode::Draft) {
            submit_in(
                &mut tx,
                fixture.creator,
                material.id(),
                revision.id(),
                after(fixture.at, 30)?,
            )
            .await?;
            approve_in(
                &mut tx,
                fixture.moderator,
                material.id(),
                revision.id(),
                after(fixture.at, 60)?,
            )
            .await?;
            material = load::material(&mut tx, material.id(), true)
                .await?
                .ok_or(MaterialWriteError::InvalidFixture)?;
            if matches!(
                fixture.mode,
                FixtureMode::PublishedWithDraft | FixtureMode::Updated
            ) {
                next_content.title.push_str(" — новая редакция");
                let actor = access::load(
                    &mut tx,
                    fixture.creator,
                    Some(material.id()),
                    owner_institution(material.owner()),
                )
                .await?;
                let id = MaterialRevisionId(
                    next_id(
                        &mut tx,
                        r#"
                            SELECT nextval(
                                pg_get_serial_sequence(
                                    'material_revisions',
                                    'id'
                                )
                            )
                        "#,
                    )
                    .await?,
                );
                let draft = MaterialRevision::new_draft(
                    &actor.actor(),
                    &material,
                    id,
                    next_content,
                    after(fixture.at, 120)?,
                )?;
                children::insert_draft(&mut tx, &draft).await?;
                if matches!(fixture.mode, FixtureMode::Updated) {
                    submit_in(
                        &mut tx,
                        fixture.creator,
                        material.id(),
                        id,
                        after(fixture.at, 150)?,
                    )
                    .await?;
                    approve_in(
                        &mut tx,
                        fixture.moderator,
                        material.id(),
                        id,
                        after(fixture.at, 180)?,
                    )
                    .await?;
                }
            }
            if target_visibility != MaterialVisibility::Public {
                let actor = access::load(
                    &mut tx,
                    fixture.creator,
                    Some(material.id()),
                    owner_institution(material.owner()),
                )
                .await?;
                material.set_visibility(&actor.actor(), target_visibility)?;
                sqlx::query("UPDATE materials SET visibility='private' WHERE id=$1")
                    .bind(material.id().0)
                    .execute(&mut *tx)
                    .await
                    .map_err(map_sqlx_error)?;
            }
        }
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(Some(material.id()))
    }
}
fn after(at: SystemTime, seconds: u64) -> Result<SystemTime, MaterialWriteError> {
    at.checked_add(Duration::from_secs(seconds))
        .ok_or_else(|| study_core::timestamp::TimestampError::OutOfRange.into())
}
