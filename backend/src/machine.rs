use std::sync::Arc;
use axum::{
    extract::{
        Query, 
        State
    },
    Json,
    http::StatusCode
};
use serde_json::error;
use sqlx::{
    FromRow, 
    Type
};
use serde::{
    Serialize,
    Deserialize
};
use chrono::{
    DateTime, 
    Utc
};
use uuid::Uuid;

use crate::{AppState, ErrorResponse};

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
    edited: DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
pub struct QueryMachine {
    id: Uuid
}

#[derive(Deserialize)]
pub struct NewMachine {
    name: String,
    make: Option<String>,
    machine_type: Option<String>,
    status: Option<MachineStatus>
}

#[derive(Deserialize)]
pub struct UpdateMachine {
    id: Uuid,
    name: Option<String>,
    make: Option<String>,
    machine_type: Option<String>,
    status: Option<MachineStatus>
}

// ___________________________________ FUNCTIONS ___________________________________

pub async fn details(
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<QueryMachine>,
) -> Result<Json<Machine>, (StatusCode, Json<ErrorResponse>)> {
    let machine = sqlx::query_as::<_, Machine>("SELECT id, name, make, machine_type, CAST(status AS SIGNED) status, created, edited FROM machine WHERE id = ?")
        .bind(query.id)
        .fetch_one(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error executing query for machine::details: {:?}", e);

            match e {
                sqlx::Error::RowNotFound => {
                    let error_response = ErrorResponse {
                        status: "fail",
                        message: "The specified machine does not exist".to_owned()
                    };
                    (StatusCode::NOT_FOUND, Json(error_response))
                },
                _ => {
                    let error_response = ErrorResponse {
                        status: "fail",
                        message: "Server error".to_owned()
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
                }
            }

        })?;

    Ok(Json(machine)) 
}

pub async fn index(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Machine>>, (StatusCode, Json<ErrorResponse>)> {
    let machines: Vec<Machine> = sqlx::query_as::<_, Machine>("SELECT id, name, make, machine_type, CAST(status AS SIGNED) status, created, edited FROM machine")
        .fetch_all(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error executing query for machine::index: {:?}", e);
            let error_response = ErrorResponse {
                status: "fail",
                message: "Could not retrieve the machines from database".to_owned()
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    Ok(Json(machines))
}

pub async fn create(
    State(app_state): State<Arc<AppState>>,
    Json(input): Json<NewMachine>,
) -> Result<(StatusCode, Json<QueryMachine>) , (StatusCode, Json<ErrorResponse>)> {

    let id = uuid::Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO machine (id, name, make, machine_type, status) VALUES (?, ?, ?, ?, ?)",
        id,
        input.name,
        input.make,
        input.machine_type,
        input.status
    )
        .execute(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error executing query for machine::create: {:?}", e);
            let error_response = ErrorResponse {
                status: "fail",
                message: "Could not create machine in database".to_owned()
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let id = QueryMachine { id };

    Ok(
        (
            StatusCode::CREATED,
            Json(id)
        )
    )
}

pub async fn delete(
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<QueryMachine>
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {

    let result = sqlx::query!(
        "DELETE FROM machine WHERE id = ?",
        query.id
    )
    .execute(&app_state.db)
    .await
    .map_err(|e| {
        eprintln!("Error executing query for machine::delete: {:?}", e);
        let error_response = ErrorResponse {
            status: "fail",
            message: "Could not delete the machine".to_owned()
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        let error_response = ErrorResponse {
            status: "fail",
            message: "The machine was not found in the database".to_owned()
        };
        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }
}

pub async fn update(
    State(app_state): State<Arc<AppState>>,
    Json(input): Json<UpdateMachine>
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {

    let result = sqlx::query!(
        "UPDATE machine SET name = COALESCE(?, name), make = COALESCE(?, make), machine_type = COALESCE(?, machine_type), status = COALESCE(?, status) WHERE id = ?",
        input.name,
        input.make,
        input.machine_type,
        input.status,
        input.id
    )
    .execute(&app_state.db)
    .await
    .map_err(|e| {
        eprintln!("Error executing update for machine::update: {:?}", e);
        let error_response = ErrorResponse {
            status: "fail",
            message: "Could not update the machine in the database".to_owned()
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if result.rows_affected() > 0 {
        return Ok(StatusCode::NO_CONTENT);
    } else {
        let error_response = ErrorResponse {
            status: "fail",
            message: "The machine was not found in the database".to_owned()
        };
        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }
}