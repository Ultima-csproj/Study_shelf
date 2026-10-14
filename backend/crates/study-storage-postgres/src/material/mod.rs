mod access;
mod audit;
mod children;
mod draft;
mod edit;
mod fixture;
mod load;
pub(crate) mod mapping;
mod publication;
mod retry;
mod row;
mod workspace;
mod write;
pub use fixture::{FixtureMode, FixturePlacement, MaterialFixture};
pub use workspace::*;
pub use write::{CreateMaterial, MaterialWriteError, PostgresMaterialRepository};

pub struct StoredRevision {
    pub revision: study_core::material::MaterialRevision,
    pub edit_version: i64,
}
