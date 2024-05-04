use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};

pub enum Nullable<T> {
    Absent,
    Null,
    Value(T),
}

impl<T> Default for Nullable<T> {
    fn default() -> Self {
        Self::Absent
    }
}

struct NullableVisitor<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for NullableVisitor<T> {
    type Value = Nullable<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("null or a value")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Nullable::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Nullable::Value)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Nullable<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_option(NullableVisitor {
            _marker: std::marker::PhantomData,
        })
    }
}
