use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Serialize, Type)]
pub struct MachineType {
    pub id: Uuid,
    pub name: String,
}

// Details

#[derive(Deserialize)]
pub struct QueryMachineType {
    pub id: Uuid,
}

// Create

#[derive(Deserialize)]
pub struct NewMachineType {
    pub name: String,
}

// Update

#[derive(Deserialize)]
pub struct UpdateMachineType {
    pub id: Uuid,
    pub name: String,
}
