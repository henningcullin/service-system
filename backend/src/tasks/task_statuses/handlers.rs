use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use sqlx::{query, query_as};

use crate::{
    users::models::User,
    utils::{check_permission, errors::ApiError},
    AppState,
};

use super::{
    models::{NewTaskStatus, QueryTaskStatus, UpdateTaskStatus},
    TaskStatus,
};

pub async fn details(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryTaskStatus>,
) -> Result<Json<TaskStatus>, ApiError> {
    check_permission(user.role.task_view)?;

    let task_status = query_as!(
        TaskStatus,
        r#"
        SELECT
            *
        FROM
            task_statuses ts
        WHERE
            ts.id = $1
        "#,
        params.id
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(task_status))
}

pub async fn index(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<TaskStatus>>, ApiError> {
    check_permission(user.role.task_view)?;

    let task_statuses = query_as!(
        TaskStatus,
        r#"
        SELECT
            *
        FROM
            task_statuses
        "#
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(task_statuses))
}

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NewTaskStatus>,
) -> Result<(StatusCode, Json<TaskStatus>), ApiError> {
    check_permission(user.role.task_create)?;

    let task_status = query_as!(
        TaskStatus,
        r#"
        INSERT INTO
            task_statuses
        (
            name
        )
        VALUES
        (
            $1
        )
        RETURNING
            *
        "#,
        body.name
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok((StatusCode::CREATED, Json(task_status)))
}

pub async fn update(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateTaskStatus>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.task_edit)?;

    let result = query!(
        r#"
        UPDATE 
            task_statuses ts
        SET
            name = $1
        WHERE
            ts.id = $2
        "#,
        body.name,
        body.id
    )
    .execute(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    match result.rows_affected() {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn delete(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryTaskStatus>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.task_delete)?;

    let result = query!(r#"DELETE FROM task_statuses WHERE id = $1"#, params.id)
        .execute(&app_state.db)
        .await
        .map_err(ApiError::from)?;

    match result.rows_affected() {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}
