use crate::{SessionData, SessionError, SessionId, SessionStore};
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Cookie-based sessions — data stored in signed cookie
/// Good for: Small session data, stateless servers
/// Limits: ~4KB max
pub struct CookieSessionStore {
    secret_key: Vec<u8>,
    #[allow(dead_code)]
    max_age: std::time::Duration,
}

impl CookieSessionStore {
    pub fn new(secret_key: &[u8], max_age: std::time::Duration) -> Self {
        Self {
            secret_key: secret_key.to_vec(),
            max_age,
        }
    }

    fn sign(&self, data: &[u8]) -> String {
        let mut mac =
            HmacSha256::new_from_slice(&self.secret_key).expect("HMAC can take key of any size");
        mac.update(data);
        let signature = mac.finalize().into_bytes();
        BASE64.encode(signature)
    }

    fn verify(&self, data: &[u8], signature: &str) -> bool {
        let expected = self.sign(data);
        expected == signature
    }
}

#[async_trait]
impl SessionStore for CookieSessionStore {
    async fn load(&self, id: &SessionId) -> Result<Option<SessionData>, SessionError> {
        // ID is actually the signed data
        let parts: Vec<&str> = id.split('.').collect();
        if parts.len() != 2 {
            return Err(SessionError::Invalid);
        }

        let data_b64 = parts[0];
        let signature = parts[1];

        if !self.verify(data_b64.as_bytes(), signature) {
            return Err(SessionError::Invalid);
        }

        let data = BASE64
            .decode(data_b64)
            .map_err(|e| SessionError::Serialization(e.to_string()))?;

        let session_data: SessionData = serde_json::from_slice(&data)
            .map_err(|e| SessionError::Serialization(e.to_string()))?;

        Ok(Some(session_data))
    }

    async fn save(
        &self,
        _id: Option<&SessionId>,
        data: &SessionData,
    ) -> Result<SessionId, SessionError> {
        let json =
            serde_json::to_vec(data).map_err(|e| SessionError::Serialization(e.to_string()))?;
        let data_b64 = BASE64.encode(&json);
        let signature = self.sign(data_b64.as_bytes());
        Ok(format!("{}.{}", data_b64, signature))
    }

    async fn delete(&self, _id: &SessionId) -> Result<(), SessionError> {
        Ok(()) // Stateless — nothing to delete
    }

    async fn cleanup(&self) -> Result<usize, SessionError> {
        Ok(0) // Stateless — nothing to clean
    }
}
