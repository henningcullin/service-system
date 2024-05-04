use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::errors::{ApiError, ForbiddenReason};

pub enum Field {
    Str(Option<String>),
    Int(Option<i32>),
    Bool(Option<bool>),
    Uuid(Option<Uuid>),
    DateTime(Option<DateTime<Utc>>),
}

impl From<String> for Field {
    fn from(value: String) -> Self {
        Field::Str(Some(value))
    }
}

impl From<i32> for Field {
    fn from(value: i32) -> Self {
        Field::Int(Some(value))
    }
}

impl From<bool> for Field {
    fn from(value: bool) -> Self {
        Field::Bool(Some(value))
    }
}

impl From<Uuid> for Field {
    fn from(value: Uuid) -> Self {
        Field::Uuid(Some(value))
    }
}

impl From<Option<DateTime<Utc>>> for Field {
    fn from(value: Option<DateTime<Utc>>) -> Self {
        Field::DateTime(value)
    }
}

impl From<Option<String>> for Field {
    fn from(value: Option<String>) -> Self {
        Field::Str(value)
    }
}

impl From<Option<i32>> for Field {
    fn from(value: Option<i32>) -> Self {
        Field::Int(value)
    }
}

impl From<Option<bool>> for Field {
    fn from(value: Option<bool>) -> Self {
        Field::Bool(value)
    }
}

impl From<Option<Uuid>> for Field {
    fn from(value: Option<Uuid>) -> Self {
        Field::Uuid(value)
    }
}

pub fn check_permission(permission: bool) -> Result<(), ApiError> {
    if !permission {
        return Err(ApiError::Forbidden(ForbiddenReason::MissingPermission));
    }
    Ok(())
}
