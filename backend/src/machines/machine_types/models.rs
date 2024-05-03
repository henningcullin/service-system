use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Serialize, Type)]
pub struct MachineType {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize)]
pub struct NewMachineType {
    pub name: String,
}
