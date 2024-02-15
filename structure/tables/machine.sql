CREATE TABLE machine (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    machine_type VARCHAR(255),
    status ENUM('Active', 'Inactive') DEFAULT 'Inactive'
);
