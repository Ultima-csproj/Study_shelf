use super::MaterialError;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RevisionMetadata {
    pub source_year: Option<i32>,
    pub preview_url: Option<String>,
    pub duration_seconds: Option<u32>,
    pub file_size_bytes: Option<i64>,
}

impl RevisionMetadata {
    pub(super) fn validate(&self) -> Result<(), MaterialError> {
        if self
            .source_year
            .is_some_and(|year| !(1..=9999).contains(&year))
        {
            return Err(MaterialError::InvalidContent(
                super::ContentError::InvalidSourceYear,
            ));
        }
        if self.file_size_bytes.is_some_and(|size| size < 0) {
            return Err(MaterialError::InvalidContent(
                super::ContentError::InvalidFileSize,
            ));
        }
        Ok(())
    }
}
