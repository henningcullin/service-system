use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::Nullable;

#[derive(PartialEq, Debug)]
pub enum Field {
    Str(Option<String>),
    Int(Option<i32>),
    Bool(Option<bool>),
    Uuid(Option<Uuid>),
    DateTime(Option<DateTime<Utc>>),
    Ignore,
}

pub trait IntoField {
    fn into_field(self) -> Field;
}

impl IntoField for String {
    fn into_field(self) -> Field {
        Field::Str(Some(self))
    }
}

impl IntoField for i32 {
    fn into_field(self) -> Field {
        Field::Int(Some(self))
    }
}

impl IntoField for bool {
    fn into_field(self) -> Field {
        Field::Bool(Some(self))
    }
}

impl IntoField for Uuid {
    fn into_field(self) -> Field {
        Field::Uuid(Some(self))
    }
}

impl IntoField for DateTime<Utc> {
    fn into_field(self) -> Field {
        Field::DateTime(Some(self))
    }
}

impl<T: IntoField> IntoField for Option<T> {
    fn into_field(self) -> Field {
        match self {
            Some(v) => v.into_field(),
            None => Field::Ignore,
        }
    }
}

impl<T: NullableIntoField> IntoField for Nullable<T> {
    fn into_field(self) -> Field {
        match self {
            Nullable::Absent => Field::Ignore,
            Nullable::Value(v) => v.into_field(),
            Nullable::Null => T::into_field_none(),
        }
    }
}

// Headache starts here

pub trait NullableIntoField: IntoField {
    fn into_field_none() -> Field;
}

impl NullableIntoField for String {
    fn into_field_none() -> Field {
        Field::Str(None)
    }
}

impl NullableIntoField for i32 {
    fn into_field_none() -> Field {
        Field::Int(None)
    }
}

impl NullableIntoField for bool {
    fn into_field_none() -> Field {
        Field::Bool(None)
    }
}

impl NullableIntoField for Uuid {
    fn into_field_none() -> Field {
        Field::Uuid(None)
    }
}

impl NullableIntoField for DateTime<Utc> {
    fn into_field_none() -> Field {
        Field::DateTime(None)
    }
}
