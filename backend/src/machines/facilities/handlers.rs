use std::sync::Arc;

use axum::{extract::{Query, State}, Json};
use sqlx::query_as;

use crate::{machines::facilities::Facility, utils::errors::ApiError, AppState};

use super::models::QueryFacility;





pub async fn index(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Facility>>, ApiError> {

    let facilities = query_as!(
        Facility,
        r#"
        SELECT
            *
        FROM
            facilities
        "#
    )
        .fetch_all(&app_state.db)
        .await
        .map_err(ApiError::from)?;

    Ok(Json(facilities))

}