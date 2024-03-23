use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::models::{value_objects::email::Email, CreateUser};

pub type ArcUserRepository = Arc<dyn UserRepository + Send + Sync + 'static>;

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: CreateUser) -> Result<(), UserRepositoryError>;

    async fn is_email_unique(&self, email: &Email) -> Result<bool, UserRepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum UserRepositoryError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}
