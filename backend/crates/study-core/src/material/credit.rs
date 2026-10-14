use crate::ids::ContributorId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContributorRole {
    Author,
    CoAuthor,
    Editor,
    Translator,
    Lecturer,
    Reviewer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevisionCredit {
    pub contributor_id: ContributorId,
    /// The credited name in this revision, independent of subsequent profile changes.
    pub display_name: String,
    pub role: ContributorRole,
}
