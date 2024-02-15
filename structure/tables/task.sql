CREATE TABLE task (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status ENUM('Pending', 'In Progress', 'Completed') DEFAULT 'Pending',
    created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    edited TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    creator INT UNSIGNED NOT NULL,
    executor INT UNSIGNED,
    machine INT UNSIGNED,
    CONSTRAINT fk_creator FOREIGN KEY (creator) REFERENCES user(user_id),
    CONSTRAINT fk_executor FOREIGN KEY (executor) REFERENCES user(user_id),
    CONSTRAINT fk_machine FOREIGN KEY (machine) REFERENCES machine(machine_id)
);
