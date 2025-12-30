use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DomainError {
    // Post
    #[error("Cannot publish a post with empty body")]
    CannotPublishEmptyPost,

    #[error("Post is already published")]
    AlreadyPublished,

    // Slug
    #[error("Slug cannot be empty")]
    EmptySlug,

    #[error("Slug too long: {actual} chars (max {max})")]
    SlugTooLong { max: usize, actual: usize },

    #[error("Slug can only contain letters, numbers, and hyphens")]
    InvalidSlugCharacters,

    // Email
    #[error("Invalid email address")]
    InvalidEmail,

    // Title
    #[error("Title cannot be empty")]
    EmptyTitle,

    #[error("Title too long: {actual} chars (max {max})")]
    TitleTooLong { max: usize, actual: usize },

    // Body
    #[error("Body cannot be empty")]
    EmptyBody,

    // Password
    #[error("Password must be at least {min} characters")]
    PasswordTooShort { min: usize },

    #[error("Password is too weak")]
    PasswordTooWeak,

    // User
    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Invalid credentials")]
    InvalidCredentials,
}
