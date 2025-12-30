use async_trait::async_trait;
use chrono::Utc;
use ferreiro_domain::events::DomainEvent;
use ferreiro_domain::models::Post;
use ferreiro_domain::ports::driven::{EventPublisher, PostRepository};
use ferreiro_domain::ports::driving::{
    CreatePostCommand, ListPostsQuery, PostService, ServiceError, UpdatePostCommand,
};
use ferreiro_domain::values::{Body, PostId, Slug, Title};
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

        if self
            .post_repo
            .exists_by_slug(&slug)
            .await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))?
        {
            return Err(ServiceError::Conflict("Slug already exists".into()));
        }

        let post = Post::new(title, slug, body, cmd.author_id.clone());

        self.post_repo
            .save(&post)
            .await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))?;

        self.events
            .publish(DomainEvent::PostCreated {
                post_id: post.id().clone(),
                author_id: cmd.author_id,
                occurred_at: Utc::now(),
            })
            .await
            .ok();

        Ok(post)
    }

    async fn update(&self, cmd: UpdatePostCommand) -> Result<Post, ServiceError> {
        let mut post = self
            .post_repo
            .find_by_id(&cmd.id)
            .await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))?
            .ok_or(ServiceError::NotFound)?;

        let title = Title::new(&cmd.title)?;
        let body = Body::new(&cmd.body);

        post.update_content(title, body);

        self.post_repo
            .save(&post)
            .await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))?;

        Ok(post)
    }

    async fn publish(&self, id: &PostId) -> Result<Post, ServiceError> {
        let mut post = self
            .post_repo
            .find_by_id(id)
            .await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))?
            .ok_or(ServiceError::NotFound)?;

        post.publish()?;

        self.post_repo
            .save(&post)
            .await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))?;

        self.events
            .publish(DomainEvent::PostPublished {
                post_id: id.clone(),
                occurred_at: Utc::now(),
            })
            .await
            .ok();

        Ok(post)
    }

    async fn archive(&self, id: &PostId) -> Result<Post, ServiceError> {
        let mut post = self
            .post_repo
            .find_by_id(id)
            .await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))?
            .ok_or(ServiceError::NotFound)?;

        post.archive();

        self.post_repo
            .save(&post)
            .await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))?;

        self.events
            .publish(DomainEvent::PostArchived {
                post_id: id.clone(),
                occurred_at: Utc::now(),
            })
            .await
            .ok();

        Ok(post)
    }

    async fn delete(&self, id: &PostId) -> Result<(), ServiceError> {
        self.post_repo
            .delete(id)
            .await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))?;
        Ok(())
    }

    async fn get(&self, id: &PostId) -> Result<Option<Post>, ServiceError> {
        self.post_repo
            .find_by_id(id)
            .await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))
    }

    async fn get_by_slug(&self, slug: &str) -> Result<Option<Post>, ServiceError> {
        let slug = Slug::new(slug)?;
        self.post_repo
            .find_by_slug(&slug)
            .await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))
    }

    async fn list(
        &self,
        query: ListPostsQuery,
    ) -> Result<ferreiro_domain::ports::driven::PaginatedResult<Post>, ServiceError> {
        self.post_repo
            .list(query.filter, query.pagination)
            .await
            .map_err(|e| ServiceError::Internal(format!("{:?}", e)))
    }
}
