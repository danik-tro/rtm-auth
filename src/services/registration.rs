use crate::domain::{
    models::CreateUserDto,
    repositories::ArcUserRepository,
    services::{RegistrationService, RegistrationServiceError},
};

pub struct RegistrationServiceImpl {
    user_repository: ArcUserRepository,
}

impl RegistrationServiceImpl {
    pub fn new(user_repository: ArcUserRepository) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl RegistrationService for RegistrationServiceImpl {
    async fn register(&self, user: CreateUserDto) -> Result<(), RegistrationServiceError> {
        self.user_repository.create_user(user).await?;
        Ok(())
    }
}
