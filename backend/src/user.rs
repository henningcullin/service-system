use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{extract::{Query, State}, http::{header, Response, StatusCode}, Extension, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use serde_json::json;
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

#[derive(Debug, Deserialize)]
pub struct QueryUser {
    pub id: Uuid
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize
}

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUser {
    pub first_name: String,
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub role: UserRole
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateUser {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub phone: Option<String>,
    pub role: Option<UserRole>
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginUser {
    #[validate(email)]
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, FromRow)]
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

pub async fn details(
    State(app_state): State<Arc<AppState>>,
    Query(querys): Query<QueryUser>
) -> Result<Json<FilteredUser>, (StatusCode, Json<ErrorResponse>)> {

    let user = sqlx::query_as::<_, FilteredUser>(
        "SELECT 
        id, 
        first_name, 
        last_name, 
        email,
        phone, 
        CAST(role AS SIGNED) role, 
        last_login 
        FROM user 
        WHERE id = ?"
    )
        .bind(querys.id)
        .fetch_one(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error executing query for user::details: {:?}", e);
            match e {
                sqlx::Error::RowNotFound => {
                    (StatusCode::NOT_FOUND, Json(ErrorResponse {
                        status: "fail",
                        message: "The specified user does not exist".to_owned()
                    }))
                },
                _ => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                        status: "fail",
                        message: "Server error".to_owned()
                    }))
                }
            }
        })?;

    Ok(Json(user)) 

} 

pub async fn index(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<FilteredUser>>, (StatusCode, Json<ErrorResponse>)> {
    let users: Vec<FilteredUser> = sqlx::query_as::<_, FilteredUser>(
    "SELECT 
        id, 
        first_name, 
        last_name, 
        email,
        phone, 
        CAST(role AS SIGNED) role, 
        last_login 
        FROM user
        "
    )
        .fetch_all(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error executing query for user::index: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                status: "fail",
                message: "Could not retrieve the users from database".to_owned()
            }))
        })?;
    
        Ok(Json(users))
}

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<RegisterUser>,
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, Json<ErrorResponse>)> {

    if user.role == UserRole::Worker || user.role == UserRole::Basic {
        return Err((StatusCode::FORBIDDEN, Json(ErrorResponse{
            status: "fail",
            message: "You don't have permission to add users".to_owned()
        })));
    }

    if body.role == UserRole::Super || body.role == UserRole::Administrator {
        return Err((StatusCode::FORBIDDEN, Json(ErrorResponse{
            status: "fail",
            message: format!("You can't create users with role {:?}", body.role)
        })));
    }

    body.validate()
        .map_err(|_| {
            (StatusCode::BAD_REQUEST, Json(ErrorResponse {
                status: "fail",
                message: "Invalid email".to_owned()
            }))
        })?;

    let user_exists: Option<bool> = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM user WHERE email = ?)")
        .bind(body.email.to_owned().to_ascii_lowercase())
        .fetch_one(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error checking if user exist | user::create: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse{
                status: "fail",
                message: "Database error".to_owned(),
            }))
        })?;

    if let Some(exists) = user_exists {
        if exists {
            return Err((StatusCode::CONFLICT, Json(ErrorResponse {
                status: "fail",
                message: "an User with that email already exists".to_owned(),
            })));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            eprintln!("Error hashing password | user::create: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                status: "fail",
                message: "Password error".to_owned(),
            }))
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
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
            status: "fail",
            message: "Could not create user".to_owned(),
        }))
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

    Ok(
        (
            StatusCode::CREATED,
            Json(UserResponse {
                status: "success".to_owned(),
                data: UserData { user: user.to_filtered() }
            })
        )
    )
}

pub async fn update(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateUser>
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {

    match user.role {
        UserRole::Basic => {
            if user.id != body.id {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: "You can't change other peoples information".to_owned()})));
            }
            if body.role.is_some() {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: "You can't change your role".to_owned()})));
            }
        },
        UserRole::Worker => {
            if user.id != body.id {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: "You can't change other peoples information".to_owned()})));
            }
            if body.email.is_some() {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: "You can't change your email if you use it to login".to_owned()})));
            }
            if body.role.is_some() {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: "You can't change your role".to_owned()})));
            }
        },
        UserRole::Administrator => {
            if body.role.is_some() && (body.role == Some(UserRole::Administrator) || body.role == Some(UserRole::Super)) {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: format!("You can't set role to {:?}", body.role)})));
            }
        }
        _ => {}
    }

    body.validate()
        .map_err(|_| {
            (StatusCode::BAD_REQUEST, Json(ErrorResponse {
                status: "fail",
                message: "Invalid email".to_owned()
            }))
        })?;

    let result = sqlx::query!(
        "UPDATE user SET 
        first_name = COALESCE(?, first_name), 
        last_name = COALESCE(?, last_name), 
        email = COALESCE(?, email), 
        phone = COALESCE(?, phone), 
        role = COALESCE(?, role) 
        WHERE id = ?",
        body.first_name,
        body.last_name,
        body.email,
        body.phone,
        body.role,
        body.id
    )
    .execute(&app_state.db)
    .await
    .map_err(|e| {
        eprintln!("Error executing update for user::update: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
            status: "fail",
            message: "Could not update the user in the database".to_owned()
        }))
    })?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, Json(ErrorResponse {
            status: "fail",
            message: "The user was not found in the database".to_owned()
        })))
    }

}

pub async fn login_internal(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<LoginUser>,
) -> Result<Response<String>, (StatusCode, Json<ErrorResponse>)> {
    body.validate()
        .map_err(|e| {
            eprintln!("Error validating email | user::login_user: {:?}", e);
            (StatusCode::BAD_REQUEST, Json(ErrorResponse {
                status: "fail",
                message: "Invalid email".to_owned()
            }))
        })?;

    let user = sqlx::query_as::<_, User>("SELECT id, first_name, last_name, email, password, phone, CAST(role AS SIGNED) role, last_login FROM user WHERE email = ?")
    .bind(body.email.to_ascii_lowercase())
    .fetch_optional(&app_state.db)
    .await
    .map_err(|e| {
        eprintln!("Error retrieving user from database | user::login_user: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
            status: "fail",
            message: "Could not retrieve user from database".to_owned(),
        }))
    })?
    .ok_or_else(|| {
        (StatusCode::NOT_FOUND, Json(ErrorResponse {
            status: "fail",
            message: "No user with this email".to_owned(),
        }))
    })?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            status: "fail",
            message: "Incorrect password".to_owned()
        })));
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
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
            status: "fail",
            message: "Could create your token".to_owned(),
        }))
    })?;

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    let mut response = Response::new(json!({"status": "success", "token": token, "id": user.id}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie
            .to_string()
            .parse()
            .map_err(|e| {
                eprintln!("Error creating cookie | user::login_user: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                    status: "fail",
                    message: "Could not create a cookie".to_owned(),
                }))
        })?);
    Ok(response)
}

pub async fn logout() -> Result<Response<String>, (StatusCode, Json<ErrorResponse>)> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    let mut response = Response::new(json!({"status": "success", "message": "Successfully logged out"}).to_string());

    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie
            .to_string()
            .parse()
            .map_err(|e| {
                eprintln!("Error creating cookie | user::logout: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                    status: "fail",
                    message: "Could not create a cookie".to_owned(),
                }))
            })?);
    Ok(response)
}
