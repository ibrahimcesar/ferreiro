use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

pub type SessionId = String;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionData {
    pub data: HashMap<String, serde_json::Value>,
    pub modified: bool,
}

impl SessionData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.data
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    pub fn set<T: Serialize>(&mut self, key: &str, value: T) {
        if let Ok(v) = serde_json::to_value(value) {
            self.data.insert(key.to_string(), v);
            self.modified = true;
        }
    }

    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
        self.modified = true;
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.modified = true;
    }
}

#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn load(&self, id: &SessionId) -> Result<Option<SessionData>, SessionError>;
    async fn save(
        &self,
        id: Option<&SessionId>,
        data: &SessionData,
    ) -> Result<SessionId, SessionError>;
    async fn delete(&self, id: &SessionId) -> Result<(), SessionError>;
    async fn cleanup(&self) -> Result<usize, SessionError>;
}

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Session expired")]
    Expired,

    #[error("Invalid session")]
    Invalid,
}

pub mod cookie;
pub mod memory;
