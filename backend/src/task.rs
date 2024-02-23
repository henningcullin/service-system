use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, Extension, Json};
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

use crate::{user::User, AppState, ErrorResponse};

#[derive(Debug, Deserialize, Serialize, Type)]
#[repr(i32)]
pub enum TaskType {
    Suggestion = 1,
    Issue = 2,
    Service = 3,
    Other = 4
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[repr(i32)]
pub enum TaskStatus {
    Pending = 1,
    Active = 2,
    Completed = 3
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Task {
    id: Uuid,
    title: String,
    description: Option<String>,
    task_type: TaskType,
    status: TaskStatus,
    archived: bool,
    created: DateTime<Utc>,
    edited: DateTime<Utc>,
    creator: Uuid,
    executor: Uuid,
    machine: Uuid
}

pub async fn index(
    State(app_state): State<Arc<AppState>>
) -> Result<Json<Vec<Task>>, (StatusCode, Json<ErrorResponse>)> {
    let tasks: Vec<Task> = sqlx::query_as::<_, Task>("SELECT id, title, description, CAST(task_type AS SIGNED) task_type, CAST(status AS SIGNED) status, archived, created, edited, creator, executor, machine FROM task")
        .fetch_all(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error executing query for task::index: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                status: "fail",
                message: "Could not retrieve the tasks from database".to_owned()
            }))
        })?;

    Ok(Json(tasks))
}