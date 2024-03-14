use super::email::ParseEmailError;

#[derive(Debug, thiserror::Error)]
pub enum ValueObjectError {
    #[error("Validation error: {0}")]
    Email(
        #[from]
        #[source]
        ParseEmailError,
    ),
}
