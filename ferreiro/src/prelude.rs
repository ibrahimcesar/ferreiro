//! Prelude module - commonly used imports
//!
//! Import everything you need to get started:
//! ```rust
//! use ferreiro::prelude::*;
//! ```

// Domain exports
pub use ferreiro_domain::errors::DomainError;
pub use ferreiro_domain::events::DomainEvent;
pub use ferreiro_domain::models::{Post, PostStatus, User};
pub use ferreiro_domain::ports::driven::{
    EventPublisher, PaginatedResult, Pagination, PasswordHasher, PostFilter, PostRepository,
    RepositoryError, UserRepository,
};
pub use ferreiro_domain::ports::driving::{
    AuthService, CreatePostCommand, ListPostsQuery, PostService, RegisterCommand, ServiceError,
    UpdatePostCommand,
};
pub use ferreiro_domain::values::{Body, Email, PostId, Slug, Title, UserId};

// Application exports
pub use ferreiro_application::services::PostServiceImpl;

// Database adapters
pub use ferreiro_adapters_db::{InMemoryEventPublisher, InMemoryPostRepository};

// HTTP adapters
pub use ferreiro_adapters_http::serve;

// Template adapters
pub use ferreiro_adapters_templates::{context, Context, TemplateEngine, TemplateError};

// Session adapters
pub use ferreiro_adapters_session::{SessionData, SessionError, SessionId, SessionStore};

// Common external re-exports
pub use async_trait::async_trait;
pub use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
pub use serde::{Deserialize, Serialize};
pub use tokio;
