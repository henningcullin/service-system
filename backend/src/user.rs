use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::{AppendHeaders, IntoResponse},
    Extension, Json,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand_core::{OsRng, RngCore};
use uuid::Uuid;

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

use validator::Validate;

use crate::{
    AppState, ResponseData,
    ResponseType::{Fail, Success},
};

#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq)]
#[repr(i32)]
pub enum UserRole {
    Super = 1,
    Administrator = 2,
    Basic = 3,
    Worker = 4,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: Option<String>,
    pub phone: Option<String>,
    pub role: UserRole,
    pub active: bool,
    pub last_login: Option<DateTime<Utc>>,
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
            active: self.active,
            last_login: self.last_login,
        }
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct FilteredUser {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub role: UserRole,
    pub active: bool,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct QueryUser {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginTokenClaims {
    pub sub: String,
    pub hash: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUser {
    pub first_name: String,
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
    pub role: UserRole,
    pub active: bool,
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
    pub role: Option<UserRole>,
    pub active: Option<bool>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginInternalUser {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginExternalUser {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct VerifyExternalUser {
    pub code: String,
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

pub async fn me(Extension(user): Extension<User>) -> Json<FilteredUser> {
    Json(user.to_filtered())
}

pub async fn details(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryUser>,
) -> Result<Json<FilteredUser>, (StatusCode, Json<ResponseData>)> {
    let user = sqlx::query_as_unchecked!(
        FilteredUser,
        "SELECT 
        id, 
        first_name, 
        last_name, 
        email,
        phone, 
        CAST(role AS SIGNED) role, 
        active,
        last_login 
        FROM user 
        WHERE id = ?",
        params.id
    )
        .fetch_one(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error executing query for user::details: {:?}", e);
            match e {
                sqlx::Error::RowNotFound => (
                    StatusCode::NOT_FOUND,
                    Json(ResponseData {
                        status: Fail,
                        message: "The specified user does not exist".to_owned(),
                    }),
                ),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseData {
                        status: Fail,
                        message: "Server error".to_owned(),
                    }),
                ),
            }
        })?;

    Ok(Json(user))
}

pub async fn index(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<FilteredUser>>, (StatusCode, Json<ResponseData>)> {
    let users: Vec<FilteredUser> = sqlx::query_as_unchecked!(
        FilteredUser,
        "SELECT 
        id, 
        first_name, 
        last_name, 
        email,
        phone, 
        CAST(role AS SIGNED) role,
        active,
        last_login 
        FROM user
        ",
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(|e| {
        eprintln!("Error executing query for user::index: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseData {
                status: Fail,
                message: "Could not retrieve the users from database".to_owned(),
            }),
        )
    })?;

    Ok(Json(users))
}

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<RegisterUser>,
) -> Result<(StatusCode, Json<UserResponse>), (StatusCode, Json<ResponseData>)> {
    match user.role {
        UserRole::Worker => {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ResponseData {
                    status: Fail,
                    message: "You don't have permission to add users".to_owned(),
                }),
            ));
        }
        UserRole::Basic => {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ResponseData {
                    status: Fail,
                    message: "You don't have permission to add users".to_owned(),
                }),
            ));
        }
        UserRole::Administrator => {
            if body.role == UserRole::Administrator || body.role == UserRole::Super {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ResponseData {
                        status: Fail,
                        message: format!("You can't create users with role {:?}", body.role),
                    }),
                ));
            }
        }
        UserRole::Super => {
            if body.role == UserRole::Super {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ResponseData {
                        status: Fail,
                        message: format!("You can't create users with role {:?}", body.role),
                    }),
                ));
            }
        }
    }

    body.validate().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ResponseData {
                status: Fail,
                message: "Invalid email".to_owned(),
            }),
        )
    })?;

    let user_exists: Option<bool> =
        sqlx::query_scalar_unchecked!(
            "SELECT EXISTS(SELECT 1 FROM user WHERE email = ?)",
            body.email.to_owned().to_ascii_lowercase()
        )
            .fetch_one(&app_state.db)
            .await
            .map_err(|e| {
                eprintln!("Error checking if user exist | user::create: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseData {
                        status: Fail,
                        message: "Database error".to_owned(),
                    }),
                )
            })?;

    if let Some(exists) = user_exists {
        if exists {
            return Err((
                StatusCode::CONFLICT,
                Json(ResponseData {
                    status: Fail,
                    message: "an User with that email already exists".to_owned(),
                }),
            ));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            eprintln!("Error hashing password | user::create: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseData {
                    status: Fail,
                    message: "Password error".to_owned(),
                }),
            )
        })
        .map(|hash| hash.to_string())?;

    let id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO user (id, first_name, last_name, email, password, phone, role, active) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        id,
        body.first_name.to_string(),
        body.last_name.to_string(),
        body.email,
        hashed_password,
        body.phone,
        body.role,
        body.active,
    )
    .execute(&app_state.db)
    .await
    .map_err(|e| {
        eprintln!("Error creating user | user::register_user: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseData {
            status: Fail,
            message: "Could not create user".to_owned(),
        }))
    })?;

    let user = User {
        id,
        first_name: body.first_name.to_string(),
        last_name: body.last_name.to_string(),
        email: body.email,
        password: Some(hashed_password),
        phone: body.phone,
        role: body.role,
        active: body.active,
        last_login: None,
    };

    Ok((
        StatusCode::CREATED,
        Json(UserResponse {
            status: "success".to_owned(),
            data: UserData {
                user: user.to_filtered(),
            },
        }),
    ))
}

pub async fn update(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateUser>,
) -> Result<StatusCode, (StatusCode, Json<ResponseData>)> {
    let target_user = sqlx::query_as_unchecked!(
        FilteredUser,
        "SELECT 
        id, 
        first_name, 
        last_name, 
        email,
        phone, 
        CAST(role AS SIGNED) role, 
        active,
        last_login 
        FROM user 
        WHERE id = ?",
        body.id
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(|e| {
        eprintln!("Error executing query for user::details: {:?}", e);
        match e {
            sqlx::Error::RowNotFound => (
                StatusCode::NOT_FOUND,
                Json(ResponseData {
                    status: Fail,
                    message: "The specified user does not exist".to_owned(),
                }),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseData {
                    status: Fail,
                    message: "Server error".to_owned(),
                }),
            ),
        }
    })?;

    match user.role {
        UserRole::Worker => {
            if body.password.is_some() {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ResponseData {
                        status: Fail,
                        message: "You can't set a password".to_owned(),
                    }),
                ));
            } // WORKERS CAN'T HAVE PASSWORD
            if user.id != body.id {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ResponseData {
                        status: Fail,
                        message: "You can't change other peoples information".to_owned(),
                    }),
                ));
            } // WORKER CAN'T CHANGE OTHERS INFO
            if body.email.is_some() {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ResponseData {
                        status: Fail,
                        message: "You can't change your email if you use it to login".to_owned(),
                    }),
                ));
            } // WORKER CAN'T CHANGE EMAIL
            if body.role.is_some() {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ResponseData {
                        status: Fail,
                        message: "You can't change your role".to_owned(),
                    }),
                ));
            } // WORKER CAN'T CHANGE ROLE
        }
        UserRole::Basic => {
            if body.password.is_some() {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ResponseData {
                        status: Fail,
                        message: "You can't change your password".to_owned(),
                    }),
                ));
            } // BASIC CAN'T CHANGE PASSWORD HERE
            if user.id != body.id {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ResponseData {
                        status: Fail,
                        message: "You can't change other peoples information".to_owned(),
                    }),
                ));
            } // BASIC CAN'T CHANGE OTHERS INFO
            if body.role.is_some() {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ResponseData {
                        status: Fail,
                        message: "You can't change your role".to_owned(),
                    }),
                ));
            } // BASIC CAN'T CHANGE OWN ROLE
        }
        UserRole::Administrator => {
            if body.password.is_some()
                && (target_user.role == UserRole::Administrator
                    || target_user.role == UserRole::Super)
                && user.id != body.id
            {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ResponseData {
                        status: Fail,
                        message: format!(
                            "You can't change the password of people with role {:?}",
                            target_user.role
                        ),
                    }),
                ));
            } // ADMINISTRATOR CAN'T CHANGE OTHER ADMINISTRATOR'S OR SUPER'S PASSWORD
            if body.role.is_some()
                && (target_user.role == UserRole::Administrator
                    || target_user.role == UserRole::Super)
            {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ResponseData {
                        status: Fail,
                        message: format!("You can't change role for {:?}", target_user.role),
                    }),
                ));
            } // ADMINISTRATOR CAN'T CHANGE ROLE TO ADMINISTRATOR OR SUPER
            if user.id == body.id && body.role.is_some() {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ResponseData {
                        status: Fail,
                        message: "You can't change your own role".to_owned(),
                    }),
                ));
            } // ADMINISTRATOR CAN'T CHANGE OWN ROLE
        }
        UserRole::Super => {
            if user.id == body.id && body.role.is_some() {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ResponseData {
                        status: Fail,
                        message: "You can't change your own role".to_owned(),
                    }),
                ));
            } // SUPER CAN'T CHANGE OWN ROLE
        }
    }

    body.validate().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ResponseData {
                status: Fail,
                message: "Invalid email".to_owned(),
            }),
        )
    })?;

    let hashed_password = match body.password {
        None => None,
        Some(password) => {
            let salt = SaltString::generate(&mut OsRng);
            let hashed_password = Argon2::default()
                .hash_password(password.as_bytes(), &salt)
                .map_err(|e| {
                    eprintln!("Error hashing password | user::create: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ResponseData {
                            status: Fail,
                            message: "Password error".to_owned(),
                        }),
                    )
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
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseData {
                status: Fail,
                message: "Could not update the user in the database".to_owned(),
            }),
        )
    })?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ResponseData {
                status: Fail,
                message: "The user was not found in the database".to_owned(),
            }),
        ))
    }
}

pub async fn login_internal(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<LoginInternalUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<ResponseData>)> {
    body.validate().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ResponseData {
                status: Fail,
                message: "Invalid email".to_owned(),
            }),
        )
    })?;

    let user = sqlx::query_as_unchecked!(User, "SELECT id, first_name, last_name, email, password, phone, CAST(role AS SIGNED) role, active, last_login FROM user WHERE email = ?", body.email.to_ascii_lowercase())
        .fetch_optional(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error retrieving user from database | user::login_internal: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseData {
                status: Fail,
                message: "Could not retrieve user from database".to_owned(),
            }))
        })?
        .ok_or_else(|| {
            (StatusCode::NOT_FOUND, Json(ResponseData {
                status: Fail,
                message: "No user with this email".to_owned(),
            }))
        })?;

    if !user.active {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ResponseData {
                status: Fail,
                message: "Your account is deactivated".to_string(),
            }),
        ));
    }

    match user.role {
        UserRole::Super => {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ResponseData {
                    status: Fail,
                    message: "You don't use a password to login".to_string(),
                }),
            ));
        }
        UserRole::Worker => {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ResponseData {
                    status: Fail,
                    message: "You don't use a password to login".to_string(),
                }),
            ));
        }
        _ => {}
    }

    let password = match user.password {
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ResponseData {
                    status: Fail,
                    message: "You need to provide a password".to_string(),
                }),
            ));
        }
        Some(password) => password,
    };

    let is_valid = match PasswordHash::new(&password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ResponseData {
                status: Fail,
                message: "Incorrect password".to_owned(),
            }),
        ));
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
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseData {
                status: Fail,
                message: "Could create your token".to_owned(),
            }),
        )
    })?;

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::minutes(app_state.env.jwt_expires_in))
        .same_site(SameSite::Lax)
        .http_only(true)
        .to_string();

    Ok((
        AppendHeaders([(header::SET_COOKIE, cookie)]),
        Json(ResponseData {
            status: Success,
            message: "Successfully logged in".to_string(),
        }),
    ))
}

pub async fn login_initiate(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<LoginExternalUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<ResponseData>)> {
    body.validate().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ResponseData {
                status: Fail,
                message: "Invalid email".to_owned(),
            }),
        )
    })?;

    let user = sqlx::query_as_unchecked!(User, "SELECT id, first_name, last_name, email, password, phone, CAST(role AS SIGNED) role, active, last_login FROM user WHERE email = ?", body.email.to_ascii_lowercase())
        .fetch_optional(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error retrieving user from database | user::login_internal: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseData {
                status: Fail,
                message: "Could not retrieve user from database".to_owned(),
            }))
        })?
        .ok_or_else(|| {
            (StatusCode::NOT_FOUND, Json(ResponseData {
                status: Fail,
                message: "No user with this email".to_owned(),
            }))
        })?;

    if !user.active {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ResponseData {
                status: Fail,
                message: "Your account is deactivated".to_string(),
            }),
        ));
    }
    
    if user.role == UserRole::Administrator || user.role == UserRole::Basic {
        return 
        Err((
            StatusCode::OK, 
            Json(ResponseData {
                status: Success,
                message: "password".to_owned(),
            })
        ));
    }


    let mut rng = OsRng;
    let mut buffer = [0u8; 3];

    rng.fill_bytes(&mut buffer);

    let code = buffer
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<String>(); // EMAIL THIS TO USER

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(code.as_bytes(), &salt)
        .map_err(|e| {
            eprintln!("Error creating hash | user::login_external: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseData {
                    status: Fail,
                    message: "Password error".to_owned(),
                }),
            )
        })
        .map(|hash| hash.to_string())?;

    println!("Code: {}", code);

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(5)).timestamp() as usize;

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
        eprintln!(
            "Error creating token for user | user::login_external: {:?}",
            e
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseData {
                status: Fail,
                message: "Could create your token".to_owned(),
            }),
        )
    })?;

    let cookie = Cookie::build(("auth_token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::minutes(5))
        .same_site(SameSite::Lax)
        .http_only(true)
        .to_string();

    Ok((
        AppendHeaders([(header::SET_COOKIE, cookie)]),
        Json(ResponseData {
            status: Success,
            message: "code".to_string(),
        }),
    ))
}

pub async fn verify_external(
    State(app_state): State<Arc<AppState>>,
    cookie_jar: CookieJar,
    Json(body): Json<VerifyExternalUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<ResponseData>)> {
    let token = cookie_jar
        .get("auth_token")
        .map(|cookie| cookie.value().to_string());

    let token = token.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ResponseData {
                status: Fail,
                message: "You need to enter your email first".to_string(),
            }),
        )
    })?;

    let claims = decode::<LoginTokenClaims>(
        &token,
        &DecodingKey::from_secret(app_state.env.jwt_pwl_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| {
        eprintln!("Error decoding claims | user::verify_external: {:?}", e);
        (
            StatusCode::UNAUTHORIZED,
            Json(ResponseData {
                status: Fail,
                message: "Invalid token".to_string(),
            }),
        )
    })?
    .claims;

    let is_valid = match PasswordHash::new(&claims.hash) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.code.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ResponseData {
                status: Fail,
                message: "Incorrect code".to_owned(),
            }),
        ));
    }

    let user = sqlx::query_as_unchecked!(User, "SELECT id, first_name, last_name, email, password, phone, CAST(role AS SIGNED) role, active, last_login FROM user WHERE email = ?", claims.sub.to_ascii_lowercase())
        .fetch_optional(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error retrieving user from database | user::login_internal: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseData {
                status: Fail,
                message: "Could not retrieve user from database".to_owned(),
            }))
        })?
        .ok_or_else(|| {
            (StatusCode::NOT_FOUND, Json(ResponseData {
                status: Fail,
                message: "No user with this email".to_owned(),
            }))
        })?;

    if !user.active {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ResponseData {
                status: Fail,
                message: "Your account is deactivated".to_string(),
            }),
        ));
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
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseData {
                status: Fail,
                message: "Could create your token".to_owned(),
            }),
        )
    })?;

    let token_cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::minutes(app_state.env.jwt_expires_in))
        .same_site(SameSite::Lax)
        .http_only(true)
        .to_string();

    let auth_token_cookie = Cookie::build(("auth_token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .to_string();

    Ok((
        AppendHeaders([
            (header::SET_COOKIE, token_cookie),
            (header::SET_COOKIE, auth_token_cookie),
        ]),
        Json(ResponseData {
            status: Success,
            message: "Successfully logged in".to_string(),
        }),
    ))
}

pub async fn logout() -> Result<impl IntoResponse, (StatusCode, Json<ResponseData>)> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .to_string();

    Ok((
        AppendHeaders([(header::SET_COOKIE, cookie)]),
        Json(ResponseData {
            status: Success,
            message: "Successfully logged out".to_string(),
        }),
    ))
}
