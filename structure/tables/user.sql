CREATE TABLE user (
    id BINARY(16) PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID(), true)),
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NULL,
    phone VARCHAR(255) UNIQUE,
    role ENUM('Super', 'Administrator', 'Basic', 'Worker') NOT NULL DEFAULT 'Worker',
    active BOOLEAN NOT NULL DEFAULT TRUE,
    last_login TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (password IS NULL OR (role NOT IN ('Worker', 'Super') AND password IS NOT NULL) OR (role IN ('Worker', 'Super') AND password IS NULL))
);

CREATE INDEX idx_email ON user (email);

INSERT INTO user (first_name, last_name, email, phone, role)
VALUES ('Axami', 'Systems', 'henade0401@edu.halmstad.se', '0761999773', 'Super');