CREATE TABLE task (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status ENUM('Pending', 'In Progress', 'Completed') DEFAULT 'Pending',
    created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    edited TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    creator INT UNSIGNED NOT NULL,
    executor INT UNSIGNED,
    machine INT UNSIGNED,
    FOREIGN KEY (creator) REFERENCES user(id),
    FOREIGN KEY (executor) REFERENCES user(id),
    FOREIGN KEY (machine) REFERENCES machine(id)
);
