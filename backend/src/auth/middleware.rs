use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{header, Request},
    middleware::Next,
    response::Response,
};

use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use sqlx::query_as;
use uuid::Uuid;

use crate::{
    auth::models::TokenClaims,
    machines::facilities::Facility,
    user_from_id,
    users::{models::User, roles::models::Role},
    utils::errors::{ApiError, ForbiddenReason},
    AppState,
};

pub async fn auth(
    cookie_jar: CookieJar,
    State(app_state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, ApiError> {
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

    let token = token.ok_or_else(|| ApiError::Unauthorized)?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(app_state.env.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| ApiError::Unauthorized)?
    .claims;

    let user_id = Uuid::parse_str(&claims.sub).map_err(ApiError::from)?;

    let user: User = user_from_id!(user_id)
        .fetch_one(&app_state.db)
        .await
        .map_err(ApiError::from)?;

    if !user.active {
        Err(ApiError::Forbidden(ForbiddenReason::AccountDeactivated))?
    }

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
