use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Machine {
    pub id: Uuid,
    pub name: String,
    pub make: Option<String>,
    pub machine_type: MachineType,
    pub status: MachineStatus,
    pub created: DateTime<Utc>,
    pub edited: DateTime<Utc>,
    pub facility: Option<Facility>,
    pub image: Option<String>,
}

#[derive(Serialize, Type)]
pub struct MachineType {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Type)]
pub struct MachineStatus {
    pub id: Uuid,
    pub name: Option<String>,
}

#[derive(Serialize, Type)]
pub struct Facility {
    pub id: Uuid,
    pub name: Option<String>,
    pub address: Option<String>,
}