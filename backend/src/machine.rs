use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    user::{User, UserRole},
    AppState, ResponseData,
    ResponseType::Fail,
};

// ______________________________________ STRUCTS ______________________________________

#[derive(Serialize, Deserialize, Type)]
#[repr(i32)]
pub enum MachineStatus {
    Active = 1,
    Inactive = 2,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Machine {
    id: Uuid,
    name: String,
    make: Option<String>,
    machine_type: Option<String>,
    status: MachineStatus,
    created: DateTime<Utc>,
    edited: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct QueryMachine {
    id: Uuid,
}

#[derive(Deserialize)]
pub struct NewMachine {
    name: String,
    make: Option<String>,
    machine_type: Option<String>,
    status: Option<MachineStatus>,
}

#[derive(Deserialize)]
pub struct UpdateMachine {
    id: Uuid,
    name: Option<String>,
    make: Option<String>,
    machine_type: Option<String>,
    status: Option<MachineStatus>,
}

// ___________________________________ FUNCTIONS ___________________________________

pub async fn details(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryMachine>,
) -> Result<Json<Machine>, (StatusCode, Json<ResponseData>)> {
    let machine = sqlx::query_as_unchecked!(Machine, "SELECT id, name, make, machine_type, CAST(status AS SIGNED) status, created, edited FROM machine WHERE id = ?", params.id)
        .fetch_one(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error executing query for machine::details: {:?}", e);
            match e {
                sqlx::Error::RowNotFound => {
                    (StatusCode::NOT_FOUND, Json(ResponseData {
                        status: Fail,
                        message: "The specified machine does not exist".to_owned()
                    }))
                },
                _ => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseData {
                        status: Fail,
                        message: "Server error".to_owned()
                    }))
                }
            }
        })?;

    Ok(Json(machine))
}

pub async fn index(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Machine>>, (StatusCode, Json<ResponseData>)> {
    let machines: Vec<Machine> = sqlx::query_as_unchecked!(Machine, "SELECT id, name, make, machine_type, CAST(status AS SIGNED) status, created, edited FROM machine")
        .fetch_all(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error executing query for machine::index: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseData {
                status: Fail,
                message: "Could not retrieve the machines from database".to_owned()
            }))
        })?;

    Ok(Json(machines))
}

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NewMachine>,
) -> Result<(StatusCode, Json<QueryMachine>), (StatusCode, Json<ResponseData>)> {
    if user.role == UserRole::Worker {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ResponseData {
                status: Fail,
                message: "You don't have permission to create machines".to_owned(),
            }),
        ));
    }

    let id = uuid::Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO machine (id, name, make, machine_type, status) VALUES (?, ?, ?, ?, ?)",
        id,
        body.name,
        body.make,
        body.machine_type,
        body.status
    )
    .execute(&app_state.db)
    .await
    .map_err(|e| {
        eprintln!("Error executing query for machine::create: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseData {
                status: Fail,
                message: "Could not create machine in database".to_owned(),
            }),
        )
    })?;

    let id = QueryMachine { id };

    Ok((StatusCode::CREATED, Json(id)))
}

pub async fn delete(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<QueryMachine>,
) -> Result<StatusCode, (StatusCode, Json<ResponseData>)> {
    if user.role == UserRole::Worker {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ResponseData {
                status: Fail,
                message: "You don't have permission to delete machines".to_owned(),
            }),
        ));
    }

    let result = sqlx::query!("DELETE FROM machine WHERE id = ?", query.id)
        .execute(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error executing query for machine::delete: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseData {
                    status: Fail,
                    message: "Could not delete the machine".to_owned(),
                }),
            )
        })?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ResponseData {
                status: Fail,
                message: "The machine was not found in the database".to_owned(),
            }),
        ))
    }
}

pub async fn update(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateMachine>,
) -> Result<StatusCode, (StatusCode, Json<ResponseData>)> {
    if user.role == UserRole::Worker {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ResponseData {
                status: Fail,
                message: "You don't have permission to edit machines".to_owned(),
            }),
        ));
    }

    let result = sqlx::query!(
        "UPDATE machine SET name = COALESCE(?, name), make = COALESCE(?, make), machine_type = COALESCE(?, machine_type), status = COALESCE(?, status) WHERE id = ?",
        body.name,
        body.make,
        body.machine_type,
        body.status,
        body.id
    )
    .execute(&app_state.db)
    .await
    .map_err(|e| {
        eprintln!("Error executing update for machine::update: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ResponseData {
            status: Fail,
            message: "Could not update the machine in the database".to_owned()
        }))
    })?;

    if result.rows_affected() > 0 {
        return Ok(StatusCode::NO_CONTENT);
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ResponseData {
                status: Fail,
                message: "The machine was not found in the database".to_owned(),
            }),
        ))
    }
}
