mod dto;
mod read;
mod write;
use crate::api::error::ApiError;
pub use read::*;
use serde::Deserialize;
use study_storage_postgres::{administration::PageRequest, material::MaterialContext};
pub use write::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkspaceQuery {
    #[serde(default = "first")]
    page: u32,
    #[serde(default = "size")]
    page_size: u32,
    #[serde(default)]
    q: String,
    state: Option<String>,
    #[serde(default)]
    order: study_storage_postgres::material::MaterialListOrder,
    discipline_id: Option<i64>,
    collection_id: Option<i64>,
    restriction_id: Option<i64>,
}
fn first() -> u32 {
    1
}
fn size() -> u32 {
    20
}
impl WorkspaceQuery {
    fn page(&self) -> PageRequest {
        PageRequest {
            page: self.page,
            size: self.page_size,
            query: self.q.clone(),
            state: self.state.clone(),
        }
    }
    fn context(&self) -> MaterialContext {
        MaterialContext {
            restriction_id: self.restriction_id,
            discipline_id: self.discipline_id,
            collection_id: self.collection_id,
        }
    }
}
fn id(value: i64) -> Result<study_core::ids::MaterialId, ApiError> {
    if value <= 0 {
        return Err(ApiError::bad_request("ID должен быть положительным"));
    }
    Ok(study_core::ids::MaterialId(value))
}
