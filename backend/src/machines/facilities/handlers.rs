use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use sqlx::{query, query_as, Postgres, QueryBuilder};

use crate::{
    field_vec,
    machines::facilities::Facility,
    update_field,
    users::models::User,
    utils::{
        check_permission,
        db::{Field, IntoField},
        errors::{ApiError, InputInvalidReason},
    },
    AppState,
};

use super::models::{NewFacility, QueryFacility, UpdateFacility};

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
) -> Result<(StatusCode, Json<Facility>), ApiError> {
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

    Ok((StatusCode::CREATED, Json(facility)))
}

pub async fn update(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateFacility>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.facility_edit)?;

    let mut query_builder = QueryBuilder::<Postgres>::new("UPDATE facilities SET");
    let mut separated_list = query_builder.separated(",");

    let fields = field_vec![
        name => body.name,
        address => body.address
    ];

    if fields.len() < 1 {
        return Err(ApiError::InputInvalid(InputInvalidReason::NoFieldsToUpdate));
    }

    for (field, value) in fields {
        update_field!(separated_list, field, value);
    }

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(body.id);

    let result = query_builder
        .build()
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
    Query(params): Query<QueryFacility>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.facility_delete)?;

    let result = query!(r#"DELETE FROM roles WHERE id = $1"#, params.id)
        .execute(&app_state.db)
        .await
        .map_err(ApiError::from)?;

    match result.rows_affected() {
        1 => Ok(StatusCode::OK),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}
