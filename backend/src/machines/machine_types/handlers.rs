use std::sync::Arc;

use axum::{extract::State, Extension, Json};
use sqlx::query_as;

use crate::{
    users::models::User,
    utils::{check_permission, errors::ApiError},
    AppState,
};

use super::MachineType;

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
