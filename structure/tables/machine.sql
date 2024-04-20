CREATE TABLE machine_type (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE machine_status (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE facility (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    address VARCHAR(255)
);

CREATE TABLE machine (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    make VARCHAR(255),
    machine_type UUID NOT NULL REFERENCES machine_type(id),
    status UUID NOT NULL REFERENCES machine_status(id),
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    edited TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    facility UUID NOT NULL REFERENCES facility(id),
    image VARCHAR(512)
);

CREATE TRIGGER update_machine_edited
BEFORE UPDATE ON machine
FOR EACH ROW
EXECUTE PROCEDURE update_edited_column();