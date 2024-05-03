use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;
use validator::Validate;

use crate::machines::facilities::Facility;

use super::roles::models::Role;

#[derive(Serialize, Clone)]
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

// Short variant

#[derive(Type, FromRow, Serialize, Debug)]
pub struct ShortUser {
    pub id: Option<Uuid>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
}

// Details

#[derive(Deserialize)]
pub struct QueryUser {
    pub id: Uuid,
}

// Create

#[derive(Validate, Deserialize)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    pub password: Option<String>,
    pub phone: Option<String>,
    pub role: Uuid,
    pub active: Option<bool>,
    pub occupation: Option<String>,
    pub image: Option<String>,
    pub facility: Option<Uuid>,
}
