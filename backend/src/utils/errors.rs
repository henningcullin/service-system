use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use jsonwebtoken::errors::Error as JWTError;
use sqlx::Error as SqlxError;
use tracing::error;
use uuid::Error as UuidError;

#[derive(Debug)]
pub enum ApiError {
    Unauthorized,
    UuidError(UuidError),
    InvalidToken(JWTError),
    DatabaseError(SqlxError),
}

impl From<UuidError> for ApiError {
    fn from(err: UuidError) -> Self {
        Self::UuidError(err)
    }
}

impl From<JWTError> for ApiError {
    fn from(err: JWTError) -> Self {
        Self::InvalidToken(err)
    }
}

impl From<SqlxError> for ApiError {
    fn from(err: SqlxError) -> Self {
        Self::DatabaseError(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response<Body> {
        let error_message = format!("{:?}", self);
        error!(error_message);

        match self {
            Self::UuidError(_) => {
                let message = "Internal server error";
                let response_body = Json(message);
                (StatusCode::INTERNAL_SERVER_ERROR, response_body).into_response()
            }
            Self::Unauthorized => {
                let message = "You are not logged in";
                let response_body = Json(message);
                (StatusCode::UNAUTHORIZED, response_body).into_response()
            }
            Self::InvalidToken(_) => {
                let message = "Invalid token"; // You can customize this message
                let response_body = Json(message);
                (StatusCode::UNAUTHORIZED, response_body).into_response()
            }
            Self::DatabaseError(error) => match error {
                SqlxError::RowNotFound => {
                    let message = "Not found";
                    let response_body = Json(message);
                    (StatusCode::NOT_FOUND, response_body).into_response()
                }
                _ => {
                    let message = "Database error";
                    let response_body = Json(message);
                    (StatusCode::INTERNAL_SERVER_ERROR, response_body).into_response()
                }
            },
        }
    }
}
