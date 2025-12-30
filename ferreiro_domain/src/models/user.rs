use crate::values::{Email, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    email: Email,
    name: String,
    password_hash: String,
    created_at: DateTime<Utc>,
    is_active: bool,
    is_staff: bool,
    is_superuser: bool,
}

impl User {
    pub fn new(email: Email, name: String, password_hash: String) -> Self {
        Self {
            id: UserId::generate(),
            email,
            name,
            password_hash,
            created_at: Utc::now(),
            is_active: true,
            is_staff: false,
            is_superuser: false,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: UserId,
        email: Email,
        name: String,
        password_hash: String,
        created_at: DateTime<Utc>,
        is_active: bool,
        is_staff: bool,
        is_superuser: bool,
    ) -> Self {
        Self {
            id,
            email,
            name,
            password_hash,
            created_at,
            is_active,
            is_staff,
            is_superuser,
        }
    }

    // Getters
    pub fn id(&self) -> &UserId {
        &self.id
    }
    pub fn email(&self) -> &Email {
        &self.email
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    pub fn is_staff(&self) -> bool {
        self.is_staff
    }
    pub fn is_superuser(&self) -> bool {
        self.is_superuser
    }

    // Setters
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
    pub fn activate(&mut self) {
        self.is_active = true;
    }
    pub fn make_staff(&mut self) {
        self.is_staff = true;
    }
    pub fn make_superuser(&mut self) {
        self.is_superuser = true;
        self.is_staff = true;
    }
}
