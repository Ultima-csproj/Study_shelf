#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ContentError {
    #[error("revision title is empty")]
    EmptyTitle,
    #[error("credited name is empty")]
    EmptyCreditedName,
    #[error("too many tags or credits")]
    TooManyEntries,
    #[error("invalid or duplicate tag")]
    InvalidTag,
    #[error("duplicate credit role")]
    DuplicateCredit,
    #[error("source year is out of range")]
    InvalidSourceYear,
    #[error("file size must be nonnegative")]
    InvalidFileSize,
    #[error("moderation rejection reason is empty")]
    EmptyRejectionReason,
}
