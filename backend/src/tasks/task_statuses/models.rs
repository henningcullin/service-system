use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Serialize, Type)]
pub struct TaskStatus {
    pub id: Uuid,
    pub name: String,
}

// Details

#[derive(Deserialize)]
pub struct QueryTaskStatus {
    pub id: Uuid,
}

// Create

#[derive(Deserialize)]
pub struct NewTaskStatus {
    pub name: String,
}

// Update

#[derive(Deserialize)]
pub struct UpdateTaskStatus {
    pub id: Uuid,
    pub name: String,
}
