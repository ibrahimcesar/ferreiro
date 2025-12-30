use crate::{SessionData, SessionError, SessionId, SessionStore};
use async_trait::async_trait;
use rand::Rng;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// In-memory session store for testing
pub struct MemorySessionStore {
    sessions: Arc<RwLock<HashMap<SessionId, SessionData>>>,
}

impl MemorySessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn generate_id() -> SessionId {
        let random_bytes: [u8; 32] = rand::thread_rng().gen();
        hex::encode(random_bytes)
    }
}

impl Default for MemorySessionStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SessionStore for MemorySessionStore {
    async fn load(&self, id: &SessionId) -> Result<Option<SessionData>, SessionError> {
        let sessions = self.sessions.read().unwrap();
        Ok(sessions.get(id).cloned())
    }

    async fn save(
        &self,
        id: Option<&SessionId>,
        data: &SessionData,
    ) -> Result<SessionId, SessionError> {
        let session_id = id.map(|s| s.to_string()).unwrap_or_else(Self::generate_id);
        let mut sessions = self.sessions.write().unwrap();
        sessions.insert(session_id.clone(), data.clone());
        Ok(session_id)
    }

    async fn delete(&self, id: &SessionId) -> Result<(), SessionError> {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(id);
        Ok(())
    }

    async fn cleanup(&self) -> Result<usize, SessionError> {
        // In-memory sessions don't expire in this simple implementation
        Ok(0)
    }
}
