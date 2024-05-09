use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{machines::models::ShortMachine, users::models::ShortUser, utils::db::Nullable};

use super::{
    report_documents::ReportDocument, report_statuses::ReportStatus, report_types::ReportType,
};

#[derive(Serialize)]
pub struct Report {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub report_type: ReportType,
    pub status: ReportStatus,
    pub archived: bool,
    pub creator: ShortUser,
    pub machine: Option<ShortMachine>,
    pub documents: Option<Vec<ReportDocument>>,
    pub created: DateTime<Utc>,
    pub edited: DateTime<Utc>,
}

// Details

#[derive(Deserialize)]
pub struct QueryReport {
    pub report_id: Option<Uuid>,
    pub creator_id: Option<Uuid>,
}

// Create

#[derive(Deserialize)]
pub struct NewReport {
    pub title: String,
    pub description: String,
    pub report_type: Uuid,
    pub status: Uuid,
    pub archived: Option<bool>,
    pub machine: Option<Uuid>,
}

// Update

#[derive(Deserialize)]
pub struct UpdateReport {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub report_type: Option<Uuid>,
    pub status: Option<Uuid>,
    #[serde(default)]
    pub archived: Nullable<bool>,
    #[serde(default)]
    pub machine: Nullable<Uuid>,
}

// Delete

#[derive(Deserialize)]
pub struct DeleteReport {
    pub id: Uuid,
}
