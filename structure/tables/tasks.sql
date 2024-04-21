CREATE TABLE task_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE task_statuses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    task_type UUID NOT NULL REFERENCES task_types(id),
    status UUID NOT NULL REFERENCES task_statuses(id),
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    creator UUID NOT NULL REFERENCES users(id),
    machine UUID REFERENCES machines(id),
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    edited TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    due_at TIMESTAMPTZ
);

CREATE TRIGGER update_task_edited
BEFORE UPDATE ON tasks
FOR EACH ROW
EXECUTE PROCEDURE update_edited_column();

CREATE TABLE task_executors (
    task_id UUID NOT NULL REFERENCES tasks(id),
    user_id UUID NOT NULL REFERENCES users(id),
    PRIMARY KEY (task_id, user_id)
);

CREATE TABLE task_documents (
    task_id UUID NOT NULL REFERENCES tasks(id),
    uri VARCHAR(512) NOT NULL,
    name VARCHAR(255),
    description TEXT,
    PRIMARY KEY (task_id, uri)
);
