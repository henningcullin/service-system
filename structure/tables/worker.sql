CREATE TABLE worker (
    worker_id INT PRIMARY KEY AUTO_INCREMENT,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255),
    email VARCHAR(255) NOT NULL,
    phone INT,
    company VARCHAR(255) NOT NULL
);
