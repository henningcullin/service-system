use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Serialize, Type)]
pub struct ReportType {
    pub id: Uuid,
    pub name: String,
}

// Details

#[derive(Deserialize)]
pub struct QueryReportType {
    pub id: Uuid,
}

// Create
#[derive(Deserialize)]
pub struct NewReportType {
    pub name: String,
}

// Update

#[derive(Deserialize)]
pub struct UpdateReportType {
    pub id: Uuid,
    pub name: String,
}
