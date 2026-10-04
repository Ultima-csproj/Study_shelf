use super::{DisciplineReference, TeacherReference};
use crate::ids::{MaterialId, MaterialRevisionId};
use crate::material::{RevisionCredit, RevisionMetadata};
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialType {
    Lecture,
    Book,
    Article,
    Video,
    Lab,
}

impl MaterialType {
    pub const fn as_db_value(self) -> &'static str {
        match self {
            Self::Lecture => "lecture",
            Self::Book => "book",
            Self::Article => "article",
            Self::Video => "video",
            Self::Lab => "lab",
        }
    }
    pub fn from_db_value(value: &str) -> Option<Self> {
        match value {
            "lecture" => Some(Self::Lecture),
            "book" => Some(Self::Book),
            "article" => Some(Self::Article),
            "video" => Some(Self::Video),
            "lab" => Some(Self::Lab),
            _ => None,
        }
    }
}

/// Complete public read projection. It contains no draft or ownership claims.
#[derive(Debug, Clone)]
pub struct MaterialSummary {
    pub id: MaterialId,
    pub published_revision_id: MaterialRevisionId,
    pub title: String,
    pub description: String,
    pub material_type: MaterialType,
    pub tags: Vec<String>,
    pub credits: Vec<RevisionCredit>,
    pub placements: Vec<EducationalPlacement>,
    pub metadata: RevisionMetadata,
    pub first_published_at: SystemTime,
    pub published_at: SystemTime,
}

#[derive(Debug, Clone)]
pub struct EducationalPlacement {
    pub discipline: DisciplineReference,
    pub responsible: Option<TeacherReference>,
    pub course: Option<u8>,
}
