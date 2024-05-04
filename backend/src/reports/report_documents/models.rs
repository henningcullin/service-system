use serde::Serialize;
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Type, Debug, Serialize)]
pub struct ReportDocument {
    pub task_id: Option<Uuid>,
    pub uri: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}
