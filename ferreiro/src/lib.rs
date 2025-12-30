//! # Ferreiro
//!
//! A Django-inspired web framework for Rust, built on hexagonal architecture.
//! **For developers who want to build, not configure.**
//!
//! ⚠️ **Alpha Release**: Ferreiro is in early development (v0.0.x). The core architecture
//! is solid, but many features are still being implemented.
//!
//! ## What Works Now
//!
//! - ✅ Domain modeling with value objects and events
//! - ✅ In-memory repositories for testing
//! - ✅ HTTP server (Axum-based)
//! - ✅ Template engines (Tera, MiniJinja)
//! - ✅ Session management
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use ferreiro::prelude::*;
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Setup repositories
//!     let repo = Arc::new(InMemoryPostRepository::new());
//!     let events = Arc::new(InMemoryEventPublisher::new());
//!
//!     // Create service
//!     let service = Arc::new(PostServiceImpl::new(repo, events));
//!
//!     // Create a post
//!     let post = service.create(CreatePostCommand {
//!         title: "Hello World".to_string(),
//!         slug: "hello-world".to_string(),
//!         body: "My first post".to_string(),
//!         author_id: UserId::generate(),
//!     }).await?;
//!
//!     // Build HTTP API
//!     let app = Router::new()
//!         .route("/", get(|| async { "Welcome to Ferreiro!" }));
//!
//!     // Start server
//!     serve(app, "127.0.0.1", 8000).await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! Ferreiro follows hexagonal (ports and adapters) architecture:
//!
//! - **Domain**: Pure business logic with zero framework dependencies
//! - **Ports**: Trait interfaces for repositories and services
//! - **Adapters**: Implementations for HTTP, databases, templates, etc.
//!
//! This keeps your domain clean and makes everything swappable and testable.
//!
//! ## Modules
//!
//! - [`domain`]: Domain models, value objects, and ports
//! - [`application`]: Service implementations and use cases
//! - [`db`]: Database adapters (currently in-memory, PostgreSQL/SQLite coming)
//! - [`http`]: HTTP server and routing
//! - [`templates`]: Template engines (Tera, MiniJinja)
//! - [`session`]: Session management
//! - [`admin`]: Admin interface (coming soon)
//! - [`prelude`]: Convenient imports for common use cases

pub mod prelude;

// Re-export all major modules
pub use ferreiro_adapters_admin as admin;
pub use ferreiro_adapters_db as db;
pub use ferreiro_adapters_http as http;
pub use ferreiro_adapters_session as session;
pub use ferreiro_adapters_templates as templates;
pub use ferreiro_application as application;
pub use ferreiro_domain as domain;

// Re-export common types
pub use ferreiro_domain::{errors::DomainError, events::DomainEvent};
