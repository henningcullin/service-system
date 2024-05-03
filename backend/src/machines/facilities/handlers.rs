use std::sync::Arc;

use axum::{extract::State, Extension, Json};
use sqlx::query_as;

use crate::{
    machines::facilities::Facility,
    users::models::User,
    utils::errors::{ApiError, ForbiddenReason},
    AppState,
};

pub async fn index(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Facility>>, ApiError> {
    if !user.role.facility_view {
        return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission));
    }

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
