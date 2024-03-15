use super::value_objects::email::ParseEmailError;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("{0}")]
    ParseEmail(
        #[from]
        #[source]
        ParseEmailError,
    ),
}
