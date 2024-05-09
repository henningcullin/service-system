use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Serialize, Type, Debug)]
pub struct ReportStatus {
    pub id: Uuid,
    pub name: String,
}

// Details

#[derive(Deserialize)]
pub struct QueryReportStatus {
    pub id: Uuid,
}

// Create

#[derive(Deserialize)]
pub struct NewReportStatus {
    pub name: String,
}

// Update

#[derive(Deserialize)]
pub struct UpdateReportStatus {
    pub id: Uuid,
    pub name: String,
}
