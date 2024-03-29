CREATE TABLE machine (
    id BINARY(16) PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID(), true)),
    name VARCHAR(255) NOT NULL,
    make VARCHAR(255),
    machine_type VARCHAR(255),
    status ENUM('Active', 'Inactive') NOT NULL DEFAULT 'Inactive',
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    edited TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
