use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use sqlx::{query, query_as, Postgres, QueryBuilder};

use crate::{
    field_vec, update_field,
    users::models::User,
    utils::{
        check_permission,
        db::{Field, IntoField},
        errors::{ApiError, InputInvalidReason},
    },
    AppState,
};

use super::{
    facilities::Facility,
    machine_statuses::MachineStatus,
    machine_types::MachineType,
    models::{DeleteMachine, Machine, NewMachine, QueryMachine, UpdateMachine},
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
    .await?;

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
    .await?;

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
    .await?;

    Ok((StatusCode::CREATED, Json(machine)))
}

pub async fn update(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateMachine>,
) -> Result<Json<Machine>, ApiError> {
    check_permission(user.role.machine_edit)?;

    let mut tx = app_state.db.begin().await?;

    let mut query_builder = QueryBuilder::<Postgres>::new("UPDATE machines SET");
    let mut separated_list = query_builder.separated(",");

    let fields = field_vec![
        name => body.name,
        make => body.make,
        machine_type => body.machine_type,
        status => body.status,
        facility => body.facility
    ];

    if fields.len() < 1 {
        return Err(ApiError::InputInvalid(InputInvalidReason::NoFieldsToUpdate));
    }

    for (field, value) in fields {
        update_field!(separated_list, field, value);
    }

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(body.id);

    let result = query_builder.build().execute(&mut *tx).await?;

    if result.rows_affected() != 1 {
        return Err(ApiError::GeneralOversight(
            "Provided machine to update didn't exist".to_owned(),
        ));
    }

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
        body.id
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(machine))
}

pub async fn delete(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<DeleteMachine>,
) -> Result<StatusCode, ApiError> {
    check_permission(user.role.machine_delete)?;

    let result = query!(r#"DELETE FROM machines WHERE id = $1"#, params.id)
        .execute(&app_state.db)
        .await?;

    match result.rows_affected() {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}
