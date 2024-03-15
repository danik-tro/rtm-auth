use crate::domain::{
    models::{CreateUser, CreateUserDto},
    repositories::ArcUserRepository,
    services::{RegistrationService, RegistrationServiceError},
};

use super::hash_password;

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
        if !self.user_repository.is_email_unique(&user.email).await? {
            return Err(RegistrationServiceError::EmailIsNotUnique);
        }

        let password_hash =
            tokio::task::spawn_blocking(move || hash_password(&user.password)).await??;

        let create_user = CreateUser {
            id: uuid7::new_v7(),
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            password_hash,
        };

        self.user_repository.create_user(create_user).await?;
        Ok(())
    }
}
