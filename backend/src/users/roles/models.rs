use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[derive(Serialize, FromRow, Type, Clone)]
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

// Details

#[derive(Deserialize)]
pub struct QueryRole {
    pub id: Uuid,
}

// Create

#[derive(Deserialize)]
pub struct NewRole {
    pub name: String,
    pub level: i32,
    pub has_password: Option<bool>,
    pub user_view: Option<bool>,
    pub user_create: Option<bool>,
    pub user_edit: Option<bool>,
    pub user_delete: Option<bool>,
    pub machine_view: Option<bool>,
    pub machine_create: Option<bool>,
    pub machine_edit: Option<bool>,
    pub machine_delete: Option<bool>,
    pub task_view: Option<bool>,
    pub task_create: Option<bool>,
    pub task_edit: Option<bool>,
    pub task_delete: Option<bool>,
    pub report_view: Option<bool>,
    pub report_create: Option<bool>,
    pub report_edit: Option<bool>,
    pub report_delete: Option<bool>,
    pub facility_view: Option<bool>,
    pub facility_create: Option<bool>,
    pub facility_edit: Option<bool>,
    pub facility_delete: Option<bool>,
}

// Update

#[derive(Deserialize)]
pub struct UpdateRole {
    pub id: Uuid,
    pub name: Option<String>,
    pub level: Option<i32>,
    pub has_password: Option<bool>,
    pub user_view: Option<bool>,
    pub user_create: Option<bool>,
    pub user_edit: Option<bool>,
    pub user_delete: Option<bool>,
    pub machine_view: Option<bool>,
    pub machine_create: Option<bool>,
    pub machine_edit: Option<bool>,
    pub machine_delete: Option<bool>,
    pub task_view: Option<bool>,
    pub task_create: Option<bool>,
    pub task_edit: Option<bool>,
    pub task_delete: Option<bool>,
    pub report_view: Option<bool>,
    pub report_create: Option<bool>,
    pub report_edit: Option<bool>,
    pub report_delete: Option<bool>,
    pub facility_view: Option<bool>,
    pub facility_create: Option<bool>,
    pub facility_edit: Option<bool>,
    pub facility_delete: Option<bool>,
}
