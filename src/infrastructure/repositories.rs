use crate::domain::{
    models::{value_objects::email::Email, CreateUser},
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
    async fn create_user(&self, user: CreateUser) -> Result<(), UserRepositoryError> {
        todo!()
    }

    async fn is_email_unique(&self, email: &Email) -> Result<bool, UserRepositoryError> {
        todo!()
    }
}
