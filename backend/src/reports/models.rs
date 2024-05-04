use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::users::models::ShortUser;

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
    pub documents: Option<Vec<ReportDocument>>,
    pub created: DateTime<Utc>,
    pub edited: DateTime<Utc>,
}

// Create

#[derive(Deserialize)]
pub struct NewReport {
    pub title: String,
    pub description: String,
    pub report_type: Uuid,
    pub status: Uuid,
    pub archived: Option<bool>,
}
