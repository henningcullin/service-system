CREATE TABLE user (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    phone INT UNSIGNED UNIQUE,
    role ENUM('Super', 'Administrator', 'Basic') DEFAULT 'Basic'
);
