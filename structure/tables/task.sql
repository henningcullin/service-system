CREATE TABLE task (
    id BINARY(16) PRIMARY KEY DEFAULT (UUID_TO_BIN(UUID(), true)),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    task_type ENUM('Suggestion', 'Issue', 'Service', 'Other') NOT NULL DEFAULT 'Other',
    status ENUM('Pending', 'Active', 'Completed') NOT NULL DEFAULT 'Pending',
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    edited TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    creator BINARY(16) NOT NULL,
    executor BINARY(16),
    machine BINARY(16),
    FOREIGN KEY (creator) REFERENCES user(id),
    FOREIGN KEY (executor) REFERENCES user(id),
    FOREIGN KEY (machine) REFERENCES machine(id)
);