use axum::{
    extract::{
        Query, 
        State
    },
    Json,
    http::StatusCode
};
use sqlx::{
    MySqlPool, 
    FromRow, 
    Type
};
use serde_derive::{
    Serialize,
    Deserialize
};
use chrono::{
    DateTime, 
    Utc
};
use uuid::Uuid;

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
    status: MachineStatus
}

#[derive(Deserialize)]
pub struct UpdateMachine {
    id: u64,
    name: Option<String>,
    machine_type: Option<String>,
    status: Option<MachineStatus>
}

// ___________________________________ FUNCTIONS ___________________________________

pub async fn details(
        State(pool): State<MySqlPool>,
        Query(query): Query<QueryMachine>,
    ) -> Result<Json<Machine>, StatusCode> {
        let machine = sqlx::query_as::<_, Machine>("SELECT id, name, make, machine_type, CAST(status AS SIGNED) status, created, edited FROM machine WHERE id = ?")
            .bind(query.id)
            .fetch_one(&pool)
            .await
            .map_err(|e| {
                eprintln!("Error executing query for machine::details: {:?}", e);

                if let sqlx::Error::RowNotFound = e {
                    return StatusCode::NOT_FOUND;
                }

                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        Ok(Json(machine)) 
}

pub async fn index(
        State(pool): State<MySqlPool>
    ) -> Result<Json<Vec<Machine>>, StatusCode> {
        let machines: Vec<Machine> = sqlx::query_as::<_, Machine>("SELECT id, name, make, machine_type, CAST(status AS SIGNED) status, created, edited FROM machine")
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                eprintln!("Error executing query for machine::index: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        Ok(Json(machines))
}

pub async fn create(
        State(pool): State<MySqlPool>,
        Json(input): Json<NewMachine>,
    ) -> Result<(StatusCode, Json<QueryMachine>) , StatusCode> {

        let id = uuid::Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO machine (id, name, make, machine_type, status) VALUES (?, ?, ?, ?, ?)",
            id,
            input.name,
            input.make,
            input.machine_type,
            input.status
        )
            .execute(&pool)
            .await
            .map_err(|e| {
                eprintln!("Error executing query for machine::create: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
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
        State(pool): State<MySqlPool>,
        Query(query): Query<QueryMachine>
    ) -> Result<StatusCode, StatusCode> {

        let result = sqlx::query!(
            "DELETE FROM machine WHERE id = ?",
            query.id
        )
        .execute(&pool)
        .await
        .map_err(|e| {
            eprintln!("Error executing query for machine::delete: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        if result.rows_affected() > 0 {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Ok(StatusCode::NOT_FOUND)
        }
}

pub async fn update(
        State(pool): State<MySqlPool>,
        Json(input): Json<UpdateMachine>
    ) -> Result<StatusCode, StatusCode> {

        let mut query = sqlx::QueryBuilder::new("UPDATE machine SET");
        let mut first = true;

        if let Some(name) = input.name {
            if !first {
                query.push(",");
            }
            query.push(" name = ").push_bind(name);
            first = false;
        }

        if let Some(machine_type) = input.machine_type {
            if !first {
                query.push(",");
            }
            query.push(" machine_type = ").push_bind(machine_type);
            first = false;
        }

        if let Some(status) = input.status {
            if !first {
                query.push(",");
            }
            query.push(" status = ").push_bind(status);
        }

        query.push(" WHERE id = ").push_bind(input.id);

        let result = query.build().execute(&pool)
            .await
            .map_err(|e| {
                eprintln!("Error executing query for machine::update: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        if result.rows_affected() > 0 {
            return Ok(StatusCode::NO_CONTENT);
        } else {
            return Ok(StatusCode::NOT_FOUND);
        }
}