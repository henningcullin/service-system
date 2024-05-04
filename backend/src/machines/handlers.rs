use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use sqlx::query_as;

use crate::{
    users::models::User,
    utils::{check_permission, errors::ApiError},
    AppState,
};

use super::{
    facilities::Facility,
    machine_statuses::MachineStatus,
    machine_types::MachineType,
    models::{Machine, NewMachine, QueryMachine},
};

pub async fn details(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryMachine>,
) -> Result<Json<Machine>, ApiError> {
    check_permission(user.role.machine_view)?;

    let machine = query_as!(
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
    check_permission(user.role.machine_view)?;

    let machines = query_as!(
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

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NewMachine>,
) -> Result<(StatusCode, Json<Machine>), ApiError> {
    check_permission(user.role.machine_create)?;

    let machine = query_as!(
        Machine,
        r#"
        WITH
            new_machine
        AS 
        (
            INSERT INTO 
                machines
            (
                name,
                make,
                machine_type,
                status,
                facility
            )
            VALUES
            (
                $1,
                $2,
                $3,
                $4,
                $5
            )
            RETURNING *
        )
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
            new_machine m
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
        body.name,
        body.make,
        body.machine_type,
        body.status,
        body.facility
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(ApiError::from)?;

    Ok((StatusCode::CREATED, Json(machine)))
}
