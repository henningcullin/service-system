use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

use crate::utils::db::Nullable;

use super::{facilities::Facility, machine_statuses::MachineStatus, machine_types::MachineType};

#[derive(Serialize)]
pub struct Machine {
    pub id: Uuid,
    pub name: String,
    pub make: Option<String>,
    pub machine_type: MachineType,
    pub status: MachineStatus,
    /*     pub tasks: Option<Vec<ShortTask>>,
    pub reports: Option<Vec<ShortReport>>, */
    pub created: DateTime<Utc>,
    pub edited: DateTime<Utc>,
    pub facility: Option<Facility>,
    pub image: Option<String>,
}

// Short variant
#[derive(Type, FromRow, Serialize, Debug)]
pub struct ShortMachine {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub make: Option<String>,
    pub image: Option<String>,
}

// Details
#[derive(Deserialize)]
pub struct QueryMachine {
    pub id: Uuid,
}

// Create
#[derive(Deserialize)]
pub struct NewMachine {
    pub name: String,
    pub make: Option<String>,
    pub machine_type: Uuid,
    pub status: Uuid,
    pub facility: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct UpdateMachine {
    pub id: Uuid,
    pub name: Option<String>,
    #[serde(default)]
    pub make: Nullable<String>,
    pub machine_type: Option<Uuid>,
    pub status: Option<Uuid>,
    #[serde(default)]
    pub facility: Nullable<Uuid>,
}

// Delete
#[derive(Deserialize)]
pub struct DeleteMachine {
    pub id: Uuid,
}
