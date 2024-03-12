use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::models::CreateUserDto;

use super::UserRepositoryError;

type Result<T> = std::result::Result<T, UserRepositoryError>;

pub type ArcUserRepository = Arc<dyn UserRepository + Send + Sync + 'static>;

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: CreateUserDto) -> Result<()>;
    // TODO: replace email type with specialized type like Email
    async fn is_email_unique(&self, email: String) -> Result<bool>;
}
