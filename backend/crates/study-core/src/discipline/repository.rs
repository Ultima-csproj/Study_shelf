use super::DisciplineSummary;
use crate::error::RepositoryResult;
use crate::ids::{DisciplineId, SectionId};
use crate::learning::{LearningDetail, LearningPage, LearningQuery, PageRequest, SectionPage};
use async_trait::async_trait;
#[async_trait]
pub trait DisciplineRepository: Send + Sync {
    async fn list(
        &self,
        query: &LearningQuery,
    ) -> RepositoryResult<LearningPage<DisciplineSummary>>;
    async fn get_by_id(
        &self,
        id: DisciplineId,
    ) -> RepositoryResult<Option<LearningDetail<DisciplineSummary>>>;
    async fn section_items(
        &self,
        id: DisciplineId,
        section: SectionId,
        page: PageRequest,
    ) -> RepositoryResult<Option<SectionPage>>;
}
