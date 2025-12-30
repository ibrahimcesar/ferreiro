# Ferreiro

[![Crates.io](https://img.shields.io/crates/v/ferreiro.svg)](https://crates.io/crates/ferreiro)
[![Documentation](https://docs.rs/ferreiro/badge.svg)](https://docs.rs/ferreiro)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

A Django-inspired web framework for Rust, built on hexagonal architecture. **For developers who want to build, not configure.**

> ⚠️ **ALPHA RELEASE - Work in Progress**
>
> Ferreiro is in early development (v0.0.x). The architecture is solid and the foundation is ready, but many features described below are **planned** and not yet implemented. See the [Current Status](#current-status) section to understand what works today.
>
> **What works now:**
> - ✅ Domain modeling with value objects and events
> - ✅ In-memory repositories for testing
> - ✅ HTTP server (Axum-based)
> - ✅ Template engines (Tera, MiniJinja)
> - ✅ Session management (Cookie, Memory)
> - ✅ Hexagonal architecture foundations
>
> **Coming soon:**
> - 🚧 PostgreSQL/SQLite adapters
> - 🚧 Migration engine
> - 🚧 Admin interface
> - 🚧 CLI commands (`startproject`, `runserver`, etc.)
> - 🚧 Authentication service
> - 🚧 Background jobs
>
> We're actively developing and welcome early adopters and contributors!

## For Lazy Developers

Most Rust web frameworks make you assemble everything yourself. Ferreiro gives you what Django developers take for granted:

```bash
$ ferreiro startproject webapp
$ cd webapp
$ ferreiro runserver
```

That's it. You get:
- Auto-reloading dev server (templates AND code)
- Database with migrations
- Admin interface
- Session management
- Background tasks
- Authentication scaffolding
- Frontend integration (React, Svelte, Vue, or full-stack Rust)

No decision fatigue. No hunting for crates. Just build.

### What You Don't Have to Do

**Other Rust frameworks**: Pick a router (axum? actix? warp?), choose a database crate, find a migration tool, set up sessions, configure logging, add tracing, wire up background jobs, build an admin panel...

**Ferreiro**: `ferreiro startproject`. Done.

You're not lazy for wanting this. You're smart. You want to build features, not infrastructure.

### Your First Endpoint in 5 Minutes

```rust
// webapp/handlers.rs
use ferreiro::prelude::*;

#[derive(Model)]
pub struct Post {
    title: String,
    body: Text,
    status: PostStatus,
}

#[get("/posts/{id}")]
async fn post_detail(id: PostId) -> Result<Post> {
    Post::objects().get(id).await
}
```

That's it. No manual routing registration. No database setup. No JSON serialization boilerplate. Just write your handler.

Run `ferreiro makemigrations && ferreiro migrate`. Your database table exists. Visit `/admin`. Your CRUD interface exists. It just works.

## Philosophy

Django succeeded because it made the common cases trivial and the complex cases possible. Ferreiro brings this to Rust: **convention over configuration, sensible defaults, and a cohesive ecosystem where components work together out of the box.**

What sets Ferreiro apart:

- **Batteries included, not assembly required** — Everything you need to ship, included
- **Hexagonal architecture** — Your domain logic stays pure, framework concerns live in adapters
- **Swappable everything** — Start with SQLite, swap to Postgres later. Zero domain changes.
- **Magic in adapters, not domain** — Auto-admin and introspection happen outside your business logic
- **Fast feedback loops** — Hot reload for templates, background compilation, instant iterations

## Developer Experience

### Instant Feedback Loops

**Hot Module Replacement for Templates**: Not just reload — actually push template changes to the browser without losing state. Edit a template, see it update instantly without losing form data or navigation state.

**Auto-reloading Server**: Code changes trigger automatic recompilation and restart. No manual rebuilds, no shell scripts, no tmux windows.

```bash
$ ferreiro runserver --hot-reload
# Edit any file. See changes. Keep building.
```

### Type-Safe URL Reversing

```rust
// Compile-time checked URL generation
let url = routes::webapp::post_detail(post.id());  // Error if route doesn't exist
```

### Automatic OpenAPI/JSON Schema Generation

Derive from your domain types. No separate spec to maintain.

### Built-in REPL with Async Support

```bash
$ ferreiro shell
>>> let posts = Post::objects().filter(status: Published).all().await;
>>> posts.len()
42
```

### Background Tasks Without the Boilerplate

Most frameworks make you set up Redis, workers, and job queues. Ferreiro includes everything:

```rust
#[job(delay = "1h")]
async fn send_welcome_email(user_id: UserId) {
    let user = User::objects().get(user_id).await?;
    emails::send_welcome(&user).await?;
}

// Schedule from anywhere
send_welcome_email.schedule(user.id()).await?;
```

Transactional outbox included. Failed jobs go to dead letter queue. Retry logic built-in. You just write the function.

### Zero-Config Observability

Logging, metrics, and tracing are built-in, not bolted on:

```rust
// Every request automatically gets:
// - Request ID
// - User ID (if authenticated)
// - Tenant ID (if multi-tenant)
// - Execution time
// - Database query count

// In your domain code, just log normally:
log::info!("Post published");
// Output: [req:abc123][user:42][tenant:acme] Post published
```

OpenTelemetry traces track every port call. See exactly where time is spent without instrumenting every function.

### Frontend Integration

Ferreiro doesn't force you into server-side rendering. Use whatever frontend stack you want:

#### API-First Mode (React, Svelte, Vue, etc.)

```rust
// Automatically serializes to JSON
#[get("/api/posts")]
async fn list_posts() -> Result<Vec<Post>> {
    Post::objects().all().await
}

// Your React/Svelte/Vue app consumes the API
// OpenAPI spec auto-generated for type-safe clients
```

#### Vite Integration

```bash
$ ferreiro add vite --framework react
# or --framework svelte, --framework vue
```

This sets up:
- Vite dev server proxy during development
- Hot module replacement for your frontend
- Production asset bundling
- Automatic static file serving

Your `ferreiro runserver` starts both backend and Vite simultaneously. One command, full-stack development.

#### Static Asset Serving

```rust
// In settings.rs
pub struct StaticFiles {
    pub dirs: Vec<&'static str>,
    pub url_prefix: &'static str,
}

// Serves /static/* from ./static and ./dist
// Vite builds to ./dist automatically
```

#### Server-Side Rendering (Traditional)

Templates work great for server-rendered apps:

```rust
#[get("/posts/{id}")]
async fn post_detail(id: PostId) -> Result<Template> {
    let post = Post::objects().get(id).await?;
    Template::render("posts/detail.html", context! { post })
}
```

Mix and match approaches. Server-render your marketing pages, use React for your dashboard. It's your app.

#### Full-Stack Rust (Leptos, Dioxus)

Want Rust on the frontend too?

```bash
$ ferreiro add leptos
# or ferreiro add dioxus
```

Ferreiro handles the build pipeline, serves your WASM, and provides API endpoints your frontend can call. Your domain types work on both client and server with zero duplication.

### Domain-Driven Features

#### Aggregate Root Enforcement

The framework understands aggregate boundaries. Repositories only load/save aggregates, never entities directly. Enforced at the type level.

#### Event Sourcing as an Adapter

Swap your repository adapter to store events instead of state. Same domain code, different persistence strategy.

```rust
// settings.rs
let post_repo: Arc<dyn PostRepository> = match settings.persistence {
    "state" => Arc::new(PostgresPostRepository::new(db)),
    "events" => Arc::new(EventSourcedPostRepository::new(event_store)),
};
```

#### Saga/Process Manager Support

Long-running business processes that react to domain events and coordinate across aggregates.

### The Admin That Actually Helps

#### Admin Actions as Domain Commands

Admin actions map directly to your application layer commands. No special admin logic.

```rust
impl ModelAdmin for PostAdmin {
    fn actions(&self) -> Vec<AdminAction> {
        vec![
            AdminAction::new("publish", |ids| {
                // This calls your actual PostService::publish
                for id in ids {
                    post_service.publish(&id).await?;
                }
            }),
        ]
    }
}
```

#### Audit Log Built-in

Every admin action creates a domain event. Full history of who changed what.

#### Field-Level Permissions

Not just model-level. Marketing can edit title, only editors can touch status.

### Query Layer

#### CQRS-Ready Read Models

Separate read-optimized projections that update from domain events.

```rust
#[derive(ReadModel)]
#[updates_from(PostPublished, PostArchived)]
pub struct PostListingView {
    pub id: PostId,
    pub title: String,
    pub author_name: String,  // Denormalized
    pub published_at: DateTime<Utc>,
}
```

#### GraphQL Adapter

Same domain, different delivery mechanism. Automatically derived from your ports.

#### Full-Text Search Adapter

Pluggable. Meilisearch, Elasticsearch, or PostgreSQL's built-in. Indexes stay in sync via domain events.

### Background Processing

#### Transactional Outbox

Events are persisted in the same transaction as state changes. A separate process reliably delivers them. No lost events.

#### Delayed Jobs from Domain Events

```rust
// When PostCreated fires, schedule a reminder
#[job(delay = "24h", on = PostCreated)]
async fn send_engagement_reminder(event: PostCreated) {
    // Check if author has engaged with comments
}
```

#### Dead Letter Queue with Replay

Failed jobs don't disappear. Inspect, fix, replay.

### Multi-tenancy

#### Tenant Isolation at the Port Level

```rust
pub trait PostRepository: Send + Sync {
    async fn find_by_id(&self, tenant: &TenantId, id: &PostId) -> Result<Option<Post>>;
}
```

The adapter handles whether that's row-level filtering, schema separation, or separate databases.

#### Tenant-Aware Middleware

Extracts tenant from subdomain, header, or JWT. Available everywhere via request extensions.

### Observability

#### Structured Logging with Context Propagation

Request ID, user ID, tenant ID automatically attached to every log line.

#### OpenTelemetry Traces

Each port call is a span. See exactly where time is spent across adapters.

#### Health Checks as Adapters

```rust
pub trait HealthCheck: Send + Sync {
    async fn check(&self) -> HealthStatus;
}

// Each adapter implements it
impl HealthCheck for PostgresBackend { /* ... */ }
impl HealthCheck for RedisSessionStore { /* ... */ }
```

### Testing

#### In-Memory Adapters for Everything

Every driven port has an in-memory implementation. Tests run fast, no containers needed.

```rust
#[test]
async fn test_publish_post() {
    let repo = InMemoryPostRepository::new();
    let events = InMemoryEventPublisher::new();
    let service = PostServiceImpl::new(repo, events);

    // Test domain logic without any infrastructure
}
```

#### Scenario Testing DSL

```rust
scenario!("User publishes a draft post")
    .given(a_draft_post().with_body("content"))
    .when(user_publishes_the_post())
    .then(post_status_is(Published))
    .and(event_was_published(PostPublished));
```

#### Contract Testing for Adapters

Verify your PostgreSQL adapter behaves identically to SQLite adapter.

### Security

#### RBAC/ABAC at the Port Level

Permissions checked before entering the application layer.

```rust
#[requires(Permission::PublishPost)]
async fn publish(&self, id: &PostId) -> Result<Post, ServiceError>;
```

#### Automatic CSRF for State-Changing Operations

Not just forms. API calls from browsers too.

#### Rate Limiting as Middleware

Per-user, per-IP, per-tenant. Configurable per-route.

### Deployment

#### Single Binary with Embedded Migrations

```bash
$ ferreiro build --release
$ ./myapp migrate && ./myapp serve
```

#### Feature Flags as Configuration

```rust
if features.is_enabled("new_checkout", user.id()) {
    // New flow
}
```

Adapters for LaunchDarkly, Unleash, or simple JSON file.

#### Zero-Downtime Migrations

Framework guides you toward backward-compatible schema changes. Warns when a migration would require downtime.

## Architecture Overview

```
                         ┌─────────────────────────────────────────────┐
                         │              Adapters (driving)             │
                         │  ┌─────────┐ ┌─────────┐ ┌───────────────┐  │
                         │  │  HTTP   │ │   CLI   │ │    GraphQL    │  │
                         │  └────┬────┘ └────┬────┘ └──────┬────────┘  │
                         │       │           │             │           │
                         │       ▼           ▼             ▼           │
                         │  ┌─────────────────────────────────────┐    │
                         │  │          Ports (driving)            │    │
                         │  │     trait PostService               │    │
                         │  │     trait UserService               │    │
                         │  │     trait AuthService               │    │
                         │  └───────────────┬─────────────────────┘    │
                         │                  │                          │
                         │                  ▼                          │
                         │  ┌─────────────────────────────────────┐    │
                         │  │             Domain                  │    │
                         │  │                                     │    │
                         │  │   Models: Post, User, Comment       │    │
                         │  │   Values: Email, Slug, Password     │    │
                         │  │   Rules: validation, state machines │    │
                         │  │                                     │    │
                         │  │   (Pure Rust, no dependencies)      │    │
                         │  └───────────────┬─────────────────────┘    │
                         │                  │                          │
                         │                  ▼                          │
                         │  ┌─────────────────────────────────────┐    │
                         │  │          Ports (driven)             │    │
                         │  │     trait PostRepository            │    │
                         │  │     trait SessionStore              │    │
                         │  │     trait TemplateEngine            │    │
                         │  │     trait EmailSender               │    │
                         │  └───┬─────────┬─────────┬─────────┬───┘    │
                         │      │         │         │         │        │
                         │      ▼         ▼         ▼         ▼        │
                         │ ┌────────┐┌────────┐┌────────┐┌────────┐    │
                         │ │Postgres││  Tera  ││ Cookie ││  SMTP  │    │
                         │ │ SQLite ││Minijnja││ Redis  ││Sendgrid│    │
                         │ │        ││        ││Database││        │    │
                         │ └────────┘└────────┘└────────┘└────────┘    │
                         │              Adapters (driven)              │
                         └─────────────────────────────────────────────┘
```

## Project Structure

```
ferreiro/
├── ferreiro_domain/              # Pure Rust, zero dependencies
│   ├── models/                   # Aggregates and entities
│   │   ├── post.rs
│   │   ├── user.rs
│   │   └── comment.rs
│   ├── values/                   # Value objects
│   │   ├── email.rs
│   │   ├── slug.rs
│   │   ├── password.rs
│   │   └── pagination.rs
│   ├── events.rs                 # Domain events
│   ├── errors.rs                 # Domain errors
│   └── ports/
│       ├── mod.rs
│       ├── driving.rs            # Inbound ports (services)
│       └── driven.rs             # Outbound ports (repositories, etc.)
│
├── ferreiro_application/         # Use cases, orchestration
│   ├── services/
│   │   ├── post_service.rs
│   │   ├── user_service.rs
│   │   └── auth_service.rs
│   ├── commands/                 # Write operations
│   └── queries/                  # Read operations
│
├── ferreiro_adapters_db/         # Database implementations
│   ├── postgres/
│   │   ├── mod.rs
│   │   ├── post_repository.rs
│   │   ├── user_repository.rs
│   │   └── session_store.rs
│   ├── sqlite/
│   │   ├── mod.rs
│   │   └── post_repository.rs
│   ├── schema.rs                 # Shared schema introspection
│   ├── migrations.rs             # Migration engine
│   └── backend.rs                # DatabaseBackend trait
│
├── ferreiro_adapters_http/       # HTTP layer
│   ├── server.rs
│   ├── routing.rs
│   ├── middleware/
│   │   ├── session.rs
│   │   ├── auth.rs
│   │   ├── csrf.rs
│   │   └── logging.rs
│   ├── handlers/                 # Thin translation layer
│   └── extractors.rs
│
├── ferreiro_adapters_templates/  # Template engines
│   ├── port.rs                   # TemplateEngine trait
│   ├── tera.rs                   # Tera implementation (default)
│   └── minijinja.rs              # Minijinja implementation
│
├── ferreiro_adapters_session/    # Session storage
│   ├── port.rs                   # SessionStore trait
│   ├── cookie.rs                 # Signed cookie sessions
│   ├── database.rs               # Database-backed sessions
│   └── redis.rs                  # Redis sessions
│
├── ferreiro_adapters_email/      # Email sending
│   ├── port.rs
│   ├── smtp.rs
│   └── sendgrid.rs
│
├── ferreiro_adapters_admin/      # Auto-generated admin
│   ├── introspection.rs          # Reads model metadata via traits
│   ├── views.rs                  # Generated CRUD views
│   ├── widgets.rs                # Form widgets
│   └── templates/                # Admin UI templates
│
├── ferreiro_cli/                 # Management commands
│   ├── commands/
│   │   ├── startproject.rs
│   │   ├── startapp.rs
│   │   ├── runserver.rs
│   │   ├── migrate.rs
│   │   ├── makemigrations.rs
│   │   └── createsuperuser.rs
│   └── main.rs
│
└── ferreiro/                     # Umbrella crate
    ├── prelude.rs                # Common imports
    └── lib.rs                    # Re-exports
```

---

## Domain Layer

The domain is pure Rust with no external dependencies. It knows nothing about HTTP, databases, or templates.

### Models

```rust
// ferreiro_domain/models/post.rs

use crate::values::{PostId, Title, Slug, Body, UserId};
use crate::errors::DomainError;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Post {
    id: PostId,
    title: Title,
    slug: Slug,
    body: Body,
    author_id: UserId,
    status: PostStatus,
    created_at: DateTime<Utc>,
    published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostStatus {
    Draft,
    Published,
    Archived,
}

impl Post {
    pub fn new(title: Title, slug: Slug, body: Body, author_id: UserId) -> Self {
        Self {
            id: PostId::generate(),
            title,
            slug,
            body,
            author_id,
            status: PostStatus::Draft,
            created_at: Utc::now(),
            published_at: None,
        }
    }

    /// Reconstitute from persistence — no validation, no events
    pub fn reconstitute(
        id: PostId,
        title: Title,
        slug: Slug,
        body: Body,
        author_id: UserId,
        status: PostStatus,
        created_at: DateTime<Utc>,
        published_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self { id, title, slug, body, author_id, status, created_at, published_at }
    }

    pub fn publish(&mut self) -> Result<(), DomainError> {
        if self.body.is_empty() {
            return Err(DomainError::CannotPublishEmptyPost);
        }
        if self.status == PostStatus::Published {
            return Err(DomainError::AlreadyPublished);
        }
        self.status = PostStatus::Published;
        self.published_at = Some(Utc::now());
        Ok(())
    }

    pub fn archive(&mut self) {
        self.status = PostStatus::Archived;
    }

    pub fn update_content(&mut self, title: Title, body: Body) {
        self.title = title;
        self.body = body;
    }

    // Getters
    pub fn id(&self) -> &PostId { &self.id }
    pub fn title(&self) -> &Title { &self.title }
    pub fn slug(&self) -> &Slug { &self.slug }
    pub fn body(&self) -> &Body { &self.body }
    pub fn author_id(&self) -> &UserId { &self.author_id }
    pub fn status(&self) -> &PostStatus { &self.status }
    pub fn created_at(&self) -> DateTime<Utc> { self.created_at }
    pub fn published_at(&self) -> Option<DateTime<Utc>> { self.published_at }
    pub fn is_published(&self) -> bool { self.status == PostStatus::Published }
}
```

### Value Objects

```rust
// ferreiro_domain/values/slug.rs

use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Slug(String);

impl Slug {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        let normalized = value.trim().to_lowercase();
        
        if normalized.is_empty() {
            return Err(DomainError::EmptySlug);
        }
        
        if normalized.len() > 200 {
            return Err(DomainError::SlugTooLong { max: 200, actual: normalized.len() });
        }
        
        if !normalized.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
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
```

```rust
// ferreiro_domain/values/email.rs

use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        let normalized = value.trim().to_lowercase();
        
        if !normalized.contains('@') || !normalized.contains('.') {
            return Err(DomainError::InvalidEmail);
        }
        
        Ok(Self(normalized))
    }

    pub fn from_trusted(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }
}
```

### Domain Errors

```rust
// ferreiro_domain/errors.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    // Post
    CannotPublishEmptyPost,
    AlreadyPublished,
    
    // Slug
    EmptySlug,
    SlugTooLong { max: usize, actual: usize },
    InvalidSlugCharacters,
    
    // Email
    InvalidEmail,
    
    // Title
    EmptyTitle,
    TitleTooLong { max: usize, actual: usize },
    
    // Password
    PasswordTooShort { min: usize },
    PasswordTooWeak,
    
    // User
    UserAlreadyExists,
    InvalidCredentials,
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CannotPublishEmptyPost => write!(f, "Cannot publish a post with empty body"),
            Self::AlreadyPublished => write!(f, "Post is already published"),
            Self::EmptySlug => write!(f, "Slug cannot be empty"),
            Self::SlugTooLong { max, actual } => write!(f, "Slug too long: {} chars (max {})", actual, max),
            Self::InvalidSlugCharacters => write!(f, "Slug can only contain letters, numbers, and hyphens"),
            Self::InvalidEmail => write!(f, "Invalid email address"),
            Self::EmptyTitle => write!(f, "Title cannot be empty"),
            Self::TitleTooLong { max, actual } => write!(f, "Title too long: {} chars (max {})", actual, max),
            Self::PasswordTooShort { min } => write!(f, "Password must be at least {} characters", min),
            Self::PasswordTooWeak => write!(f, "Password is too weak"),
            Self::UserAlreadyExists => write!(f, "User already exists"),
            Self::InvalidCredentials => write!(f, "Invalid credentials"),
        }
    }
}

impl std::error::Error for DomainError {}
```

### Domain Events

```rust
// ferreiro_domain/events.rs

use crate::values::{PostId, UserId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum DomainEvent {
    PostCreated { post_id: PostId, author_id: UserId, occurred_at: DateTime<Utc> },
    PostPublished { post_id: PostId, occurred_at: DateTime<Utc> },
    PostArchived { post_id: PostId, occurred_at: DateTime<Utc> },
    UserRegistered { user_id: UserId, email: String, occurred_at: DateTime<Utc> },
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
```

---

## Ports

Ports define the boundaries between domain and adapters.

### Driven Ports (Outbound)

```rust
// ferreiro_domain/ports/driven.rs

use crate::models::{Post, User, PostStatus};
use crate::values::{PostId, UserId, Slug, Email};
use crate::events::DomainEvent;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

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
    fn default() -> Self { Self { page: 1, per_page: 20 } }
}

#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub total_pages: usize,
}

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_by_id(&self, id: &PostId) -> Result<Option<Post>, RepositoryError>;
    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<Post>, RepositoryError>;
    async fn save(&self, post: &Post) -> Result<(), RepositoryError>;
    async fn delete(&self, id: &PostId) -> Result<(), RepositoryError>;
    async fn list(&self, filter: PostFilter, pagination: Pagination) -> Result<PaginatedResult<Post>, RepositoryError>;
    async fn exists_by_slug(&self, slug: &Slug) -> Result<bool, RepositoryError>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, RepositoryError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, RepositoryError>;
    async fn save(&self, user: &User) -> Result<(), RepositoryError>;
    async fn delete(&self, id: &UserId) -> Result<(), RepositoryError>;
    async fn exists_by_email(&self, email: &Email) -> Result<bool, RepositoryError>;
}

#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    Conflict,
    Connection(String),
    Query(String),
}

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: DomainEvent) -> Result<(), EventError>;
    async fn publish_all(&self, events: Vec<DomainEvent>) -> Result<(), EventError>;
}

#[derive(Debug)]
pub enum EventError {
    PublishFailed(String),
}

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, HashError>;
    fn verify(&self, password: &str, hash: &str) -> Result<bool, HashError>;
}

#[derive(Debug)]
pub enum HashError {
    HashingFailed(String),
    VerificationFailed(String),
}
```

### Driving Ports (Inbound)

```rust
// ferreiro_domain/ports/driving.rs

use crate::models::{Post, User};
use crate::values::{PostId, UserId};
use crate::errors::DomainError;
use crate::ports::driven::{Pagination, PaginatedResult, PostFilter};
use async_trait::async_trait;

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

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn register(&self, cmd: RegisterCommand) -> Result<User, ServiceError>;
    async fn login(&self, cmd: LoginCommand) -> Result<AuthenticatedUser, ServiceError>;
    async fn logout(&self, session_token: &str) -> Result<(), ServiceError>;
    async fn get_user_by_session(&self, session_token: &str) -> Result<Option<User>, ServiceError>;
}

#[derive(Debug)]
pub enum ServiceError {
    Domain(DomainError),
    NotFound,
    Unauthorized,
    Conflict(String),
    Internal(String),
}

impl From<DomainError> for ServiceError {
    fn from(err: DomainError) -> Self { Self::Domain(err) }
}
```

---

## Application Layer

The application layer implements driving ports and orchestrates domain operations.

```rust
// ferreiro_application/services/post_service.rs

use ferreiro_domain::models::Post;
use ferreiro_domain::values::{Title, Slug, Body, PostId};
use ferreiro_domain::events::DomainEvent;
use ferreiro_domain::ports::driving::{PostService, CreatePostCommand, ServiceError};
use ferreiro_domain::ports::driven::{PostRepository, EventPublisher};
use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;

pub struct PostServiceImpl<R, E>
where
    R: PostRepository,
    E: EventPublisher,
{
    post_repo: Arc<R>,
    events: Arc<E>,
}

impl<R, E> PostServiceImpl<R, E>
where
    R: PostRepository,
    E: EventPublisher,
{
    pub fn new(post_repo: Arc<R>, events: Arc<E>) -> Self {
        Self { post_repo, events }
    }
}

#[async_trait]
impl<R, E> PostService for PostServiceImpl<R, E>
where
    R: PostRepository + 'static,
    E: EventPublisher + 'static,
{
    async fn create(&self, cmd: CreatePostCommand) -> Result<Post, ServiceError> {
        let title = Title::new(&cmd.title)?;
        let slug = Slug::new(&cmd.slug)?;
        let body = Body::new(&cmd.body);

        if self.post_repo.exists_by_slug(&slug).await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))? 
        {
            return Err(ServiceError::Conflict("Slug already exists".into()));
        }

        let post = Post::new(title, slug, body, cmd.author_id.clone());

        self.post_repo.save(&post).await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))?;

        self.events.publish(DomainEvent::PostCreated {
            post_id: post.id().clone(),
            author_id: cmd.author_id,
            occurred_at: Utc::now(),
        }).await.ok();

        Ok(post)
    }

    async fn publish(&self, id: &PostId) -> Result<Post, ServiceError> {
        let mut post = self.post_repo.find_by_id(id).await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))?
            .ok_or(ServiceError::NotFound)?;

        post.publish()?;

        self.post_repo.save(&post).await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))?;

        self.events.publish(DomainEvent::PostPublished {
            post_id: id.clone(),
            occurred_at: Utc::now(),
        }).await.ok();

        Ok(post)
    }

    // ... other methods follow same pattern
}
```

---

## Database Adapters

### Database Backend Trait

```rust
// ferreiro_adapters_db/backend.rs

use async_trait::async_trait;

#[async_trait]
pub trait DatabaseBackend: Send + Sync {
    async fn execute(&self, sql: &str, params: &[Value]) -> Result<u64, DbError>;
    async fn query(&self, sql: &str, params: &[Value]) -> Result<Vec<Row>, DbError>;
    async fn query_one(&self, sql: &str, params: &[Value]) -> Result<Option<Row>, DbError>;
    async fn begin(&self) -> Result<Box<dyn Transaction>, DbError>;
    async fn introspect_tables(&self) -> Result<Vec<TableInfo>, DbError>;
    
    // SQL dialect
    fn quote_identifier(&self, name: &str) -> String;
    fn placeholder(&self, index: usize) -> String;
    fn type_for(&self, field_type: FieldType) -> &'static str;
    fn supports_returning(&self) -> bool;
}

#[derive(Debug, Clone)]
pub enum FieldType {
    Boolean,
    SmallInt,
    Integer,
    BigInt,
    Float,
    Double,
    Text,
    Varchar(usize),
    Bytea,
    Timestamp,
    Uuid,
    Json,
}
```

### PostgreSQL Backend

```rust
// ferreiro_adapters_db/postgres/mod.rs

pub struct PostgresBackend {
    pool: PgPool,
}

impl PostgresBackend {
    pub async fn connect(url: &str) -> Result<Self, DbError> {
        let pool = PgPool::connect(url).await
            .map_err(|e| DbError::Connection(e.to_string()))?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl DatabaseBackend for PostgresBackend {
    fn placeholder(&self, index: usize) -> String {
        format!("${}", index)
    }
    
    fn type_for(&self, field_type: FieldType) -> &'static str {
        match field_type {
            FieldType::Boolean => "BOOLEAN",
            FieldType::Integer => "INTEGER",
            FieldType::Text => "TEXT",
            FieldType::Timestamp => "TIMESTAMP WITH TIME ZONE",
            FieldType::Uuid => "UUID",
            _ => "TEXT",
        }
    }
    
    fn supports_returning(&self) -> bool { true }
    
    // ... other methods
}
```

### SQLite Backend

```rust
// ferreiro_adapters_db/sqlite/mod.rs

pub struct SqliteBackend {
    pool: SqlitePool,
}

impl SqliteBackend {
    pub async fn connect(url: &str) -> Result<Self, DbError> {
        let pool = SqlitePool::connect(url).await
            .map_err(|e| DbError::Connection(e.to_string()))?;
        Ok(Self { pool })
    }
    
    pub async fn in_memory() -> Result<Self, DbError> {
        Self::connect(":memory:").await
    }
}

#[async_trait]
impl DatabaseBackend for SqliteBackend {
    fn placeholder(&self, _index: usize) -> String {
        "?".to_string()
    }
    
    fn type_for(&self, field_type: FieldType) -> &'static str {
        match field_type {
            FieldType::Boolean => "INTEGER",  // 0 or 1
            FieldType::Integer => "INTEGER",
            FieldType::Text => "TEXT",
            FieldType::Timestamp => "TEXT",   // ISO 8601
            FieldType::Uuid => "TEXT",
            _ => "TEXT",
        }
    }
    
    fn supports_returning(&self) -> bool { true }  // SQLite 3.35+
    
    // ... other methods
}
```

---

## Session Adapters

### Session Store Port

```rust
// ferreiro_adapters_session/port.rs

use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;

pub type SessionId = String;

#[derive(Debug, Clone, Default)]
pub struct SessionData {
    data: HashMap<String, serde_json::Value>,
    modified: bool,
}

impl SessionData {
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.data.get(key).and_then(|v| serde_json::from_value(v.clone()).ok())
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
}

#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn load(&self, id: &SessionId) -> Result<Option<SessionData>, SessionError>;
    async fn save(&self, id: Option<&SessionId>, data: &SessionData) -> Result<SessionId, SessionError>;
    async fn delete(&self, id: &SessionId) -> Result<(), SessionError>;
    async fn cleanup(&self) -> Result<usize, SessionError>;
}

#[derive(Debug)]
pub enum SessionError {
    Serialization(String),
    Storage(String),
    Expired,
    Invalid,
}
```

### Cookie Session Store

```rust
// ferreiro_adapters_session/cookie.rs

/// Cookie-based sessions — data stored in signed cookie
/// Good for: Small session data, stateless servers
/// Limits: ~4KB max
pub struct CookieSessionStore {
    secret_key: Vec<u8>,
    max_age: std::time::Duration,
}

impl CookieSessionStore {
    pub fn new(secret_key: &[u8], max_age: std::time::Duration) -> Self {
        Self { secret_key: secret_key.to_vec(), max_age }
    }
}

#[async_trait]
impl SessionStore for CookieSessionStore {
    async fn load(&self, id: &SessionId) -> Result<Option<SessionData>, SessionError> {
        // Decode and verify HMAC signature from cookie value
        // ...
    }

    async fn save(&self, _id: Option<&SessionId>, data: &SessionData) -> Result<SessionId, SessionError> {
        // Encode + sign, return as "ID" — middleware writes to cookie
        // ...
    }

    async fn delete(&self, _id: &SessionId) -> Result<(), SessionError> {
        Ok(()) // Middleware clears cookie
    }

    async fn cleanup(&self) -> Result<usize, SessionError> {
        Ok(0) // Stateless — nothing to clean
    }
}
```

### Database Session Store

```rust
// ferreiro_adapters_session/database.rs

/// Database-backed sessions
/// Good for: Larger data, server-side invalidation, multi-instance
pub struct DatabaseSessionStore<DB: DatabaseBackend> {
    db: Arc<DB>,
    table: String,
    max_age: chrono::Duration,
}

impl<DB: DatabaseBackend> DatabaseSessionStore<DB> {
    pub fn new(db: Arc<DB>, max_age: chrono::Duration) -> Self {
        Self { db, table: "ferreiro_sessions".to_string(), max_age }
    }

    pub async fn initialize(&self) -> Result<(), SessionError> {
        // CREATE TABLE IF NOT EXISTS ferreiro_sessions ...
    }
}

#[async_trait]
impl<DB: DatabaseBackend + 'static> SessionStore for DatabaseSessionStore<DB> {
    async fn load(&self, id: &SessionId) -> Result<Option<SessionData>, SessionError> {
        // SELECT FROM sessions WHERE id = ? AND expires_at > NOW()
    }

    async fn save(&self, id: Option<&SessionId>, data: &SessionData) -> Result<SessionId, SessionError> {
        // INSERT ... ON CONFLICT UPDATE
    }

    async fn delete(&self, id: &SessionId) -> Result<(), SessionError> {
        // DELETE FROM sessions WHERE id = ?
    }

    async fn cleanup(&self) -> Result<usize, SessionError> {
        // DELETE FROM sessions WHERE expires_at < NOW()
    }
}
```

### Redis Session Store

```rust
// ferreiro_adapters_session/redis.rs

/// Redis-backed sessions
/// Good for: High performance, distributed systems
pub struct RedisSessionStore {
    client: redis::Client,
    prefix: String,
    max_age_secs: usize,
}

impl RedisSessionStore {
    pub fn new(url: &str, max_age_secs: usize) -> Result<Self, SessionError> {
        let client = redis::Client::open(url)
            .map_err(|e| SessionError::Storage(e.to_string()))?;
        Ok(Self { client, prefix: "ferreiro:session:".to_string(), max_age_secs })
    }
}

#[async_trait]
impl SessionStore for RedisSessionStore {
    async fn load(&self, id: &SessionId) -> Result<Option<SessionData>, SessionError> {
        // GET ferreiro:session:{id}
    }

    async fn save(&self, id: Option<&SessionId>, data: &SessionData) -> Result<SessionId, SessionError> {
        // SETEX ferreiro:session:{id} max_age data
    }

    async fn delete(&self, id: &SessionId) -> Result<(), SessionError> {
        // DEL ferreiro:session:{id}
    }

    async fn cleanup(&self) -> Result<usize, SessionError> {
        Ok(0) // Redis TTL handles expiration automatically
    }
}
```

---

## Template Adapters

### Template Engine Port

```rust
// ferreiro_adapters_templates/port.rs

pub trait TemplateEngine: Send + Sync {
    fn render(&self, name: &str, context: &Context) -> Result<String, TemplateError>;
    fn render_string(&self, template: &str, context: &Context) -> Result<String, TemplateError>;
    fn register_filter(&mut self, name: &str, filter: Box<dyn Filter>);
    fn register_function(&mut self, name: &str, func: Box<dyn Function>);
    fn reload(&mut self) -> Result<(), TemplateError>;
}

#[derive(Debug, Default)]
pub struct Context {
    data: HashMap<String, serde_json::Value>,
}

impl Context {
    pub fn new() -> Self { Self::default() }
    
    pub fn insert<T: Serialize>(&mut self, key: &str, value: T) {
        if let Ok(v) = serde_json::to_value(value) {
            self.data.insert(key.to_string(), v);
        }
    }
}

#[macro_export]
macro_rules! context {
    ($($key:ident : $value:expr),* $(,)?) => {{
        let mut ctx = Context::new();
        $(ctx.insert(stringify!($key), $value);)*
        ctx
    }};
}

#[derive(Debug)]
pub enum TemplateError {
    NotFound(String),
    Parse(String),
    Render(String),
}
```

### Tera Adapter (Default)

```rust
// ferreiro_adapters_templates/tera.rs

pub struct TeraEngine {
    tera: RwLock<tera::Tera>,
}

impl TeraEngine {
    pub fn new(template_dir: &str) -> Result<Self, TemplateError> {
        let glob = format!("{}/**/*", template_dir);
        let tera = tera::Tera::new(&glob)
            .map_err(|e| TemplateError::Parse(e.to_string()))?;
        Ok(Self { tera: RwLock::new(tera) })
    }
}

impl TemplateEngine for TeraEngine {
    fn render(&self, name: &str, context: &Context) -> Result<String, TemplateError> {
        let tera = self.tera.read().unwrap();
        tera.render(name, &to_tera_context(context))
            .map_err(|e| TemplateError::Render(e.to_string()))
    }
    
    fn reload(&mut self) -> Result<(), TemplateError> {
        self.tera.write().unwrap().full_reload()
            .map_err(|e| TemplateError::Parse(e.to_string()))
    }
    
    // ...
}
```

### MiniJinja Adapter (Django-like syntax)

```rust
// ferreiro_adapters_templates/minijinja.rs

/// MiniJinja has syntax closer to Django's template language
pub struct MiniJinjaEngine {
    env: RwLock<minijinja::Environment<'static>>,
}

impl MiniJinjaEngine {
    pub fn new(template_dir: &str) -> Result<Self, TemplateError> {
        let mut env = minijinja::Environment::new();
        env.set_loader(minijinja::path_loader(template_dir));
        Ok(Self { env: RwLock::new(env) })
    }
}

impl TemplateEngine for MiniJinjaEngine {
    fn render(&self, name: &str, context: &Context) -> Result<String, TemplateError> {
        let env = self.env.read().unwrap();
        let template = env.get_template(name)
            .map_err(|e| TemplateError::NotFound(e.to_string()))?;
        template.render(&context.data)
            .map_err(|e| TemplateError::Render(e.to_string()))
    }
    
    // ...
}
```

---

## Admin Adapter

Magic lives here, not in the domain. The admin introspects domain models via traits.

### Admin Introspection Traits

```rust
// ferreiro_adapters_admin/introspection.rs

/// Implement to expose a domain model to the admin
/// This trait lives in the ADAPTER layer, not the domain
pub trait AdminModel: Send + Sync {
    fn name(&self) -> &'static str;
    fn name_plural(&self) -> &'static str;
    fn fields(&self) -> Vec<AdminField>;
    fn primary_key(&self) -> &'static str;
    fn display(&self, instance: &dyn std::any::Any) -> String;
}

#[derive(Debug, Clone)]
pub struct AdminField {
    pub name: &'static str,
    pub display_name: String,
    pub field_type: AdminFieldType,
    pub required: bool,
    pub editable: bool,
}

#[derive(Debug, Clone)]
pub enum AdminFieldType {
    String { max_length: Option<usize> },
    Text,
    Integer,
    Boolean,
    DateTime,
    ForeignKey { model: &'static str },
    Enum { variants: Vec<String> },
}

/// Configuration for how a model appears in admin
pub trait ModelAdmin: Send + Sync {
    type Model: AdminModel;
    
    fn list_display(&self) -> Vec<&'static str> { vec![] }
    fn list_filter(&self) -> Vec<&'static str> { vec![] }
    fn search_fields(&self) -> Vec<&'static str> { vec![] }
    fn readonly_fields(&self) -> Vec<&'static str> { vec![] }
    fn ordering(&self) -> Vec<&'static str> { vec!["-created_at"] }
}
```

### Example: Exposing Post to Admin

```rust
// In your application wiring, NOT in domain

pub struct PostAdminModel;

impl AdminModel for PostAdminModel {
    fn name(&self) -> &'static str { "Post" }
    fn name_plural(&self) -> &'static str { "Posts" }
    
    fn fields(&self) -> Vec<AdminField> {
        vec![
            AdminField {
                name: "title",
                display_name: "Title".into(),
                field_type: AdminFieldType::String { max_length: Some(200) },
                required: true,
                editable: true,
            },
            AdminField {
                name: "status",
                display_name: "Status".into(),
                field_type: AdminFieldType::Enum {
                    variants: vec!["Draft".into(), "Published".into(), "Archived".into()],
                },
                required: true,
                editable: true,
            },
        ]
    }
    
    fn primary_key(&self) -> &'static str { "id" }
    
    fn display(&self, instance: &dyn std::any::Any) -> String {
        instance.downcast_ref::<Post>()
            .map(|p| p.title().as_str().to_string())
            .unwrap_or_else(|| "Unknown".into())
    }
}

pub struct PostAdmin;

impl ModelAdmin for PostAdmin {
    type Model = PostAdminModel;
    
    fn list_display(&self) -> Vec<&'static str> {
        vec!["title", "author_id", "status", "created_at"]
    }
    
    fn search_fields(&self) -> Vec<&'static str> {
        vec!["title", "body"]
    }
}
```

---

## Migrations

Async ORM with `block_on` escape hatch for CLI operations.

```rust
// ferreiro_adapters_db/migrations.rs

pub struct MigrationEngine<DB: DatabaseBackend> {
    db: DB,
}

impl<DB: DatabaseBackend> MigrationEngine<DB> {
    /// CLI entry point — sync wrapper
    pub fn migrate_sync(&self) -> Result<Vec<String>, MigrationError> {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.migrate())
    }

    pub async fn migrate(&self) -> Result<Vec<String>, MigrationError> {
        self.ensure_migrations_table().await?;
        let applied = self.get_applied_migrations().await?;
        let pending = self.get_pending_migrations(&applied)?;
        
        let mut newly_applied = Vec::new();
        for migration in pending {
            self.apply_migration(&migration).await?;
            newly_applied.push(migration.name.clone());
        }
        Ok(newly_applied)
    }

    pub fn makemigrations_sync(&self, app: &str) -> Result<Option<String>, MigrationError> {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.makemigrations(app))
    }

    pub async fn makemigrations(&self, app: &str) -> Result<Option<String>, MigrationError> {
        let db_schema = self.db.introspect_tables().await?;
        let model_schema = self.get_model_schema(app)?;
        let diff = self.diff_schemas(&db_schema, &model_schema);
        
        if diff.is_empty() {
            return Ok(None);
        }
        
        self.generate_migration(app, &diff)
    }
}
```

---

## CLI

```rust
// ferreiro_cli/main.rs

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ferreiro")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Startproject { name: String },
    Startapp { name: String },
    Runserver {
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short, long, default_value = "8000")]
        port: u16,
    },
    Migrate { #[arg(short, long)] app: Option<String> },
    Makemigrations { #[arg(short, long)] app: Option<String> },
    Createsuperuser,
    Shell,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Migrate { app } => commands::migrate::run(app.as_deref()),
        Commands::Makemigrations { app } => commands::makemigrations::run(app.as_deref()),
        Commands::Runserver { host, port } => commands::runserver::run(&host, port),
        Commands::Startproject { name } => commands::startproject::run(&name),
        Commands::Startapp { name } => commands::startapp::run(&name),
        Commands::Createsuperuser => commands::createsuperuser::run(),
        Commands::Shell => commands::shell::run(),
    }
}
```

---

## Wiring It Together

```rust
// main.rs

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::load()?;
    
    // =========== Driven Adapters ===========
    
    // Database
    let db: Arc<dyn DatabaseBackend> = match settings.database.backend.as_str() {
        "postgresql" => Arc::new(PostgresBackend::connect(&settings.database.url).await?),
        "sqlite" => Arc::new(SqliteBackend::connect(&settings.database.url).await?),
        _ => panic!("Unsupported database backend"),
    };
    
    // Sessions
    let session_store: Arc<dyn SessionStore> = match settings.session.backend.as_str() {
        "cookie" => Arc::new(CookieSessionStore::new(
            settings.secret_key.as_bytes(),
            std::time::Duration::from_secs(14 * 24 * 3600),
        )),
        "database" => Arc::new(DatabaseSessionStore::new(db.clone(), chrono::Duration::days(14))),
        "redis" => Arc::new(RedisSessionStore::new(&settings.redis.url, 14 * 24 * 3600)?),
        _ => panic!("Unsupported session backend"),
    };
    
    // Templates
    let templates: Arc<dyn TemplateEngine> = match settings.templates.engine.as_str() {
        "tera" => Arc::new(TeraEngine::new(&settings.templates.dir)?),
        "minijinja" => Arc::new(MiniJinjaEngine::new(&settings.templates.dir)?),
        _ => panic!("Unsupported template engine"),
    };
    
    // Repositories
    let post_repo = Arc::new(PostgresPostRepository::new(db.clone()));
    let events = Arc::new(InMemoryEventPublisher::new());
    
    // =========== Application Services ===========
    
    let post_service = Arc::new(PostServiceImpl::new(post_repo, events));
    
    // =========== HTTP Layer ===========
    
    let post_handlers = PostHandlers::new(post_service, templates.clone());
    
    let app = Router::new()
        .route("/", get(|r| post_handlers.list(r)))
        .route("/posts/:slug", get(|r, s| post_handlers.detail(r, s)))
        .layer(SessionMiddleware::new(session_store))
        .layer(LoggingMiddleware::new());
    
    println!("Starting server at http://{}:{}", settings.host, settings.port);
    serve(app, &settings.host, settings.port).await
}
```

---

## Why This Exists

Building web apps in Rust shouldn't require assembling dozens of crates and making architectural decisions before writing a single handler. Frameworks like Django and Rails succeeded because they eliminated decision fatigue and let developers focus on building.

Rust deserves the same. Not a minimal router that leaves you to figure out the rest. A complete framework where the batteries are actually included.

Ferreiro is for developers who:
- Want to ship quickly without sacrificing Rust's safety and performance
- Prefer convention over configuration
- Value their time more than demonstrating they can wire up infrastructure from scratch
- Believe "batteries included" is a feature, not a flaw

This framework embraces the philosophy that wanting things to "just work" doesn't make you lazy — it makes you productive.

**Inspired by**: [Rust needs a web framework for lazy developers](https://ntietz.com/blog/rust-needs-a-web-framework-for-lazy-developers/) and decades of Django showing the way.

---

## Implementation Order

1. **ferreiro_domain** — Models, value objects, ports (traits only)
2. **ferreiro_adapters_db** — Backend trait, PostgreSQL, SQLite
3. **ferreiro_adapters_db** — Repository implementations
4. **ferreiro_application** — Service implementations
5. **ferreiro_adapters_session** — Cookie, Database, Redis
6. **ferreiro_adapters_templates** — Tera, MiniJinja
7. **ferreiro_adapters_http** — Routing, middleware, handlers
8. **ferreiro_adapters_db** — Migration engine
9. **ferreiro_adapters_admin** — Introspection traits, CRUD generation
10. **ferreiro_cli** — Management commands
11. **ferreiro** — Umbrella crate, prelude

---

## Dependencies

```toml
[workspace]
members = [
    "ferreiro_domain",
    "ferreiro_application", 
    "ferreiro_adapters_db",
    "ferreiro_adapters_http",
    "ferreiro_adapters_templates",
    "ferreiro_adapters_session",
    "ferreiro_adapters_admin",
    "ferreiro_cli",
    "ferreiro",
]

# ferreiro_domain/Cargo.toml — minimal dependencies
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4"] }
thiserror = "1"

# ferreiro_adapters_db/Cargo.toml
[dependencies]
ferreiro_domain = { path = "../ferreiro_domain" }
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "sqlite", "uuid", "chrono"] }
async-trait = "0.1"
tokio = { version = "1", features = ["full"] }

# ferreiro_adapters_session/Cargo.toml
[dependencies]
ferreiro_domain = { path = "../ferreiro_domain" }
async-trait = "0.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
redis = { version = "0.24", features = ["tokio-comp"] }
hmac = "0.12"
sha2 = "0.10"
base64 = "0.21"
rand = "0.8"
chrono = "0.4"

# ferreiro_adapters_templates/Cargo.toml
[dependencies]
tera = "1"
minijinja = { version = "1", features = ["loader"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# ferreiro_adapters_http/Cargo.toml
[dependencies]
ferreiro_domain = { path = "../ferreiro_domain" }
ferreiro_adapters_templates = { path = "../ferreiro_adapters_templates" }
ferreiro_adapters_session = { path = "../ferreiro_adapters_session" }
axum = "0.7"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
tokio = { version = "1", features = ["full"] }

# ferreiro_cli/Cargo.toml
[dependencies]
ferreiro_domain = { path = "../ferreiro_domain" }
ferreiro_adapters_db = { path = "../ferreiro_adapters_db" }
clap = { version = "4", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
dialoguer = "0.11"
```

---

## License

[Apache 2.0](LICENSE)
