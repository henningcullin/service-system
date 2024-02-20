use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{extract::State, http::StatusCode, Extension, Json};
use chrono::{DateTime, Utc};
use rand_core::OsRng;
use uuid::Uuid;

use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Type};

use validator::Validate;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq)]
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
    pub phone: Option<String>,
    pub role: UserRole,
    pub last_login: Option<DateTime<Utc>>
}

impl User {
    pub fn to_filtered(self) -> FilteredUser {
        FilteredUser {
            id: self.id,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            phone: self.phone,
            role: self.role,
            last_login: self.last_login
        }
    } 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub role: i32
}

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUserSchema {
    pub first_name: String,
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub role: UserRole
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginUserSchema {
    #[validate(email)]
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub role: UserRole,
    pub last_login: Option<DateTime<Utc>>
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

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<RegisterUserSchema>,
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, Json<serde_json::Value>)> {

    if user.role == UserRole::Worker || user.role == UserRole::Basic {
        let error_response = serde_json::json!({
            "status": "Fail",
            "message": "You don't have permission to add users"
        });
        return Err((StatusCode::FORBIDDEN, Json(error_response)));
    }

    body.validate()
        .map_err(|e| {
            eprintln!("Error validating email | user::create: {:?}", e);
            let error_response = serde_json::json!({
                "status": "Fail",
                "message": "Invalid email"
            });
            (StatusCode::BAD_REQUEST, Json(error_response))
        })?;

    let user_exists: Option<bool> = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM user WHERE email = ?)")
        .bind(body.email.to_owned().to_ascii_lowercase())
        .fetch_one(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error checking if user exist | user::create: {:?}", e);
            let error_response = serde_json::json!({
                "status": "Fail",
                "message": "Database error",
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    if let Some(exists) = user_exists {
        if exists {
            let error_response = serde_json::json!({
                "status": "Fail",
                "message": "an User with that email already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            eprintln!("Error hashing password | user::create: {:?}", e);
            let error_response = serde_json::json!({
                "status": "Fail",
                "message": "Password error",
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    let id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO user (id, first_name, last_name, email, password, phone, role) VALUES (?, ?, ?, ?, ?, ?, ?)",
        id,
        body.first_name.to_string(),
        body.last_name.to_string(),
        body.email,
        hashed_password,
        body.phone,
        body.role
    )
    .execute(&app_state.db)
    .await
    .map_err(|e| {
        eprintln!("Error creating user | user::register_user: {:?}", e);
        let error_response = serde_json::json!({
            "status": "Fail",
            "message": "Could not create user",
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let user = User {
        id,
        first_name: body.first_name.to_string(),
        last_name: body.last_name.to_string(),
        email: body.email,
        password: hashed_password,
        phone: body.phone,
        role: body.role,
        last_login: None
    };

    let user_response = UserResponse {
        status: "Success".to_owned(),
        data: UserData { user: user.to_filtered() }
    };

    Ok(
        (
            StatusCode::CREATED,
            Json(user_response)
        )
    )
}