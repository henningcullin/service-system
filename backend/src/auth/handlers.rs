use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{
    extract::State,
    http::header,
    response::{AppendHeaders, IntoResponse},
    Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::{OsRng, RngCore};
use sqlx::query;
use validator::Validate;

use crate::{
    auth::models::LoginToken,
    users::roles::models::Role,
    utils::errors::{ApiError, ForbiddenReason},
    AppState,
};

use super::models::{LoginEmail, LoginKind};

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
        body.email
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
