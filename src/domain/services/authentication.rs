use async_trait::async_trait;

use crate::domain::models::LoginDto;

use super::AuthenticationServiceError;

type Result<T> = std::result::Result<T, AuthenticationServiceError>;

#[async_trait]
pub trait AuthenticationService {
    async fn login(&self, creds: LoginDto) -> Result<()>;
}
