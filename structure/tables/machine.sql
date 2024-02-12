CREATE TABLE machine (
    machine_id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(255),
    status ENUM('Active', 'Inactive') DEFAULT 'Inactive',
    location INT NOT NULL REFERENCES facility(facility_id)
);