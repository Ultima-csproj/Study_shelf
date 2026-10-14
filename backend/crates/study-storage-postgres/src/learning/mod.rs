mod section_queries;
mod section_rows;
mod sections;
pub(crate) use section_rows::ScopeSql;
pub(crate) use sections::{detail_sections, section_items};

use study_core::error::{RepositoryError, RepositoryResult};
use study_core::learning::{LearningPage, LearningQueryError, PageRequest};

pub(crate) fn page<T>(
    items: Vec<T>,
    total: i64,
    request: PageRequest,
) -> RepositoryResult<LearningPage<T>> {
    let total = count(total)?;
    let page_count = u32::try_from(total.div_ceil(u64::from(request.page_size)))
        .map_err(|error| RepositoryError::invalid_data_with_source("page count overflow", error))?;
    Ok(LearningPage {
        items,
        total,
        page: request.page,
        page_count,
    })
}
pub(crate) fn count(value: i64) -> RepositoryResult<u64> {
    u64::try_from(value)
        .map_err(|error| RepositoryError::invalid_data_with_source("negative count", error))
}
pub(crate) fn validate_id(value: i64) -> RepositoryResult<()> {
    if value <= 0 {
        return Err(RepositoryError::invalid_query(
            LearningQueryError::InvalidIdentifier,
        ));
    }
    Ok(())
}
pub(crate) fn course(value: i16) -> RepositoryResult<u8> {
    u8::try_from(value)
        .ok()
        .filter(|value| *value > 0)
        .ok_or_else(|| crate::error::invalid_relation("course"))
}
