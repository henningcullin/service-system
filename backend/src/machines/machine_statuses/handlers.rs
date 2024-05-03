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
    models::{NewMachineStatus, QueryMachineStatus, UpdateMachineStatus},
    MachineStatus,
};

pub async fn details(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryMachineStatus>,
) -> Result<Json<MachineStatus>, ApiError> {
    check_permission(user.role.machine_view)?;

    let machine_status = query_as!(
        MachineStatus,
        r#"
        SELECT
            *
        FROM
            machine_statuses ms
        WHERE
            ms.id = $1
        "#,
        params.id
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(machine_status))
}

pub async fn index(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<MachineStatus>>, ApiError> {
    check_permission(user.role.machine_view)?;

    let machine_statuses = query_as!(
        MachineStatus,
        r#"
        SELECT
            *
        FROM
            machine_statuses
        "#
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(machine_statuses))
}

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NewMachineStatus>,
) -> Result<(StatusCode, Json<MachineStatus>), ApiError> {
    check_permission(user.role.machine_create)?;

    let machine_status = query_as!(
        MachineStatus,
        r#"
        INSERT INTO
            machine_statuses
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

    Ok((StatusCode::CREATED, Json(machine_status)))
}

pub async fn update(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateMachineStatus>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.machine_edit)?;

    let result = query!(
        r#"
        UPDATE 
            machine_statuses ms
        SET
            name = $1
        WHERE
            ms.id = $2
        "#,
        body.name,
        body.id
    )
    .execute(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    match result.rows_affected() {
        1 => Ok(StatusCode::OK),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn delete(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryMachineStatus>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.machine_delete)?;

    let result = query!(r#"DELETE FROM machine_statuses WHERE id = $1"#, params.id)
        .execute(&app_state.db)
        .await
        .map_err(ApiError::from)?;

    match result.rows_affected() {
        1 => Ok(StatusCode::OK),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}
