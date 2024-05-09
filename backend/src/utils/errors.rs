use argon2::password_hash::Error as Argon2Error;
use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use jsonwebtoken::errors::Error as JWTError;
use sqlx::Error as SqlxError;
use tracing::{error, warn};
use uuid::Error as UuidError;
use validator::ValidationErrors as ValidationError;

#[derive(Debug)]
pub enum ApiError {
    Forbidden(ForbiddenReason),
    Conflict(ConflictReason),
    InputInvalid(InputInvalidReason),
    Unauthorized,
    UuidError(UuidError),
    PasswordError(Argon2Error),
    ValidationError(ValidationError),
    InvalidToken(JWTError),
    DatabaseError(SqlxError),
    GeneralOversight(String),
}

#[derive(Debug)]
pub enum ForbiddenReason {
    MissingPermission,
    AccountDeactivated,
    IncorrectPassword,
    IncorrectCode,
}

#[derive(Debug)]
pub enum InputInvalidReason {
    NoPasswordSupplied,
    NoFieldsToUpdate,
}

#[derive(Debug)]
pub enum ConflictReason {
    EmailTaken,
}

impl From<UuidError> for ApiError {
    fn from(err: UuidError) -> Self {
        Self::UuidError(err)
    }
}

impl From<ValidationError> for ApiError {
    fn from(err: ValidationError) -> Self {
        Self::ValidationError(err)
    }
}

impl From<Argon2Error> for ApiError {
    fn from(err: Argon2Error) -> Self {
        Self::PasswordError(err)
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

        let (code, msg) = match self {
            Self::InputInvalid(reason) => {
                let message = match reason {
                    InputInvalidReason::NoPasswordSupplied => "No password supplied",
                    InputInvalidReason::NoFieldsToUpdate => "No fields to update provided",
                };
                (StatusCode::BAD_REQUEST, message)
            }
            Self::Conflict(reason) => {
                let message = match reason {
                    ConflictReason::EmailTaken => "This email is already taken",
                };
                (StatusCode::CONFLICT, message)
            }
            Self::GeneralOversight(error) => {
                error!(error);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Self::PasswordError(_) => {
                error!(error_message);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Self::ValidationError(_) => {
                error!(error_message);
                (StatusCode::BAD_REQUEST, "Invalid email")
            }
            Self::Forbidden(reason) => {
                let message = match reason {
                    ForbiddenReason::MissingPermission => "You lack permission to do this action",
                    ForbiddenReason::AccountDeactivated => "Your account has been deactivated",
                    ForbiddenReason::IncorrectPassword => "Incorrect password",
                    ForbiddenReason::IncorrectCode => "Incorrect code",
                };
                (StatusCode::FORBIDDEN, message)
            }
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "You are not logged in"),
            Self::UuidError(_) => {
                error!(error_message);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Self::InvalidToken(_) => {
                error!(error_message);
                (StatusCode::UNAUTHORIZED, "Invalid token")
            }
            Self::DatabaseError(error) => match error {
                SqlxError::RowNotFound => {
                    warn!(error_message);
                    (StatusCode::NOT_FOUND, "Not found")
                }
                _ => {
                    error!(error_message);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
                }
            },
        };

        (code, Json(msg)).into_response()
    }
}
