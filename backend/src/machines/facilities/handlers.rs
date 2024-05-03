use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Extension, Json,
};
use sqlx::query_as;

use crate::{
    machines::facilities::Facility,
    users::models::User,
    utils::{check_permission, errors::ApiError},
    AppState,
};

use super::models::{NewFacility, QueryFacility};

pub async fn details(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryFacility>,
) -> Result<Json<Facility>, ApiError> {
    check_permission(user.role.facility_view)?;

    let facility = query_as!(
        Facility,
        r#"
        SELECT
            *
        FROM
            facilities f
        WHERE
            f.id = $1
        "#,
        params.id
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(facility))
}

pub async fn index(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Facility>>, ApiError> {
    check_permission(user.role.facility_view)?;

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

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NewFacility>,
) -> Result<Json<Facility>, ApiError> {
    check_permission(user.role.facility_create)?;

    let facility = query_as!(
        Facility,
        r#"
        INSERT INTO
            facilities
        (
            name,
            address
        )
        VALUES
        (
            $1,
            $2
        )
        RETURNING
            *
        "#,
        body.name,
        body.address
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(facility))
}
