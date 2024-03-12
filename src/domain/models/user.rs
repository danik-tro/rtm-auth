use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct CreateUserDto {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
}

#[derive(Debug)]
pub struct LoginDto {
    pub email: String,
    pub password_hash: String,
}
