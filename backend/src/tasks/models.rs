use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{machines::models::ShortMachine, users::models::ShortUser};

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
