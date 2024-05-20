CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255),
    phone VARCHAR(255) UNIQUE,
    role UUID NOT NULL REFERENCES roles(id) ON DELETE RESTRICT,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    last_login TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    occupation VARCHAR(255),
    image VARCHAR(512),
    facility UUID REFERENCES facilities(id) ON DELETE SET NULL
);

CREATE OR REPLACE FUNCTION is_password_required(role_id UUID, user_password VARCHAR)
RETURNS BOOLEAN AS $$
DECLARE
    role_has_password BOOLEAN;
BEGIN
    SELECT INTO role_has_password has_password FROM roles WHERE id = role_id;
    
    RETURN (NOT role_has_password OR user_password IS NOT NULL);
END;
$$ LANGUAGE plpgsql;

ALTER TABLE users
ADD CONSTRAINT is_password_required_constraint
CHECK (is_password_required(role, password));

CREATE INDEX idx_email ON users(email);

INSERT INTO users (first_name, last_name, email, phone, role, occupation)
VALUES ('Service', 'Systems', 'henning@email.com', '123456789', (SELECT id FROM roles WHERE name = 'Super'), 'Administration');
