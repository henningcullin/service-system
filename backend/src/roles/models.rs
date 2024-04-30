use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub level: i32,
    pub has_password: bool,
    pub user_view: bool,
    pub user_create: bool,
    pub user_edit: bool,
    pub user_delete: bool,
    pub machine_view: bool,
    pub machine_create: bool,
    pub machine_edit: bool,
    pub machine_delete: bool,
    pub task_view: bool,
    pub task_create: bool,
    pub task_edit: bool,
    pub task_delete: bool,
    pub report_view: bool,
    pub report_create: bool,
    pub report_edit: bool,
    pub report_delete: bool,
    pub facility_view: bool,
    pub facility_create: bool,
    pub facility_edit: bool,
    pub facility_delete: bool,
}

#[derive(Deserialize)]
pub struct QueryRole {
    pub id: Uuid
}