use tokio::task::JoinError;

use crate::domain::repositories::UserRepositoryError;

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
