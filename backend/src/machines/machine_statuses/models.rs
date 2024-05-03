use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Serialize, Type)]
pub struct MachineStatus {
    pub id: Uuid,
    pub name: String,
}

// Details

#[derive(Deserialize)]
pub struct QueryMachineStatus {
    pub id: Uuid,
}

// Create

#[derive(Deserialize)]
pub struct NewMachineStatus {
    pub name: String,
}

// Update

#[derive(Deserialize)]
pub struct UpdateMachineStatus {
    pub id: Uuid,
    pub name: String,
}
