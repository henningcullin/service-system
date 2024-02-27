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

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginTokenClaims {
    pub sub: String,
    pub hash: String,
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
    pub password: Option<String>,
    pub phone: Option<String>,
    pub role: Option<UserRole>
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginInternalUser {
    #[validate(email)]
    pub email: String,
    pub password: String
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginExternalUser {
    #[validate(email)]
    pub email: String
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

    let target_user = sqlx::query_as::<_, FilteredUser>(
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
        .bind(body.id)
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

    match user.role {
        UserRole::Worker => {
            if body.password.is_some() {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: "You can't set a password".to_owned()})));
            } // WORKERS CAN'T HAVE PASSWORD
            if user.id != body.id {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: "You can't change other peoples information".to_owned()})));
            } // WORKER CAN'T CHANGE OTHERS INFO
            if body.email.is_some() {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: "You can't change your email if you use it to login".to_owned()})));
            } // WORKER CAN'T CHANGE EMAIL
            if body.role.is_some() {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: "You can't change your role".to_owned()})));
            } // WORKER CAN'T CHANGE ROLE
        },
        UserRole::Basic => {
            if body.password.is_some() {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: "You can't change your password".to_owned()})));
            } // BASIC CAN'T CHANGE PASSWORD HERE
            if user.id != body.id {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: "You can't change other peoples information".to_owned()})));
            } // BASIC CAN'T CHANGE OTHERS INFO
            if body.role.is_some() {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: "You can't change your role".to_owned()})));
            } // BASIC CAN'T CHANGE OWN ROLE
        },
        UserRole::Administrator => {
            if body.password.is_some() && (target_user.role == UserRole::Administrator || target_user.role == UserRole::Super) && user.id != body.id {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: format!("You can't change the password of people with role {:?}", target_user.role)})));
            } // ADMINISTRATOR CAN'T CHANGE OTHER ADMINISTRATOR'S OR SUPER'S PASSWORD
            if body.role.is_some() && (target_user.role == UserRole::Administrator || target_user.role == UserRole::Super) {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: format!("You can't change role for {:?}", target_user.role)})));
            } // ADMINISTRATOR CAN'T CHANGE ROLE TO ADMINISTRATOR OR SUPER
            if user.id == body.id && body.role.is_some() {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: "You can't change your own role".to_owned()})));
            } // ADMINISTRATOR CAN'T CHANGE OWN ROLE
        }
        UserRole::Super => {
            if user.id == body.id && body.role.is_some() {
                return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {status: "fail", message: "You can't change your own role".to_owned()})));
            } // SUPER CAN'T CHANGE OWN ROLE
        }
    }

    body.validate()
        .map_err(|_| {
            (StatusCode::BAD_REQUEST, Json(ErrorResponse {
                status: "fail",
                message: "Invalid email".to_owned()
            }))
        })?;

    let hashed_password = match body.password {
        None => None,
        Some(password) => {
            let salt = SaltString::generate(&mut OsRng);
            let hashed_password = Argon2::default()
                .hash_password(password.as_bytes(), &salt)
                .map_err(|e| {
                    eprintln!("Error hashing password | user::create: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                        status: "fail",
                        message: "Password error".to_owned(),
                    }))
                })
                .map(|hash| hash.to_string())?;
            Some(hashed_password)
        }
    };

    let result = sqlx::query!(
        "UPDATE user SET 
        first_name = COALESCE(?, first_name), 
        last_name = COALESCE(?, last_name), 
        email = COALESCE(?, email), 
        password = COALESCE(?, password), 
        phone = COALESCE(?, phone), 
        role = COALESCE(?, role) 
        WHERE id = ?",
        body.first_name,
        body.last_name,
        body.email,
        hashed_password,
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
    Json(body): Json<LoginInternalUser>,
) -> Result<Response<String>, (StatusCode, Json<ErrorResponse>)> {
    body.validate()
        .map_err(|_| {
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
            eprintln!("Error retrieving user from database | user::login_internal: {:?}", e);
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
    let exp = (now + chrono::Duration::minutes(app_state.env.jwt_expires_in)).timestamp() as usize;
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

    let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
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

pub async fn login_external(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<LoginExternalUser>
) -> Result<Response<String>, (StatusCode, Json<ErrorResponse>)> {
    body.validate()
        .map_err(|_| {
            (StatusCode::BAD_REQUEST, Json(ErrorResponse {
                status: "fail",
                message: "Invalid email".to_owned()
            }))
        })?;

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(body.email.as_bytes(), &salt)
        .map_err(|e| {
            eprintln!("Error creating hash | user::login_external: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                status: "fail",
                message: "Password error".to_owned(),
            }))
        })
        .map(|hash| hash.to_string())?;
    
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(app_state.env.jwt_expires_in)).timestamp() as usize;

    let claims: LoginTokenClaims = LoginTokenClaims {
        sub: body.email,
        hash,
        iat,
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(app_state.env.jwt_pwl_secret.as_ref()),
    )
        .map_err(|e| {
            eprintln!("Error creating token for user | user::login_external: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                status: "fail",
                message: "Could create your token".to_owned(),
            }))
        })?;

    let cookie = Cookie::build(("auth_token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie
            .to_string()
            .parse()
            .map_err(|e| {
                eprintln!("Error creating cookie | user::login_external: {:?}", e);
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
