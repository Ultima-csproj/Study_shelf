mod model;
mod repository;
use crate::catalog::DisciplineReference;
use crate::learning::LearningPage;
pub use model::{
    CollectionDifficulty, CollectionOwner, CollectionParticipant, CollectionRole,
    CollectionSummary, CollectionVisibility,
};
pub use repository::CollectionRepository;

pub struct CollectionPage {
    pub page: LearningPage<CollectionSummary>,
    pub disciplines: Vec<DisciplineReference>,
    pub courses: Vec<u8>,
}
