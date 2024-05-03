use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Serialize, Type)]
pub struct TaskType {
    pub id: Uuid,
    pub name: String,
}

// Details

#[derive(Deserialize)]
pub struct QueryTaskType {
    pub id: Uuid,
}

// Create

#[derive(Deserialize)]
pub struct NewTaskType {
    pub name: String,
}

// Update

#[derive(Deserialize)]
pub struct UpdateTaskType {
    pub id: Uuid,
    pub name: String,
}
