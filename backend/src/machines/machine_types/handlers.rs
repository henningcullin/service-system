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
    models::{NewMachineType, QueryMachineType, UpdateMachineType},
    MachineType,
};

pub async fn details(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryMachineType>,
) -> Result<Json<MachineType>, ApiError> {
    check_permission(user.role.machine_view)?;

    let machine_type = query_as!(
        MachineType,
        r#"
        SELECT
            *
        FROM
            machine_types mt
        WHERE
            mt.id = $1
        "#,
        params.id
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(machine_type))
}

pub async fn index(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<MachineType>>, ApiError> {
    check_permission(user.role.machine_view)?;

    let machine_types = query_as!(
        MachineType,
        r#"
        SELECT
            *
        FROM
            machine_types
        "#
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(machine_types))
}

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NewMachineType>,
) -> Result<(StatusCode, Json<MachineType>), ApiError> {
    check_permission(user.role.machine_create)?;

    let machine_type = query_as!(
        MachineType,
        r#"
        INSERT INTO
            machine_types
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

    Ok((StatusCode::CREATED, Json(machine_type)))
}

pub async fn update(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateMachineType>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.machine_edit)?;

    let result = query!(
        r#"
        UPDATE 
            machine_types mt
        SET
            name = $1
        WHERE
            mt.id = $2
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
