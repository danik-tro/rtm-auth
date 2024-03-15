use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::value_objects::email::Email;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct CreateUserDto {
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

pub struct CreateUser {
    pub id: Uuid,
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
}

#[derive(Debug)]
pub struct LoginDto {
    pub email: Email,
    pub password_hash: String,
}
