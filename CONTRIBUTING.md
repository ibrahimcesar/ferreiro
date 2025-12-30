# Contributing to Ferreiro

Thank you for your interest in contributing to Ferreiro! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful, constructive, and collaborative. We're building something useful together.

## How Can I Contribute?

### Reporting Bugs

Open an issue with:
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Your environment (OS, Rust version)

### Suggesting Features

Open an issue with:
- Use case description
- Proposed API/interface
- Why this fits Ferreiro's philosophy

### Contributing Code

1. **Fork the repository**
2. **Create a branch**: `git checkout -b feature/your-feature-name`
3. **Make your changes**
4. **Run tests**: `cargo test`
5. **Run clippy**: `cargo clippy --all-targets -- -D warnings`
6. **Format code**: `cargo fmt --all`
7. **Commit**: Use clear, descriptive commit messages
8. **Push**: `git push origin feature/your-feature-name`
9. **Open a Pull Request**

## Development Setup

```bash
git clone https://github.com/ibrahimcesar/ferreiro
cd ferreiro
cargo build
cargo test
```

## Project Structure

```
ferreiro/
├── ferreiro_domain/          # Pure domain logic (no deps)
├── ferreiro_application/     # Use cases
├── ferreiro_adapters_*/      # Infrastructure adapters
├── ferreiro_cli/             # CLI tool
└── ferreiro/                 # Main crate
```

## Architecture Principles

Ferreiro follows hexagonal architecture:

1. **Domain layer must remain pure** - No framework dependencies
2. **Business logic in domain** - Not in handlers or adapters
3. **Ports define interfaces** - Traits for repositories and services
4. **Adapters implement ports** - Can be swapped easily

### Example: Adding a New Feature

**Bad** ❌ - Framework in domain:
```rust
// Don't do this in ferreiro_domain
use sqlx::PgPool;  // ❌ Database in domain

pub struct Post {
    pool: PgPool,  // ❌
}
```

**Good** ✅ - Port in domain, adapter outside:
```rust
// ferreiro_domain/ports/driven.rs
#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn save(&self, post: &Post) -> Result<()>;
}

// ferreiro_adapters_db/postgres/post_repository.rs
pub struct PostgresPostRepository {
    pool: PgPool,  // ✅ In adapter, not domain
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
    async fn save(&self, post: &Post) -> Result<()> {
        // Implementation
    }
}
```

## Testing

- **Domain tests**: Pure unit tests, no mocks needed
- **Service tests**: Use in-memory adapters
- **Integration tests**: Test full stack if needed

```rust
#[tokio::test]
async fn test_post_service() {
    let repo = Arc::new(InMemoryPostRepository::new());
    let events = Arc::new(InMemoryEventPublisher::new());
    let service = PostServiceImpl::new(repo, events);

    // Test your feature
}
```

## Documentation

- Add rustdoc comments to public APIs
- Update README if adding major features
- Add examples for new functionality

## Priority Areas (v0.0.x → v0.1.0)

We'd especially welcome help with:

1. **PostgreSQL Adapter** - Implement PostRepository for PostgreSQL
2. **SQLite Adapter** - Implement PostRepository for SQLite
3. **Migration Engine** - Schema introspection and migration generation
4. **Admin Interface** - Model introspection and CRUD generation
5. **Authentication Service** - User registration, login, sessions
6. **Documentation** - More examples and guides

## Style Guide

- Follow Rust conventions
- Use `cargo fmt`
- Pass `cargo clippy` with no warnings
- Keep lines under 100 characters
- Write clear, descriptive names

## Commit Messages

Format:
```
<type>: <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance

Example:
```
feat: Add PostgreSQL adapter for PostRepository

Implements the PostRepository trait for PostgreSQL using SQLx.
Includes connection pooling and proper error handling.

Closes #42
```

## Questions?

- Open an issue for discussion
- Ask in discussions
- Check existing issues and PRs

## License

By contributing, you agree that your contributions will be licensed under Apache-2.0.

---

Thank you for contributing to Ferreiro! 🔨
