use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{machines::models::Machine, users::models::User};

use super::{task_documents::TaskDocument, task_statuses::TaskStatus, task_types::TaskType};

pub struct Task {
    id: Uuid,
    title: String,
    description: String,
    task_type: TaskType,
    status: TaskStatus,
    archived: bool,
    creator: User,
    executors: Option<Vec<User>>,
    documents: Option<Vec<TaskDocument>>,
    machine: Option<Machine>,
    created: DateTime<Utc>,
    edited: DateTime<Utc>,
    due_at: Option<DateTime<Utc>>,
}
