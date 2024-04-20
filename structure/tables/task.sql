CREATE TABLE task_type (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE task_status (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE task (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    task_type UUID NOT NULL REFERENCES task_type(id),
    status UUID NOT NULL REFERENCES task_status(id),
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    creator UUID NOT NULL REFERENCES user(id),
    machine UUID REFERENCES machine(id),
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    edited TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    due_at TIMESTAMPTZ
);

CREATE TRIGGER update_task_edited
BEFORE UPDATE ON task
FOR EACH ROW
EXECUTE PROCEDURE update_edited_column();

CREATE TABLE task_executor (
    task_id UUID NOT NULL REFERENCES task(id),
    user_id UUID NOT NULL REFERENCES user(id),
    PRIMARY KEY (task_id, user_id)
);

CREATE TABLE task_document (
    task_id UUID NOT NULL REFERENCES task(id),
    uri VARCHAR(512) NOT NULL,
    name VARCHAR(255),
    description TEXT,
    PRIMARY KEY (task_id, uri)
);
