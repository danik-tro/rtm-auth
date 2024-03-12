#[derive(Debug, thiserror::Error)]
pub enum UserRepositoryError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}
