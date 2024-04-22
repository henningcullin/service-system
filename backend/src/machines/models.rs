use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Machine {
    id: Uuid,
    name: String,
    make: Option<String>,
    machine_type: MachineType,
    status: MachineStatus,
    created: DateTime<Utc>,
    edited: DateTime<Utc>,
    facility: Option<Facility>,
    image: Option<String>,
}

pub struct MachineType {
    id: Uuid,
    name: String,
}

pub struct MachineStatus {
    id: Uuid,
    name: String,
}

pub struct Facility {
    id: Uuid,
    name: String,
    address: Option<String>,
}

pub struct MachineShort {
    id: Uuid,
    name: String,
    make: Option<String>,
    image: String,
}