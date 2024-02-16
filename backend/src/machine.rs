use axum::{
    extract::{
        Query, State
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

#[derive(Serialize, Deserialize, Type)]
#[repr(i32)]
pub enum MachineStatus {
    Active = 1,
    Inactive = 2,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Machine {
    id: u64,
    name: String,
    machine_type: Option<String>,
    status: MachineStatus
}

#[derive(Serialize, Deserialize)]
pub struct QueryMachine {
    id: u64
}

#[derive(Deserialize)]
pub struct NewMachine {
    name: String,
    machine_type: Option<String>,
    status: MachineStatus
}

pub async fn details(
        State(pool): State<MySqlPool>,
        Query(query): Query<QueryMachine>,
    ) -> Result<Json<Machine>, StatusCode> {
        let facility = sqlx::query_as::<_, Machine>("SELECT id, name, machine_type, CAST(status AS SIGNED) status FROM machine WHERE id = ?")
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

        Ok(Json(facility)) 
}

pub async fn index(
        State(pool): State<MySqlPool>
    ) -> Result<Json<Vec<Machine>>, StatusCode> {
        let machines = sqlx::query_as::<_, Machine>("SELECT id, name, machine_type, CAST(status AS SIGNED) status FROM machine")
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
) -> Result<Json<QueryMachine>, StatusCode> {

    // Execute the INSERT statement with a prepared statement
    let last_inserted_id:u64 = sqlx::query!(
        "INSERT INTO machine (name, machine_type, status) VALUES (?, ?, ?)",
        input.name,
        input.machine_type,
        input.status
    )
        .execute(&pool)
        .await
        .map_err(|e| {
            eprintln!("Error executing query for machine::create: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .last_insert_id();

            // Create and return the response struct
    let id = QueryMachine { id: last_inserted_id };

    Ok(Json(id))
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
        eprintln!("Error executing query for machine::create: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}