use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Extension, Json,
};

use crate::{
    users::models::User,
    utils::errors::{ApiError, ForbiddenReason},
    AppState,
};

use super::{
    facilities::Facility,
    models::{Machine, MachineStatus, MachineType, QueryMachine},
};

pub async fn details(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryMachine>,
) -> Result<Json<Machine>, ApiError> {
    if !user.role.machine_view {
        return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission));
    }

    let machine = sqlx::query_as!(
        Machine,
        r#"
        SELECT
            m.id,
            m.name,
            m.make,
            (
                mt.id,
                mt.name
            ) AS "machine_type!: MachineType",
            (
                ms.id,
                ms.name
            ) AS "status!: MachineStatus",
            m.created,
            m.edited,
            (
                f.id,
                f.name,
                f.address
            ) AS "facility?: Facility",
            m.image
        FROM
            machines m
        INNER JOIN 
            machine_types mt
        ON
            m.machine_type = mt.id
        INNER JOIN
            machine_statuses ms
        ON
            m.status = ms.id
        LEFT JOIN
            facilities f
        ON
            m.facility = f.id
        WHERE
                m.id = $1
        "#,
        params.id
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(machine))
}

pub async fn index(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Machine>>, ApiError> {
    if !user.role.machine_view {
        return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission));
    }

    let machines = sqlx::query_as!(
        Machine,
        r#"
        SELECT
            m.id,
            m.name,
            m.make,
            (
                mt.id,
                mt.name
            ) AS "machine_type!: MachineType",
            (
                ms.id,
                ms.name
            ) AS "status!: MachineStatus",
            m.created,
            m.edited,
            (
                f.id,
                f.name,
                f.address
            ) AS "facility?: Facility",
            m.image
        FROM
            machines m
        INNER JOIN 
            machine_types mt
        ON
            m.machine_type = mt.id
        INNER JOIN
            machine_statuses ms
        ON
            m.status = ms.id
        LEFT JOIN
            facilities f
        ON
            m.facility = f.id
        "#,
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(machines))
}
