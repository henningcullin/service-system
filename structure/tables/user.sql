CREATE TABLE user (
    id BINARY(16) PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID(), true)),
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255),
    phone VARCHAR(255) UNIQUE,
    role ENUM('Super', 'Administrator', 'Basic', 'Worker') NOT NULL DEFAULT 'Worker',
    last_login TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_password_for_non_worker CHECK (NOT (role != 'Worker' AND password IS NULL))
);

CREATE INDEX idx_email ON user (email);