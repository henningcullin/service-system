use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{extract::State, http::{header, Response, StatusCode}, response::IntoResponse, Extension, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use uuid::Uuid;

use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Type};

use validator::Validate;

use crate::{AppState, ErrorResponse};

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
    pub exp: usize
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
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, Json<ErrorResponse>)> {

    if user.role == UserRole::Worker || user.role == UserRole::Basic {
        let error_response = ErrorResponse{
            status: "fail",
            message: "You don't have permission to add users".to_owned()
        };
        return Err((StatusCode::FORBIDDEN, Json(error_response)));
    }

    body.validate()
        .map_err(|e| {
            eprintln!("Error validating email | user::create: {:?}", e);
            let error_response = ErrorResponse {
                status: "fail",
                message: "Invalid email".to_owned()
            };
            (StatusCode::BAD_REQUEST, Json(error_response))
        })?;

    let user_exists: Option<bool> = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM user WHERE email = ?)")
        .bind(body.email.to_owned().to_ascii_lowercase())
        .fetch_one(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error checking if user exist | user::create: {:?}", e);
            let error_response = ErrorResponse{
                status: "fail",
                message: "Database error".to_owned(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    if let Some(exists) = user_exists {
        if exists {
            let error_response = ErrorResponse {
                status: "fail",
                message: "an User with that email already exists".to_owned(),
            };
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            eprintln!("Error hashing password | user::create: {:?}", e);
            let error_response = ErrorResponse {
                status: "fail",
                message: "Password error".to_owned(),
            };
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
        let error_response = ErrorResponse {
            status: "fail",
            message: "Could not create user".to_owned(),
        };
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
        status: "success".to_owned(),
        data: UserData { user: user.to_filtered() }
    };

    Ok(
        (
            StatusCode::CREATED,
            Json(user_response)
        )
    )
}

pub async fn login_user(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    body.validate()
        .map_err(|e| {
            eprintln!("Error validating email | user::login_user: {:?}", e);
            let error_response = ErrorResponse {
                status: "fail",
                message: "Invalid email".to_owned()
            };
            (StatusCode::BAD_REQUEST, Json(error_response))
        })?;

    let user = sqlx::query_as::<_, User>("SELECT id, first_name, last_name, email, password, phone, CAST(role AS SIGNED) role, last_login FROM user WHERE email = ?")
    .bind(body.email.to_ascii_lowercase())
    .fetch_optional(&app_state.db)
    .await
    .map_err(|e| {
        eprintln!("Error retrieving user from database | user::login_user: {:?}", e);
        let error_response = ErrorResponse {
            status: "fail",
            message: "Could not retrieve user from database".to_owned(),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?
    .ok_or_else(|| {
        let error_response = ErrorResponse {
            status: "fail",
            message: "No user with this email".to_owned(),
        };
        (StatusCode::NOT_FOUND, Json(error_response))
    })?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = ErrorResponse {
            status: "fail",
            message: "Incorrect password".to_owned()
        };
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(app_state.env.jwt_secret.as_ref()),
    )
    .map_err(|e| {
        eprintln!("Error creating token for user | user::login_user: {:?}", e);
        let error_response = ErrorResponse {
            status: "fail",
            message: "Could create your token".to_owned(),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    let mut response = Response::new(serde_json::json!({"status": "success", "token": token}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().map_err(|e| {
            eprintln!("Error creating cookie | user::login_user: {:?}", e);
            let error_response = ErrorResponse {
                status: "fail",
                message: "Could not create a cookie".to_owned(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?);
    Ok(response)
}