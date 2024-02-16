CREATE TABLE machine (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    machine_type VARCHAR(255),
    status ENUM('Active', 'Inactive') DEFAULT 'Inactive'
);
