use crate::domain::{
    models::{value_objects::email::Email, CreateUser},
    repositories::{UserRepository, UserRepositoryError},
};
use async_trait::async_trait;

#[allow(unused)]
pub struct UserSqlxRepository {
    pool: sqlx::PgPool,
}

impl UserSqlxRepository {
    #[must_use]
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserSqlxRepository {
    async fn create_user(&self, _user: CreateUser) -> Result<(), UserRepositoryError> {
        todo!()
    }

    async fn is_email_unique(&self, _email: &Email) -> Result<bool, UserRepositoryError> {
        todo!()
    }
}
