use crate::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Slug(String);

impl Slug {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        let normalized = value.trim().to_lowercase();

        if normalized.is_empty() {
            return Err(DomainError::EmptySlug);
        }

        if normalized.len() > 200 {
            return Err(DomainError::SlugTooLong {
                max: 200,
                actual: normalized.len(),
            });
        }

        if !normalized
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return Err(DomainError::InvalidSlugCharacters);
        }

        Ok(Self(normalized))
    }

    /// For reconstitution from persistence — assumes valid
    pub fn from_trusted(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
