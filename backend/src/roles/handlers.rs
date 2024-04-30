use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{utils::errors::ApiError, AppState};

use super::models::Role;

pub async fn index(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Role>>, ApiError> {

    let roles = sqlx::query_as!(
        Role,
        r#"
        SELECT 
            * 
        FROM 
            roles
        "#
    )
        .fetch_all(&app_state.db)
        .await
        .map_err(ApiError::from)?;


    Ok(Json(roles))
}