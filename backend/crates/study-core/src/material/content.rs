use crate::catalog::MaterialType;

use super::{MaterialError, RevisionCredit, RevisionMetadata};

/// The current metadata snapshot. Assets and embedded bodies are added with their use cases.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevisionContent {
    pub title: String,
    pub description: String,
    pub material_type: MaterialType,
    pub metadata: RevisionMetadata,
    pub tags: Vec<String>,
    pub credits: Vec<RevisionCredit>,
}

impl RevisionContent {
    pub(super) fn validate(&self) -> Result<(), MaterialError> {
        self.metadata.validate()?;
        if self.tags.len() > 100 || self.credits.len() > 100 {
            return Err(MaterialError::InvalidContent(
                super::ContentError::TooManyEntries,
            ));
        }
        for (index, tag) in self.tags.iter().enumerate() {
            if tag.trim().is_empty() || tag.chars().count() > 80 || self.tags[..index].contains(tag)
            {
                return Err(MaterialError::InvalidContent(
                    super::ContentError::InvalidTag,
                ));
            }
        }
        for (index, credit) in self.credits.iter().enumerate() {
            if self.credits[..index].iter().any(|previous| {
                previous.contributor_id == credit.contributor_id && previous.role == credit.role
            }) {
                return Err(MaterialError::InvalidContent(
                    super::ContentError::DuplicateCredit,
                ));
            }
        }
        if self.title.trim().is_empty() {
            return Err(MaterialError::InvalidContent(
                super::ContentError::EmptyTitle,
            ));
        }
        if self
            .credits
            .iter()
            .any(|credit| credit.contributor_id.0 <= 0)
        {
            return Err(MaterialError::InvalidIdentifier("contributor"));
        }
        if self
            .credits
            .iter()
            .any(|credit| credit.display_name.trim().is_empty())
        {
            return Err(MaterialError::InvalidContent(
                super::ContentError::EmptyCreditedName,
            ));
        }
        Ok(())
    }
}
