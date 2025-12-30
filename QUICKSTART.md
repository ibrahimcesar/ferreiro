# Ferreiro - Quick Start Guide

Get up and running with Ferreiro in 5 minutes.

## Prerequisites

- Rust 1.70 or later
- Cargo

## Installation

Clone the repository:

```bash
git clone <your-repo-url>
cd ferreiro
```

## Verify Installation

Build and test the project:

```bash
# Build everything
cargo build

# Run tests
cargo test

# Build CLI tool
cargo build --bin ferreiro
```

## Run the Example

The `simple_blog` example demonstrates the core features:

```bash
cargo run --example simple_blog
```

This will:
1. Create sample blog posts
2. Publish one of them
3. Start an HTTP server on http://127.0.0.1:8000

Try these endpoints:
```bash
# Welcome message
curl http://127.0.0.1:8000/

# List all posts (JSON)
curl http://127.0.0.1:8000/posts | jq

# Get a specific post
curl http://127.0.0.1:8000/posts/welcome-to-ferreiro | jq
```

## Your First Application

### 1. Create a New Project

```bash
# This will be implemented in the CLI
# For now, you can copy the example
mkdir my_blog
cd my_blog
```

### 2. Write Your Domain Model

```rust
// src/domain/models/article.rs
use ferreiro::prelude::*;

pub struct Article {
    id: ArticleId,
    title: Title,
    content: Body,
    // ...
}
```

### 3. Implement Your Service

```rust
// src/application/article_service.rs
use ferreiro::prelude::*;

pub struct ArticleServiceImpl<R, E>
where
    R: ArticleRepository,
    E: EventPublisher,
{
    repo: Arc<R>,
    events: Arc<E>,
}

impl<R, E> ArticleService for ArticleServiceImpl<R, E> {
    async fn create(&self, cmd: CreateArticleCommand) -> Result<Article> {
        // Your business logic here
    }
}
```

### 4. Set Up HTTP Handlers

```rust
// src/adapters/http/handlers.rs
use ferreiro::prelude::*;

async fn list_articles(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<ArticleResponse>> {
    // Your handler logic
}
```

### 5. Wire Everything Together

```rust
// src/main.rs
use ferreiro::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup repositories
    let repo = Arc::new(InMemoryArticleRepository::new());
    let events = Arc::new(InMemoryEventPublisher::new());

    // Create service
    let service = Arc::new(ArticleServiceImpl::new(repo, events));

    // Build HTTP app
    let app = Router::new()
        .route("/articles", get(list_articles))
        .with_state(service);

    // Start server
    serve(app, "127.0.0.1", 8000).await
}
```

## Project Structure

A typical Ferreiro project follows this structure:

```
my_blog/
├── src/
│   ├── domain/              # Pure domain logic
│   │   ├── models/          # Aggregates and entities
│   │   ├── values/          # Value objects
│   │   ├── events.rs        # Domain events
│   │   └── ports/           # Repository traits
│   ├── application/         # Use cases
│   │   └── services/        # Service implementations
│   ├── adapters/            # Infrastructure
│   │   ├── db/              # Database repositories
│   │   ├── http/            # HTTP handlers
│   │   └── templates/       # Template files
│   └── main.rs              # Application entry point
├── tests/                   # Integration tests
├── Cargo.toml
└── README.md
```

## Key Concepts

### Hexagonal Architecture

Ferreiro uses ports and adapters pattern:

- **Domain**: Your business logic (center)
- **Ports**: Interfaces (traits) for communication
- **Adapters**: Implementations (outside)

### Pure Domain

Your domain layer should have zero dependencies on frameworks:

```rust
// ✅ Good - Pure domain
pub struct Post {
    id: PostId,
    title: Title,
}

impl Post {
    pub fn publish(&mut self) -> Result<(), DomainError> {
        // Business rules only
    }
}

// ❌ Bad - Framework coupling
pub struct Post {
    #[sql(primary_key)]  // Database concern in domain
    id: i32,
}
```

### Ports (Traits)

Define interfaces in the domain:

```rust
#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find(&self, id: &PostId) -> Result<Option<Post>>;
    async fn save(&self, post: &Post) -> Result<()>;
}
```

### Adapters (Implementations)

Implement ports in adapter layers:

```rust
pub struct PostgresPostRepository {
    pool: PgPool,
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
    async fn find(&self, id: &PostId) -> Result<Option<Post>> {
        // Database-specific implementation
    }
}
```

## Testing

Ferreiro makes testing easy with in-memory adapters:

```rust
#[tokio::test]
async fn test_create_post() {
    let repo = Arc::new(InMemoryPostRepository::new());
    let events = Arc::new(InMemoryEventPublisher::new());
    let service = PostServiceImpl::new(repo, events);

    let post = service.create(cmd).await.unwrap();

    assert_eq!(post.title().as_str(), "Test");
}
```

## CLI Commands

```bash
# Create a new project
ferreiro startproject my_blog

# Create a new app within your project
ferreiro startapp articles

# Run the development server
ferreiro runserver

# With hot reload
ferreiro runserver --hot-reload

# Create migrations
ferreiro makemigrations

# Run migrations
ferreiro migrate

# Create admin user
ferreiro createsuperuser

# Open REPL
ferreiro shell
```

*Note: Most CLI commands are placeholders and will be implemented in future iterations.*

## Next Steps

1. **Read the README**: Understand the philosophy and architecture
2. **Explore the example**: See how everything fits together
3. **Check BOOTSTRAP.md**: See what's implemented and what's next
4. **Write your domain**: Start with pure business logic
5. **Add adapters**: Implement persistence and HTTP layers

## Getting Help

- **Documentation**: See the rustdoc (`cargo doc --open`)
- **Examples**: Check the `examples/` directory
- **Tests**: Look at tests for usage patterns
- **Issues**: Report problems on GitHub

## Common Patterns

### Creating a Post

```rust
let post = service.create(CreatePostCommand {
    title: "My Title".to_string(),
    slug: "my-title".to_string(),
    body: "Content".to_string(),
    author_id: user.id().clone(),
}).await?;
```

### Publishing a Post

```rust
let published = service.publish(post.id()).await?;
```

### Listing Posts

```rust
let result = service.list(ListPostsQuery {
    filter: PostFilter {
        status: Some(PostStatus::Published),
        ..Default::default()
    },
    pagination: Pagination {
        page: 1,
        per_page: 10,
    },
}).await?;

for post in result.items {
    println!("{}", post.title());
}
```

### Handling Errors

```rust
match service.create(cmd).await {
    Ok(post) => println!("Created: {}", post.id()),
    Err(ServiceError::Domain(DomainError::EmptySlug)) => {
        println!("Slug cannot be empty")
    }
    Err(ServiceError::Conflict(msg)) => {
        println!("Conflict: {}", msg)
    }
    Err(e) => println!("Error: {}", e),
}
```

## Performance Tips

1. **Use Arc**: Share repositories and services across handlers
2. **Connection Pools**: Use database connection pooling
3. **In-memory for tests**: Fast test execution with in-memory adapters
4. **Async everywhere**: Leverage Tokio's async runtime

## What's Different from Other Frameworks?

### vs Actix-web/Axum
- Ferreiro provides the full stack, not just HTTP
- Built-in migrations, admin, background jobs
- Domain-driven design by default

### vs Rocket
- Hexagonal architecture
- More opinionated about structure
- Adapter pattern for swappable implementations

### vs Django (Python)
- Type-safe at compile time
- Explicit over implicit
- Zero-cost abstractions

## License

Apache 2.0
