use crate::domain::{
    models::CreateUserDto,
    repositories::{UserRepository, UserRepositoryError},
};
use async_trait::async_trait;

pub struct UserSqlxRepository {
    pool: sqlx::PgPool,
}

impl UserSqlxRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserSqlxRepository {
    async fn create_user(&self, user: CreateUserDto) -> Result<(), UserRepositoryError> {
        todo!()
    }

    // TODO: replace email type with specialized type like Email
    async fn is_email_unique(&self, email: String) -> Result<bool, UserRepositoryError> {
        todo!()
    }
}
