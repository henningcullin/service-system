use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{utils::errors::ApiError, AppState};

use super::models::{Machine, MachineType, MachineStatus, Facility};



pub async fn index(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Machine>>, ApiError> {
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