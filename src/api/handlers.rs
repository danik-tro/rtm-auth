use super::ApiError;

pub mod health_check;
mod mappings;
pub mod registration;

pub type ApiResult<T> = Result<T, ApiError>;
