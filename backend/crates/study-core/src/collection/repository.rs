use super::{CollectionPage, CollectionSummary};
use crate::error::RepositoryResult;
use crate::ids::{CollectionId, SectionId};
use crate::learning::{LearningDetail, LearningQuery, PageRequest, SectionPage};
use async_trait::async_trait;

#[async_trait]
pub trait CollectionRepository: Send + Sync {
    async fn list(&self, query: &LearningQuery) -> RepositoryResult<CollectionPage>;
    async fn get_by_id(
        &self,
        id: CollectionId,
    ) -> RepositoryResult<Option<LearningDetail<CollectionSummary>>>;
    async fn section_items(
        &self,
        id: CollectionId,
        section: SectionId,
        page: PageRequest,
    ) -> RepositoryResult<Option<SectionPage>>;
}
