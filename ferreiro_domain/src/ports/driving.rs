use crate::errors::DomainError;
use crate::models::{Post, User};
use crate::ports::driven::{PaginatedResult, Pagination, PostFilter};
use crate::values::{PostId, UserId};
use async_trait::async_trait;
use thiserror::Error;

// ============= Post Service Commands =============

pub struct CreatePostCommand {
    pub title: String,
    pub slug: String,
    pub body: String,
    pub author_id: UserId,
}

pub struct UpdatePostCommand {
    pub id: PostId,
    pub title: String,
    pub body: String,
}

pub struct ListPostsQuery {
    pub filter: PostFilter,
    pub pagination: Pagination,
}

// ============= Post Service =============

#[async_trait]
pub trait PostService: Send + Sync {
    async fn create(&self, cmd: CreatePostCommand) -> Result<Post, ServiceError>;
    async fn update(&self, cmd: UpdatePostCommand) -> Result<Post, ServiceError>;
    async fn publish(&self, id: &PostId) -> Result<Post, ServiceError>;
    async fn archive(&self, id: &PostId) -> Result<Post, ServiceError>;
    async fn delete(&self, id: &PostId) -> Result<(), ServiceError>;
    async fn get(&self, id: &PostId) -> Result<Option<Post>, ServiceError>;
    async fn get_by_slug(&self, slug: &str) -> Result<Option<Post>, ServiceError>;
    async fn list(&self, query: ListPostsQuery) -> Result<PaginatedResult<Post>, ServiceError>;
}

// ============= Auth Service Commands =============

pub struct RegisterCommand {
    pub email: String,
    pub password: String,
    pub name: String,
}

pub struct LoginCommand {
    pub email: String,
    pub password: String,
}

pub struct AuthenticatedUser {
    pub user: User,
    pub session_token: String,
}

// ============= Auth Service =============

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn register(&self, cmd: RegisterCommand) -> Result<User, ServiceError>;
    async fn login(&self, cmd: LoginCommand) -> Result<AuthenticatedUser, ServiceError>;
    async fn logout(&self, session_token: &str) -> Result<(), ServiceError>;
    async fn get_user_by_session(&self, session_token: &str) -> Result<Option<User>, ServiceError>;
}

// ============= Service Errors =============

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Entity not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
