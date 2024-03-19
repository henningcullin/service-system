use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};

use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{
    user::{TokenClaims, User},
    AppState, ResponseData,
    ResponseType::Fail,
};

pub async fn auth(
    cookie_jar: CookieJar,
    State(app_state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, (StatusCode, Json<ResponseData>)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let token = token.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ResponseData {
                status: Fail,
                message: "You are not logged in".to_string(),
            }),
        )
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(app_state.env.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| {
        eprintln!("Error decoding claims | auth::auth: {:?}", e);
        (
            StatusCode::UNAUTHORIZED,
            Json(ResponseData {
                status: Fail,
                message: "Invalid token".to_string(),
            }),
        )
    })?
    .claims;

    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ResponseData {
                status: Fail,
                message: "Invalid token".to_string(),
            }),
        )
    })?;

    let user = sqlx::query_as_unchecked!(User, "SELECT id, first_name, last_name, email, password, phone, CAST(role AS SIGNED) role, active, last_login FROM user WHERE id = ?", user_id)
        .fetch_optional(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error selecting user from database | auth::auth: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseData {
                status: Fail,
                message: "Error fetching user from database".to_owned(),
            }))
        })?;

    let user = user.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ResponseData {
                status: Fail,
                message: "The user belonging to this token no longer exists".to_string(),
            }),
        )
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

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
