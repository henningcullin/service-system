CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255),
    phone VARCHAR(255) UNIQUE,
    role UUID NOT NULL REFERENCES roles(id),
    active BOOLEAN NOT NULL DEFAULT TRUE,
    last_login TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    occupation VARCHAR(255),
    image VARCHAR(512),
    facility UUID REFERENCES facilities(id)
);

CREATE INDEX idx_email ON users(email);

INSERT INTO users (first_name, last_name, email, phone, role, occupation)
VALUES ('Service', 'Systems', 'henning@email.com', '123456789', (SELECT id FROM roles WHERE name = 'Super'), 'Administration');
