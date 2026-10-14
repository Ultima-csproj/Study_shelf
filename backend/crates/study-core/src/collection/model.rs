use crate::catalog::DisciplineReference;
use crate::ids::{CollectionId, ContributorId, InstitutionId, UserId};
use std::time::SystemTime;

#[derive(Debug, Clone, Copy)]
pub enum CollectionOwner {
    User(UserId),
    Institution(InstitutionId),
}
#[derive(Debug, Clone, Copy)]
pub enum CollectionVisibility {
    Private,
    Public,
}
#[derive(Debug, Clone, Copy)]
pub enum CollectionRole {
    Author,
    Curator,
    Contributor,
}
#[derive(Debug, Clone, Copy)]
pub enum CollectionDifficulty {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug)]
pub struct CollectionParticipant {
    pub contributor_id: ContributorId,
    pub display_name: String,
    pub role: CollectionRole,
    pub position: u64,
}
#[derive(Debug)]
pub struct CollectionSummary {
    pub id: CollectionId,
    pub slug: String,
    pub title: String,
    pub short_description: String,
    pub description: String,
    pub purpose: String,
    pub target_audience: String,
    pub discipline: Option<DisciplineReference>,
    pub course: Option<u8>,
    pub difficulty: Option<CollectionDifficulty>,
    pub estimated_minutes: Option<u32>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub contributors: Vec<CollectionParticipant>,
    pub step_count: u64,
    pub unique_material_count: u64,
}
