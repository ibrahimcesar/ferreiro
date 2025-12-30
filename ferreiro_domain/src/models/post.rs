use crate::errors::DomainError;
use crate::values::{Body, PostId, Slug, Title, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostStatus {
    Draft,
    Published,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[allow(clippy::too_many_arguments)]
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
        Self {
            id,
            title,
            slug,
            body,
            author_id,
            status,
            created_at,
            published_at,
        }
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
    pub fn id(&self) -> &PostId {
        &self.id
    }
    pub fn title(&self) -> &Title {
        &self.title
    }
    pub fn slug(&self) -> &Slug {
        &self.slug
    }
    pub fn body(&self) -> &Body {
        &self.body
    }
    pub fn author_id(&self) -> &UserId {
        &self.author_id
    }
    pub fn status(&self) -> &PostStatus {
        &self.status
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn published_at(&self) -> Option<DateTime<Utc>> {
        self.published_at
    }
    pub fn is_published(&self) -> bool {
        self.status == PostStatus::Published
    }
}
