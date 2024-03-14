use super::{
    models::value_objects::ValueObjectError,
    services::{AuthenticationServiceError, RegistrationServiceError},
};

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("{0}")]
    ValueObject(
        #[from]
        #[source]
        ValueObjectError,
    ),
    #[error("{0}")]
    RegistrationService(
        #[from]
        #[source]
        RegistrationServiceError,
    ),
    #[error("{0}")]
    AuthenticationService(
        #[from]
        #[source]
        AuthenticationServiceError,
    ),
}
