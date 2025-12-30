/// A simple blog example demonstrating Ferreiro's core features
///
/// This example shows:
/// - Domain models (Post)
/// - Service layer (PostService)
/// - In-memory repositories for testing
/// - HTTP handlers with Axum
///
/// Run with: cargo run --example simple_blog
use ferreiro::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔨 Ferreiro - Simple Blog Example");
    println!("==================================\n");

    // Setup in-memory adapters for demo
    let post_repo = Arc::new(InMemoryPostRepository::new());
    let events = Arc::new(InMemoryEventPublisher::new());

    // Create the post service
    let post_service = Arc::new(PostServiceImpl::new(post_repo.clone(), events.clone()));

    // Create some sample posts
    println!("Creating sample posts...");

    let post1 = post_service
        .create(CreatePostCommand {
            title: "Welcome to Ferreiro".to_string(),
            slug: "welcome-to-ferreiro".to_string(),
            body: "Ferreiro is a Django-inspired web framework for Rust.".to_string(),
            author_id: UserId::generate(),
        })
        .await?;

    println!("✓ Created post: {}", post1.title());

    let post2 = post_service
        .create(CreatePostCommand {
            title: "Why Hexagonal Architecture".to_string(),
            slug: "why-hexagonal-architecture".to_string(),
            body: "Hexagonal architecture keeps your domain pure.".to_string(),
            author_id: UserId::generate(),
        })
        .await?;

    println!("✓ Created post: {}", post2.title());

    // Publish the first post
    post_service.publish(post1.id()).await?;
    println!("✓ Published: {}", post1.title());

    // List all posts
    let result = post_service
        .list(ListPostsQuery {
            filter: PostFilter::default(),
            pagination: Pagination::default(),
        })
        .await?;

    println!("\n📝 All posts ({} total):", result.total);
    for post in result.items {
        let status = match post.status() {
            PostStatus::Draft => "Draft",
            PostStatus::Published => "Published",
            PostStatus::Archived => "Archived",
        };
        println!("  - {} [{}]", post.title(), status);
    }

    // Show published events
    let published_events = events.get_events();
    println!("\n📢 Events published ({} total):", published_events.len());
    for event in published_events {
        match event {
            DomainEvent::PostCreated { post_id, .. } => {
                println!("  - PostCreated: {}", post_id);
            }
            DomainEvent::PostPublished { post_id, .. } => {
                println!("  - PostPublished: {}", post_id);
            }
            _ => {}
        }
    }

    // Build a simple HTTP API
    println!("\n🌐 Starting HTTP server...");

    let app_state = AppState {
        post_service: post_service.clone(),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/posts", get(list_posts))
        .route("/posts/:slug", get(get_post))
        .with_state(Arc::new(app_state));

    println!("Server running at http://127.0.0.1:8000");
    println!("\nTry:");
    println!("  curl http://127.0.0.1:8000/");
    println!("  curl http://127.0.0.1:8000/posts");
    println!("  curl http://127.0.0.1:8000/posts/welcome-to-ferreiro");

    serve(app, "127.0.0.1", 8000).await?;

    Ok(())
}

// Application state
#[derive(Clone)]
struct AppState {
    post_service: Arc<PostServiceImpl<InMemoryPostRepository, InMemoryEventPublisher>>,
}

// Handlers
async fn index() -> &'static str {
    "🔨 Welcome to Ferreiro - A Django-inspired framework for Rust"
}

async fn list_posts(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<PostResponse>>, (axum::http::StatusCode, String)> {
    let result = state
        .post_service
        .list(ListPostsQuery {
            filter: PostFilter::default(),
            pagination: Pagination::default(),
        })
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let posts: Vec<PostResponse> = result.items.into_iter().map(|p| p.into()).collect();

    Ok(Json(posts))
}

async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Json<PostResponse>, (axum::http::StatusCode, String)> {
    let post = state
        .post_service
        .get_by_slug(&slug)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| {
            (
                axum::http::StatusCode::NOT_FOUND,
                "Post not found".to_string(),
            )
        })?;

    Ok(Json(post.into()))
}

// Response DTOs
#[derive(Serialize)]
struct PostResponse {
    id: String,
    title: String,
    slug: String,
    body: String,
    status: String,
    created_at: String,
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id().to_string(),
            title: post.title().as_str().to_string(),
            slug: post.slug().as_str().to_string(),
            body: post.body().as_str().to_string(),
            status: match post.status() {
                PostStatus::Draft => "draft".to_string(),
                PostStatus::Published => "published".to_string(),
                PostStatus::Archived => "archived".to_string(),
            },
            created_at: post.created_at().to_rfc3339(),
        }
    }
}
