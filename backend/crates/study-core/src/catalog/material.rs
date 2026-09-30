use crate::ids::{DisciplineId, MaterialId, TeacherId};

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

#[derive(Debug, Clone)]
pub struct Material {
    pub id: MaterialId,
    pub title: String,
    pub description: String,
    pub material_type: MaterialType,

    pub discipline_id: DisciplineId,
    pub teacher_id: TeacherId,

    pub tags: Vec<String>,
    pub course: u8,
    pub year: i32,

    pub created_at: String,
    pub updated_at: Option<String>,

    pub author_name: Option<String>,
    pub preview_url: Option<String>,
    pub duration_seconds: Option<u32>,
    pub file_size_bytes: Option<i64>,
}
