mod query;
mod sections;
pub use query::{LearningQuery, LearningQueryError, LearningSort, PageRequest};
pub use sections::{
    LearningDetail, LearningPage, LearningSection, SectionEntry, SectionPage, SectionScope,
};

#[derive(Debug, Clone)]
pub struct InstitutionReference {
    pub id: crate::ids::InstitutionId,
    pub name: String,
}
