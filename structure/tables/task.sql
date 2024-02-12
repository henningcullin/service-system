CREATE TABLE task (
    task_id INT PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status ENUM('Pending', 'In Progress', 'Completed') DEFAULT 'Pending',
    created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    edited TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    creator INT NOT NULL REFERENCES user(user_id),
    executor INT NOT NULL REFERENCES user(user_id),
    machine INT REFERENCES machine(machine_id)
);