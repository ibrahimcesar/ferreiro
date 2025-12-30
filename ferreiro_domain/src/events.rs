use crate::values::{PostId, UserId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum DomainEvent {
    PostCreated {
        post_id: PostId,
        author_id: UserId,
        occurred_at: DateTime<Utc>,
    },
    PostPublished {
        post_id: PostId,
        occurred_at: DateTime<Utc>,
    },
    PostArchived {
        post_id: PostId,
        occurred_at: DateTime<Utc>,
    },
    UserRegistered {
        user_id: UserId,
        email: String,
        occurred_at: DateTime<Utc>,
    },
}

impl DomainEvent {
    pub fn occurred_at(&self) -> DateTime<Utc> {
        match self {
            Self::PostCreated { occurred_at, .. } => *occurred_at,
            Self::PostPublished { occurred_at, .. } => *occurred_at,
            Self::PostArchived { occurred_at, .. } => *occurred_at,
            Self::UserRegistered { occurred_at, .. } => *occurred_at,
        }
    }
}
