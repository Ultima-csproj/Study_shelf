use crate::catalog::{MaterialSummary, TeacherReference};
use crate::ids::{CollectionId, DisciplineId, MaterialId, PlacementId, SectionEntryId, SectionId};

#[derive(Debug, Clone, Copy)]
pub enum SectionScope {
    Discipline(DisciplineId),
    Collection(CollectionId),
}

#[derive(Debug)]
pub struct LearningPage<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_count: u32,
}
#[derive(Debug)]
pub struct SectionEntry {
    pub id: SectionEntryId,
    pub material_id: MaterialId,
    pub placement_id: Option<PlacementId>,
    pub position: u64,
    pub responsible: Option<TeacherReference>,
    pub course: Option<u8>,
}
#[derive(Debug)]
pub struct LearningSection {
    pub id: SectionId,
    pub title: String,
    pub description: String,
    pub position: u64,
    pub step_count: u64,
    pub items: Vec<SectionEntry>,
}
#[derive(Debug)]
pub struct LearningDetail<T> {
    pub summary: T,
    pub sections: Vec<LearningSection>,
    /// One summary per Material, even when entries repeat across sections.
    pub materials: Vec<MaterialSummary>,
}
#[derive(Debug)]
pub struct SectionPage {
    pub items: LearningPage<SectionEntry>,
    pub materials: Vec<MaterialSummary>,
}
