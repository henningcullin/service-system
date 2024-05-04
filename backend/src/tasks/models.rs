use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    machines::models::ShortMachine, users::models::ShortUser, utils::db::nullable::Nullable,
};

use super::{task_documents::TaskDocument, task_statuses::TaskStatus, task_types::TaskType};

#[derive(Debug, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub archived: bool,
    pub creator: ShortUser,
    pub executors: Option<Vec<ShortUser>>,
    pub documents: Option<Vec<TaskDocument>>,
    pub machine: Option<ShortMachine>,
    pub created: DateTime<Utc>,
    pub edited: DateTime<Utc>,
    pub due_at: Option<DateTime<Utc>>,
}

// Details

#[derive(Deserialize)]
pub struct QueryTask {
    pub task_id: Option<Uuid>,
    pub creator_id: Option<Uuid>,
    pub executor_id: Option<Uuid>,
}

// Create

#[derive(Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: String,
    pub task_type: Uuid,
    pub status: Uuid,
    pub archived: Option<bool>,
    pub executors: Option<Vec<Uuid>>,
    pub machine: Option<Uuid>,
    pub due_at: Option<DateTime<Utc>>,
}

// Update

#[derive(Deserialize, Debug)]
pub struct UpdateTask {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub task_type: Option<Uuid>,
    pub status: Option<Uuid>,
    pub archived: Option<Uuid>,
    #[serde(default)]
    pub machine: Nullable<Uuid>,
    #[serde(default)]
    pub due_at: Nullable<DateTime<Utc>>,
}
