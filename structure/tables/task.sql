CREATE TABLE task (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    task_type ENUM('Suggestion', 'Issue', 'Service', 'Other') NOT NULL DEFAULT 'Other',
    status ENUM('Pending', 'In Progress', 'Completed') NOT NULL DEFAULT 'Pending',
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    edited TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    creator CHAR(36) NOT NULL,
    executor CHAR(36),
    machine CHAR(36),
    FOREIGN KEY (creator) REFERENCES user(id),
    FOREIGN KEY (executor) REFERENCES user(id),
    FOREIGN KEY (machine) REFERENCES machine(id)
);
