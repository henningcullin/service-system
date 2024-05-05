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
    models::{NewTaskType, QueryTaskType, UpdateTaskType},
    TaskType,
};

pub async fn details(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryTaskType>,
) -> Result<Json<TaskType>, ApiError> {
    check_permission(user.role.task_view)?;

    let task_type = query_as!(
        TaskType,
        r#"
        SELECT
            *
        FROM
            task_types tt
        WHERE
            tt.id = $1
        "#,
        params.id
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(task_type))
}

pub async fn index(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<TaskType>>, ApiError> {
    check_permission(user.role.task_view)?;

    let task_types = query_as!(
        TaskType,
        r#"
        SELECT
            *
        FROM
            task_types
        "#
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(task_types))
}

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NewTaskType>,
) -> Result<(StatusCode, Json<TaskType>), ApiError> {
    check_permission(user.role.task_create)?;

    let task_type = query_as!(
        TaskType,
        r#"
        INSERT INTO
            task_types
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

    Ok((StatusCode::CREATED, Json(task_type)))
}

pub async fn update(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateTaskType>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.task_edit)?;

    let result = query!(
        r#"
        UPDATE 
            task_types tt
        SET
            name = $1
        WHERE
            tt.id = $2
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
    Query(params): Query<QueryTaskType>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.task_delete)?;

    let result = query!(r#"DELETE FROM task_types WHERE id = $1"#, params.id)
        .execute(&app_state.db)
        .await
        .map_err(ApiError::from)?;

    match result.rows_affected() {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}
