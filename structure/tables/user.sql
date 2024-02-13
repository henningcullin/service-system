CREATE TABLE user (
    user_id INT PRIMARY KEY AUTO_INCREMENT,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    phone INT UNIQUE,
    role ENUM('Super', 'Administrator', 'Basic') DEFAULT 'Basic'
);
