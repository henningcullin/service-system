use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Extension, Json};
use sqlx::query;

use crate::{
    users::models::User,
    utils::{check_permission, errors::ApiError},
    AppState,
};

use super::models::TaskExecutor;

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<TaskExecutor>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.task_edit)?;

    let result = query!(
        r#"
        INSERT INTO
            task_executors
        (
            task_id,
            user_id
        )
        VALUES
        (
            $1,
            $2
        )
        "#,
        body.task_id,
        body.user_id
    )
    .execute(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    match result.rows_affected() {
        1 => Ok(StatusCode::CREATED),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn delete(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<TaskExecutor>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.task_edit)?;

    let result = query!(
        r#"
        DELETE FROM
            task_executors
        WHERE
            task_id = $1
        AND
            user_id = $2
        "#,
        body.task_id,
        body.user_id
    )
    .execute(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    match result.rows_affected() {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}
