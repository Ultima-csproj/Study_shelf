use std::time::SystemTime;
use study_core::catalog::MaterialType;
use study_core::error::{RepositoryError, RepositoryResult};
use study_core::material::{ContributorRole, MaterialVisibility, RevisionState};
use time::OffsetDateTime;

pub(crate) fn stored_time(value: OffsetDateTime) -> RepositoryResult<SystemTime> {
    study_core::timestamp::from_utc(value)
        .map_err(|error| RepositoryError::invalid_data_with_source("invalid timestamp", error))
}
pub(crate) fn visibility(value: &str) -> RepositoryResult<MaterialVisibility> {
    match value {
        "public" => Ok(MaterialVisibility::Public),
        "private" => Ok(MaterialVisibility::Private),
        _ => Err(crate::error::unknown("visibility", value)),
    }
}
pub(crate) fn state(value: &str) -> RepositoryResult<RevisionState> {
    match value {
        "draft" => Ok(RevisionState::Draft),
        "pending_moderation" => Ok(RevisionState::PendingModeration),
        "published" => Ok(RevisionState::Published),
        "rejected" => Ok(RevisionState::Rejected),
        _ => Err(crate::error::unknown("revision state", value)),
    }
}
pub(crate) fn type_from_db(value: &str) -> RepositoryResult<MaterialType> {
    MaterialType::from_db_value(value).ok_or_else(|| crate::error::unknown("material type", value))
}
pub(crate) fn role(value: &str) -> RepositoryResult<ContributorRole> {
    match value {
        "author" => Ok(ContributorRole::Author),
        "co_author" => Ok(ContributorRole::CoAuthor),
        "editor" => Ok(ContributorRole::Editor),
        "translator" => Ok(ContributorRole::Translator),
        "lecturer" => Ok(ContributorRole::Lecturer),
        "reviewer" => Ok(ContributorRole::Reviewer),
        _ => Err(crate::error::unknown("credit role", value)),
    }
}
pub(crate) fn role_name(value: ContributorRole) -> &'static str {
    match value {
        ContributorRole::Author => "author",
        ContributorRole::CoAuthor => "co_author",
        ContributorRole::Editor => "editor",
        ContributorRole::Translator => "translator",
        ContributorRole::Lecturer => "lecturer",
        ContributorRole::Reviewer => "reviewer",
    }
}
pub(crate) fn visibility_name(value: MaterialVisibility) -> &'static str {
    match value {
        MaterialVisibility::Public => "public",
        MaterialVisibility::Private => "private",
    }
}
