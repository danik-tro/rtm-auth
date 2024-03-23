use std::sync::Arc;

use async_trait::async_trait;

use tokio::task::JoinError;

use super::{models::CreateUserDto, repositories::UserRepositoryError};
use crate::domain::models::LoginDto;

pub type ArcRegistrationService = Arc<dyn RegistrationService + Send + Sync + 'static>;
pub type ArcAuthenticationService = Arc<dyn AuthenticationService + Send + Sync + 'static>;

type AuthResult<T> = std::result::Result<T, AuthenticationServiceError>;
type RegistrationResult<T> = std::result::Result<T, RegistrationServiceError>;

#[async_trait]
pub trait AuthenticationService {
    async fn login(&self, creds: LoginDto) -> AuthResult<()>;
}

#[async_trait]
pub trait RegistrationService {
    async fn register(&self, user: CreateUserDto) -> RegistrationResult<()>;
}

#[derive(Debug, thiserror::Error)]
pub enum AuthenticationServiceError {
    #[error("The email or password is incorrect.")]
    IncorrectCredentials,
    #[error("Data access error: {0}")]
    UserRepository(
        #[from]
        #[source]
        UserRepositoryError,
    ),
    #[error("{0}")]
    VerifyPassword(HashError),
}

#[derive(Debug, thiserror::Error)]
pub enum RegistrationServiceError {
    #[error("The user with such an email already exists.")]
    EmailIsNotUnique,
    #[error("Data access error: {0}")]
    UserRepository(
        #[from]
        #[source]
        UserRepositoryError,
    ),
    #[error("{0}")]
    HashPassword(
        #[from]
        #[source]
        HashError,
    ),
    #[error("Internal error.")]
    Tokio(
        #[from]
        #[source]
        JoinError,
    ),
}

#[derive(Debug, thiserror::Error)]
pub enum HashError {
    #[error("The email or password is incorrect.")]
    VerifiyFailed,
    #[error("Internal error.")]
    Argon2(String),
}
