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
}
