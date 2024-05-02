use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::machines::facilities::Facility;

use super::roles::models::Role;

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub role: Role,
    pub active: bool,
    pub last_login: DateTime<Utc>,
    pub occupation: Option<String>,
    pub image: Option<String>,
    pub facility: Option<Facility>,
}

// Details

#[derive(Deserialize)]
pub struct QueryUser {
    pub id: Uuid,
}
