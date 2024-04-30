use std::sync::Arc;

use axum::{extract::{Query, State}, Json};

use crate::{utils::errors::ApiError, AppState};

use super::models::{QueryRole, Role};

pub async fn details(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryRole>,
) -> Result<Json<Role>, ApiError> {
    let role = sqlx::query_as!(
        Role,
        r#"
        SELECT
            *
        FROM
            roles r
        WHERE
            r.id = $1 
        "#,
        params.id
    )
        .fetch_one(&app_state.db)
        .await
        .map_err(ApiError::from)?;

    Ok(Json(role))
}

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