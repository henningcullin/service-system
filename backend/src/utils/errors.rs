use axum::{body::Body, http::{Response, StatusCode}, response::IntoResponse, Json};
use sqlx::Error as SqlxError;


#[derive(Debug)]
pub enum ApiError {
    DatabaseError(SqlxError),
}

impl From<SqlxError> for ApiError {
    fn from(err: SqlxError) -> Self {
        Self::DatabaseError(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response<Body> {
        match self {
            Self::DatabaseError(err) => {
                let message = "Database error";
                let response_body = Json(message);
                (StatusCode::INTERNAL_SERVER_ERROR, response_body).into_response()
            }
        }
    }
}