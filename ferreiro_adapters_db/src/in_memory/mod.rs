use async_trait::async_trait;
use ferreiro_domain::events::DomainEvent;
use ferreiro_domain::models::Post;
use ferreiro_domain::ports::driven::{
    EventError, EventPublisher, PaginatedResult, Pagination, PostFilter, PostRepository,
    RepositoryError,
};
use ferreiro_domain::values::{PostId, Slug};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// In-memory implementation for testing
#[derive(Clone)]
pub struct InMemoryPostRepository {
    posts: Arc<RwLock<HashMap<PostId, Post>>>,
}

impl InMemoryPostRepository {
    pub fn new() -> Self {
        Self {
            posts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryPostRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PostRepository for InMemoryPostRepository {
    async fn find_by_id(&self, id: &PostId) -> Result<Option<Post>, RepositoryError> {
        let posts = self.posts.read().unwrap();
        Ok(posts.get(id).cloned())
    }

    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<Post>, RepositoryError> {
        let posts = self.posts.read().unwrap();
        Ok(posts.values().find(|p| p.slug() == slug).cloned())
    }

    async fn save(&self, post: &Post) -> Result<(), RepositoryError> {
        let mut posts = self.posts.write().unwrap();
        posts.insert(post.id().clone(), post.clone());
        Ok(())
    }

    async fn delete(&self, id: &PostId) -> Result<(), RepositoryError> {
        let mut posts = self.posts.write().unwrap();
        posts.remove(id);
        Ok(())
    }

    async fn list(
        &self,
        filter: PostFilter,
        pagination: Pagination,
    ) -> Result<PaginatedResult<Post>, RepositoryError> {
        let posts = self.posts.read().unwrap();
        let mut items: Vec<Post> = posts.values().cloned().collect();

        // Apply filters
        if let Some(author_id) = &filter.author_id {
            items.retain(|p| p.author_id() == author_id);
        }
        if let Some(status) = &filter.status {
            items.retain(|p| p.status() == status);
        }
        if let Some(published_after) = filter.published_after {
            items.retain(|p| {
                p.published_at()
                    .map(|pa| pa > published_after)
                    .unwrap_or(false)
            });
        }

        let total = items.len();
        let total_pages = total.div_ceil(pagination.per_page);

        // Apply pagination
        let start = (pagination.page - 1) * pagination.per_page;
        items = items
            .into_iter()
            .skip(start)
            .take(pagination.per_page)
            .collect();

        Ok(PaginatedResult {
            items,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages,
        })
    }

    async fn exists_by_slug(&self, slug: &Slug) -> Result<bool, RepositoryError> {
        let posts = self.posts.read().unwrap();
        Ok(posts.values().any(|p| p.slug() == slug))
    }
}

/// In-memory event publisher for testing
#[derive(Clone)]
pub struct InMemoryEventPublisher {
    events: Arc<RwLock<Vec<DomainEvent>>>,
}

impl InMemoryEventPublisher {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn get_events(&self) -> Vec<DomainEvent> {
        self.events.read().unwrap().clone()
    }

    pub fn clear(&self) {
        self.events.write().unwrap().clear();
    }
}

impl Default for InMemoryEventPublisher {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventPublisher for InMemoryEventPublisher {
    async fn publish(&self, event: DomainEvent) -> Result<(), EventError> {
        self.events.write().unwrap().push(event);
        Ok(())
    }

    async fn publish_all(&self, events: Vec<DomainEvent>) -> Result<(), EventError> {
        self.events.write().unwrap().extend(events);
        Ok(())
    }
}
