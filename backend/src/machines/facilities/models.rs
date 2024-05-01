use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Serialize, Type)]
pub struct Facility {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub address: Option<String>,
}

// Details

#[derive(Deserialize)]
pub struct QueryFacility {
    pub id: Uuid
}