use crate::events::DomainEvent;
use crate::models::{Post, PostStatus, User};
use crate::values::{Email, PostId, Slug, UserId};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use thiserror::Error;

// ============= Repository Filters & Pagination =============

#[derive(Debug, Clone, Default)]
pub struct PostFilter {
    pub author_id: Option<UserId>,
    pub status: Option<PostStatus>,
    pub published_after: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub total_pages: usize,
}

// ============= Post Repository =============

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_by_id(&self, id: &PostId) -> Result<Option<Post>, RepositoryError>;
    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<Post>, RepositoryError>;
    async fn save(&self, post: &Post) -> Result<(), RepositoryError>;
    async fn delete(&self, id: &PostId) -> Result<(), RepositoryError>;
    async fn list(
        &self,
        filter: PostFilter,
        pagination: Pagination,
    ) -> Result<PaginatedResult<Post>, RepositoryError>;
    async fn exists_by_slug(&self, slug: &Slug) -> Result<bool, RepositoryError>;
}

// ============= User Repository =============

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, RepositoryError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, RepositoryError>;
    async fn save(&self, user: &User) -> Result<(), RepositoryError>;
    async fn delete(&self, id: &UserId) -> Result<(), RepositoryError>;
    async fn exists_by_email(&self, email: &Email) -> Result<bool, RepositoryError>;
}

// ============= Event Publisher =============

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: DomainEvent) -> Result<(), EventError>;
    async fn publish_all(&self, events: Vec<DomainEvent>) -> Result<(), EventError>;
}

// ============= Password Hasher =============

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, HashError>;
    fn verify(&self, password: &str, hash: &str) -> Result<bool, HashError>;
}

// ============= Errors =============

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Entity not found")]
    NotFound,

    #[error("Conflict occurred")]
    Conflict,

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Query error: {0}")]
    Query(String),
}

#[derive(Debug, Error)]
pub enum EventError {
    #[error("Failed to publish event: {0}")]
    PublishFailed(String),
}

#[derive(Debug, Error)]
pub enum HashError {
    #[error("Hashing failed: {0}")]
    HashingFailed(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(String),
}
