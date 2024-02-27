use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

use crate::{
    user::{User, UserRole},
    AppState, ErrorResponse,
};

#[derive(Debug, Deserialize, Serialize, Type, PartialEq)]
#[repr(i32)]
pub enum TaskType {
    Suggestion = 1,
    Issue = 2,
    Service = 3,
    Other = 4,
}

#[derive(Debug, Deserialize, Serialize, Type, PartialEq)]
#[repr(i32)]
pub enum TaskStatus {
    Pending = 1,
    Active = 2,
    Completed = 3,
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
    executor: Option<Uuid>,
    machine: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryTask {
    id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct NewTask {
    title: String,
    description: Option<String>,
    task_type: Option<TaskType>,
    status: Option<TaskStatus>,
    executor: Option<Uuid>,
    machine: Option<Uuid>,
}

pub async fn details(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryTask>,
) -> Result<Json<Task>, (StatusCode, Json<ErrorResponse>)> {
    let task = sqlx::query_as::<_, Task>("SELECT id, title, description, CAST(task_type AS SIGNED) task_type, CAST(status AS SIGNED) status, archived, created, edited, creator, executor, machine FROM task WHERE id = ?")
        .bind(params.id)
        .fetch_one(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error executing query for task::details: {:?}", e);
            match e {
                sqlx::Error::RowNotFound => {
                    (StatusCode::NOT_FOUND, Json(ErrorResponse {
                        status: "fail",
                        message: "The specified task does not exist".to_owned()
                    }))
                },
                _ => {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                        status: "fail",
                        message: "Server error".to_owned()
                    }))
                }
            }
        })?;

    Ok(Json(task))
}

pub async fn index(
    State(app_state): State<Arc<AppState>>,
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

pub async fn create(
    Extension(user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NewTask>,
) -> Result<(StatusCode, Json<QueryTask>), (StatusCode, Json<ErrorResponse>)> {
    if user.role == UserRole::Worker
        && (body.task_type != Some(TaskType::Suggestion) || body.task_type.is_some())
    {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                status: "fail",
                message: "You can only set tasktype to suggestion".to_owned(),
            }),
        ));
    }

    let id = uuid::Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO task (id, title, description, task_type, status, creator, executor, machine) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        id,
        body.title,
        body.description,
        body.task_type,
        body.status,
        user.id,
        body.executor,
        body.machine
    )
        .execute(&app_state.db)
        .await
        .map_err(|e| {
            eprintln!("Error executing query for machine::create: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                status: "fail",
                message: "Could not create machine in database".to_owned()
            }))
        })?;

    Ok((StatusCode::CREATED, Json(QueryTask { id })))
}
