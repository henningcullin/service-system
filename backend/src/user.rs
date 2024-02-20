use chrono::{DateTime, Utc};
use uuid::Uuid;

use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Type};

use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
#[repr(i32)]
pub enum UserRole {
    Super = 1,
    Administrator = 2,
    Basic = 3,
    Worker = 4
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub role: UserRole,
    pub last_login: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize
}

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUserSchema {
    pub first_name: String,
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub phone: String
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub role: UserRole,
    pub last_login: DateTime<Utc>
}

#[derive(Debug, Serialize)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}