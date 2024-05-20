CREATE TABLE machine_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE machine_statuses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE facilities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    address VARCHAR(255)
);

CREATE TABLE machines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    make VARCHAR(255),
    machine_type UUID NOT NULL REFERENCES machine_types(id) ON DELETE CASCADE,
    status UUID NOT NULL REFERENCES machine_statuses(id) ON DELETE CASCADE,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    edited TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    facility UUID REFERENCES facilities(id) ON DELETE SET NULL,
    image VARCHAR(512)
);

CREATE TRIGGER update_machines_edited
BEFORE UPDATE ON machines
FOR EACH ROW
EXECUTE PROCEDURE update_edited_column();