use ferreiro_application::services::PostServiceImpl;
use ferreiro_domain::models::PostStatus;
use ferreiro_domain::ports::driven::{Pagination, PostFilter};
use ferreiro_domain::ports::driving::{CreatePostCommand, ListPostsQuery, PostService};
use ferreiro_domain::values::UserId;
use std::sync::Arc;

// Import in-memory implementations from ferreiro_adapters_db
use ferreiro_adapters_db::{InMemoryEventPublisher, InMemoryPostRepository};

#[tokio::test]
async fn test_create_post() {
    let repo = Arc::new(InMemoryPostRepository::new());
    let events = Arc::new(InMemoryEventPublisher::new());
    let service = PostServiceImpl::new(repo, events.clone());

    let post = service
        .create(CreatePostCommand {
            title: "Test Post".to_string(),
            slug: "test-post".to_string(),
            body: "This is a test".to_string(),
            author_id: UserId::generate(),
        })
        .await
        .unwrap();

    assert_eq!(post.title().as_str(), "Test Post");
    assert_eq!(post.slug().as_str(), "test-post");
    assert_eq!(post.status(), &PostStatus::Draft);

    // Verify event was published
    let published_events = events.get_events();
    assert_eq!(published_events.len(), 1);
}

#[tokio::test]
async fn test_publish_post() {
    let repo = Arc::new(InMemoryPostRepository::new());
    let events = Arc::new(InMemoryEventPublisher::new());
    let service = PostServiceImpl::new(repo, events.clone());

    let post = service
        .create(CreatePostCommand {
            title: "Test Post".to_string(),
            slug: "test-post".to_string(),
            body: "This is a test".to_string(),
            author_id: UserId::generate(),
        })
        .await
        .unwrap();

    let published = service.publish(post.id()).await.unwrap();

    assert_eq!(published.status(), &PostStatus::Published);
    assert!(published.published_at().is_some());

    // Verify both events were published
    let published_events = events.get_events();
    assert_eq!(published_events.len(), 2); // Created + Published
}

#[tokio::test]
async fn test_list_posts() {
    let repo = Arc::new(InMemoryPostRepository::new());
    let events = Arc::new(InMemoryEventPublisher::new());
    let service = PostServiceImpl::new(repo, events);

    // Create multiple posts
    for i in 1..=5 {
        service
            .create(CreatePostCommand {
                title: format!("Post {}", i),
                slug: format!("post-{}", i),
                body: format!("Content {}", i),
                author_id: UserId::generate(),
            })
            .await
            .unwrap();
    }

    let result = service
        .list(ListPostsQuery {
            filter: PostFilter::default(),
            pagination: Pagination::default(),
        })
        .await
        .unwrap();

    assert_eq!(result.total, 5);
    assert_eq!(result.items.len(), 5);
}

#[tokio::test]
async fn test_get_by_slug() {
    let repo = Arc::new(InMemoryPostRepository::new());
    let events = Arc::new(InMemoryEventPublisher::new());
    let service = PostServiceImpl::new(repo, events);

    service
        .create(CreatePostCommand {
            title: "Unique Post".to_string(),
            slug: "unique-post".to_string(),
            body: "Content".to_string(),
            author_id: UserId::generate(),
        })
        .await
        .unwrap();

    let found = service.get_by_slug("unique-post").await.unwrap();

    assert!(found.is_some());
    assert_eq!(found.unwrap().title().as_str(), "Unique Post");

    let not_found = service.get_by_slug("does-not-exist").await.unwrap();
    assert!(not_found.is_none());
}
