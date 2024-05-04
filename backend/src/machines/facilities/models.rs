use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use uuid::Uuid;

use crate::utils::db::Nullable;

#[derive(Serialize, Type, Clone)]
pub struct Facility {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub address: Option<String>,
}

// Details

#[derive(Deserialize)]
pub struct QueryFacility {
    pub id: Uuid,
}

// Create

#[derive(Deserialize)]
pub struct NewFacility {
    pub name: String,
    pub address: Option<String>,
}

// Update

#[derive(Deserialize)]
pub struct UpdateFacility {
    pub id: Uuid,
    pub name: Option<String>,
    #[serde(default)]
    pub address: Nullable<String>,
}
