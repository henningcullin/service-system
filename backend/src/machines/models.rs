use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use uuid::Uuid;

use super::{facilities::Facility, machine_types::MachineType};

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
pub struct MachineStatus {
    pub id: Uuid,
    pub name: String,
}

// Details
#[derive(Deserialize)]
pub struct QueryMachine {
    pub id: Uuid,
}
