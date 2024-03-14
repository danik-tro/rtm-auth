use super::value_objects::ValueObjectError;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("{0}")]
    ValueObject(
        #[from]
        #[source]
        ValueObjectError,
    ),
}
