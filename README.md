# Ferreiro

[![Crates.io](https://img.shields.io/crates/v/ferreiro.svg)](https://crates.io/crates/ferreiro)
[![Documentation](https://docs.rs/ferreiro/badge.svg)](https://docs.rs/ferreiro)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

> **Alpha Release (v0.0.1)** - Ferreiro is in early development. The foundation is solid, but many features are planned and not yet implemented. See [What Works Now](#what-works-now) below.

A Django-inspired web framework for Rust, built on hexagonal architecture. **For developers who want to build, not configure.**

## What is Ferreiro?

Ferreiro brings Django's "batteries included" philosophy to Rust. Instead of assembling dozens of crates and making architectural decisions before writing your first handler, Ferreiro gives you a complete, opinionated framework where everything works together out of the box.

**Core Principles:**
- **Convention over configuration** - Sensible defaults that just work
- **Hexagonal architecture** - Clean separation between domain logic and infrastructure
- **Swappable adapters** - Start with in-memory, swap to PostgreSQL later with zero domain changes
- **Batteries included** - Everything you need to ship is included

## What Works Now

Ferreiro v0.0.1 provides the architectural foundation and core components:

✅ **Domain Modeling**
- Value objects with built-in validation (Email, Slug, Title, Body)
- Domain events for business logic
- Pure domain layer with zero framework dependencies

✅ **Repository Pattern**
- In-memory repositories for testing
- Clean port/adapter separation
- Ready for PostgreSQL/SQLite adapters

✅ **HTTP Server**
- Axum-based HTTP layer
- Clean routing and handlers
- Middleware support

✅ **Template Engines**
- Tera (default, Jinja2-like syntax)
- MiniJinja (Django-like syntax)
- Swappable via feature flags

✅ **Session Management**
- Cookie-based sessions (signed HMAC)
- Memory-backed sessions
- Ready for database/Redis backends

✅ **Service Layer**
- Application services implementing use cases
- Event publishing on domain operations
- Clean separation from HTTP layer

## Quick Start

Add Ferreiro to your `Cargo.toml`:

```toml
[dependencies]
ferreiro = "0.0.1"
tokio = { version = "1", features = ["full"] }
```

Create a simple blog:

```rust
use ferreiro::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Set up in-memory adapters
    let post_repo = Arc::new(InMemoryPostRepository::new());
    let events = Arc::new(InMemoryEventPublisher::new());

    // Create application service
    let post_service = Arc::new(PostServiceImpl::new(post_repo, events));

    // Create a post
    let post = post_service.create(CreatePostCommand {
        title: "Hello Ferreiro".to_string(),
        slug: "hello-ferreiro".to_string(),
        body: "My first post!".to_string(),
        author_id: UserId::generate(),
    }).await.unwrap();

    println!("Created post: {}", post.title().as_str());
}
```

See the [simple_blog example](ferreiro/examples/simple_blog.rs) for a complete working application with HTTP server.

## Architecture

Ferreiro uses hexagonal architecture (ports and adapters) to keep your business logic clean and testable:

```
┌─────────────────────────────────────────┐
│         Adapters (HTTP, CLI)            │
│                   ▼                     │
│         Ports (Service Traits)          │
│                   ▼                     │
│         Domain (Models, Logic)          │
│                   ▼                     │
│    Ports (Repository, Event Traits)     │
│                   ▼                     │
│   Adapters (Database, Templates, etc)   │
└─────────────────────────────────────────┘
```

**Key Benefits:**
- Domain logic has zero framework dependencies
- Easy to test (swap real adapters for in-memory)
- Change infrastructure without touching business logic
- Clear boundaries between layers

## Project Structure

```
ferreiro/
├── ferreiro_domain/           # Pure business logic
│   ├── models/               # Aggregates (Post, User)
│   ├── values/               # Value objects (Email, Slug)
│   ├── events.rs             # Domain events
│   └── ports/                # Trait definitions
├── ferreiro_application/      # Use case implementations
│   └── services/             # Service layer
├── ferreiro_adapters_db/      # Database adapters
│   └── in_memory/            # In-memory for testing
├── ferreiro_adapters_http/    # HTTP layer (Axum)
├── ferreiro_adapters_templates/ # Template engines
├── ferreiro_adapters_session/ # Session storage
├── ferreiro_adapters_admin/   # Admin interface (planned)
├── ferreiro_cli/             # CLI tools (planned)
└── ferreiro/                 # Main crate with prelude
```

## Roadmap

Ferreiro is being built iteratively. Here's what's coming:

**v0.1.0 - Database Layer**
- PostgreSQL adapter
- SQLite adapter
- Migration system
- UserRepository implementations

**v0.2.0 - Admin Interface**
- Auto-generated CRUD interface
- Model introspection
- Authentication and permissions

**v0.3.0 - CLI Tools**
- `ferreiro new` - Project scaffolding
- `ferreiro migrate` - Run migrations
- `ferreiro serve` - Development server

**v0.4.0 - Authentication**
- User authentication system
- Permissions and roles
- OAuth2 support

**v1.0.0 - Production Ready**
- Documentation site (built with Ferreiro!)
- Performance optimization
- Security audit
- Comprehensive examples

See [GitHub Milestones](https://github.com/ibrahimcesar/ferreiro/milestones) for detailed tracking.

## Examples

Check out the [examples directory](ferreiro/examples/):
- **simple_blog.rs** - Complete blog with posts, listing, and HTTP server

More examples coming soon:
- E-commerce application
- REST API with authentication
- Real-time chat with WebSockets

## Documentation

- **[QUICKSTART.md](QUICKSTART.md)** - Get started in 5 minutes
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - How to contribute
- **[Architecture Guide](https://docs.rs/ferreiro)** - Detailed architecture documentation

## Why Ferreiro?

**Other Rust frameworks:**
"Here's a router. Pick a database crate. Find a migration tool. Set up sessions. Configure logging. Wire up everything yourself."

**Ferreiro:**
"Here's a complete framework. Start building your app."

You're not lazy for wanting this. You're productive. You want to build features, not infrastructure.

### Inspired By
- Django - For showing that batteries-included works
- Hexagonal Architecture - For clean, testable design
- [Rust needs a web framework for lazy developers](https://ntietz.com/blog/rust-needs-a-web-framework-for-lazy-developers/)

## Contributing

Ferreiro is in active development and welcomes contributors! See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development setup
- Architecture guidelines
- Good first issues

## Community

- **Issues**: [GitHub Issues](https://github.com/ibrahimcesar/ferreiro/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ibrahimcesar/ferreiro/discussions)
- **Twitter**: [@yourhandle](https://twitter.com/yourhandle) (coming soon)

## License

Apache 2.0 - See [LICENSE](LICENSE)

---

**Status**: Alpha (v0.0.1) - The foundation is solid. Features are being built milestone by milestone. Join us on the journey! 🔨
