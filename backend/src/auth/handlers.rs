use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{AppendHeaders, IntoResponse},
    Json,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand_core::{OsRng, RngCore};
use sqlx::query;
use validator::Validate;

use crate::{
    auth::models::{LoginToken, TokenClaims},
    users::roles::models::Role,
    utils::errors::{ApiError, ForbiddenReason},
    AppState,
};

use super::models::{LoginEmail, LoginKind, LoginOTPUser, LoginPasswordUser};

pub async fn login_initiate(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<LoginEmail>,
) -> Result<impl IntoResponse, ApiError> {
    body.validate().map_err(ApiError::from)?;

    let user = query!(
        r#"
        SELECT
            u.id,
            u.email,
            (
                r.id,
                r.name,
                r.level,
                r.has_password,
                r.user_view,
                r.user_create,
                r.user_edit,
                r.user_delete,
                r.machine_view,
                r.machine_create,
                r.machine_edit,
                r.machine_delete,
                r.task_view,
                r.task_create,
                r.task_edit,
                r.task_delete,
                r.report_view,
                r.report_create,
                r.report_edit,
                r.report_delete,
                r.facility_view,
                r.facility_create,
                r.facility_edit,
                r.facility_delete
            ) AS "role!: Role",
            u.active,
            u.last_login
        FROM
            users u
        INNER JOIN
            roles r
        ON
            u.role = r.id
        WHERE
            u.email = $1
        "#,
        body.email.to_lowercase()
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    if !user.active {
        Err(ApiError::Forbidden(ForbiddenReason::AccountDeactivated))?
    }

    if user.role.has_password {
        let cookie = Cookie::build(("auth_token", ""))
            .path("/")
            .max_age(time::Duration::seconds(-1))
            .same_site(SameSite::Lax)
            .http_only(true)
            .to_string();

        return Ok((
            AppendHeaders([(header::SET_COOKIE, cookie)]),
            Json(LoginKind::Password),
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
        .map_err(ApiError::from)
        .map(|hash| hash.to_string())?;

    println!("Code: {code}"); // REPLACE WITH "EMAIL TO" IMPLEMENTATION

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(5)).timestamp() as usize;

    let claims = LoginToken {
        sub: body.email,
        iat,
        exp,
        hash,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(app_state.env.jwt_pwl_secret.as_ref()),
    )
    .map_err(ApiError::from)?;

    let cookie = Cookie::build(("auth_token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::minutes(5))
        .same_site(SameSite::Lax)
        .http_only(true)
        .to_string();

    Ok((
        AppendHeaders([(header::SET_COOKIE, cookie)]),
        Json(LoginKind::OTP),
    ))
}

pub async fn login_password(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<LoginPasswordUser>,
) -> Result<impl IntoResponse, ApiError> {
    body.validate().map_err(ApiError::from)?;

    let user = query!(
        r#"
            SELECT
                u.id,
                u.email,
                u.password,
                (
                    r.id,
                    r.name,
                    r.level,
                    r.has_password,
                    r.user_view,
                    r.user_create,
                    r.user_edit,
                    r.user_delete,
                    r.machine_view,
                    r.machine_create,
                    r.machine_edit,
                    r.machine_delete,
                    r.task_view,
                    r.task_create,
                    r.task_edit,
                    r.task_delete,
                    r.report_view,
                    r.report_create,
                    r.report_edit,
                    r.report_delete,
                    r.facility_view,
                    r.facility_create,
                    r.facility_edit,
                    r.facility_delete
                ) AS "role!: Role",
                u.active
            FROM
                users u
            INNER JOIN
                roles r
            ON
                u.role = r.id
            WHERE
                u.email = $1
        "#,
        body.email.to_lowercase()
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    if !user.role.has_password {
        return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission));
    }

    if !user.active {
        return Err(ApiError::Forbidden(ForbiddenReason::AccountDeactivated));
    }

    let stored_password = match user.password {
        Some(pw) => pw,
        None => {
            return Err(ApiError::GeneralOversight(format!(
                "User with has_password true and null password, user: {user:?}"
            )))
        }
    };

    let passwords_match = match PasswordHash::new(&stored_password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !passwords_match {
        return Err(ApiError::Forbidden(ForbiddenReason::IncorrectPassword));
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
    .map_err(ApiError::from)?;

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::minutes(app_state.env.jwt_expires_in))
        .same_site(SameSite::Lax)
        .http_only(true)
        .to_string();

    Ok((
        AppendHeaders([(header::SET_COOKIE, cookie)]),
        StatusCode::OK,
    ))
}

pub async fn login_otp(
    State(app_state): State<Arc<AppState>>,
    cookie_jar: CookieJar,
    Json(body): Json<LoginOTPUser>,
) -> Result<impl IntoResponse, ApiError> {
    let token = cookie_jar
        .get("auth_token")
        .map(|cookie| cookie.value().to_string());

    let token = token.ok_or_else(|| ApiError::Unauthorized)?;

    let claims = decode::<LoginToken>(
        &token,
        &DecodingKey::from_secret(app_state.env.jwt_pwl_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(ApiError::from)?
    .claims;

    let codes_match = match PasswordHash::new(&claims.hash) {
        Ok(stored_hash) => Argon2::default()
            .verify_password(body.code.as_bytes(), &stored_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !codes_match {
        return Err(ApiError::Forbidden(ForbiddenReason::IncorrectCode));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(app_state.env.jwt_expires_in)).timestamp() as usize;
    let claims = TokenClaims {
        sub: claims.sub,
        iat,
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(app_state.env.jwt_secret.as_ref()),
    )
    .map_err(ApiError::from)?;

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

    Ok(AppendHeaders([
        (header::SET_COOKIE, token_cookie),
        (header::SET_COOKIE, auth_token_cookie),
    ]))
}
