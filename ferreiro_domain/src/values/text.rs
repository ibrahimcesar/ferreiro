use crate::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Title(String);

impl Title {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(DomainError::EmptyTitle);
        }

        if trimmed.len() > 200 {
            return Err(DomainError::TitleTooLong {
                max: 200,
                actual: trimmed.len(),
            });
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn from_trusted(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Body(String);

impl Body {
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }

    pub fn from_trusted(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.trim().is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
