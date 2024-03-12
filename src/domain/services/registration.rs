use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::models::CreateUserDto;

use super::RegistrationServiceError;

type Result<T> = std::result::Result<T, RegistrationServiceError>;
pub type ArcRegistrationService = Arc<dyn RegistrationService + Send + Sync + 'static>;

#[async_trait]
pub trait RegistrationService {
    async fn register(&self, user: CreateUserDto) -> Result<()>;
}
