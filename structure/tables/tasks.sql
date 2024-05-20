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
    task_type UUID NOT NULL REFERENCES task_types(id) ON DELETE CASCADE,
    status UUID NOT NULL REFERENCES task_statuses(id) ON DELETE CASCADE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    creator UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    machine UUID REFERENCES machines(id) ON DELETE CASCADE,
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

CREATE OR REPLACE FUNCTION delete_task_executors() RETURNS TRIGGER AS $$
BEGIN
  DELETE FROM task_executors WHERE task_id = OLD.id;
  RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_task_executors
BEFORE DELETE ON tasks
FOR EACH ROW EXECUTE PROCEDURE delete_task_executors();

CREATE OR REPLACE FUNCTION delete_task_documents() RETURNS TRIGGER AS $$
BEGIN
  DELETE FROM task_documents WHERE task_id = OLD.id;
  RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_task_documents
BEFORE DELETE ON tasks
FOR EACH ROW EXECUTE PROCEDURE delete_task_documents();


-- NOTIFICATIONS ON TASK CHANGES

CREATE OR REPLACE FUNCTION notify_task_change() RETURNS TRIGGER AS $$
DECLARE
  data json;
  task_id UUID;
  operation_type text;
BEGIN
  IF (TG_OP = 'DELETE') THEN
    task_id := OLD.id;
    operation_type := 'DELETE';
  ELSIF (TG_OP = 'INSERT') THEN
    task_id := NEW.id;
    operation_type := 'INSERT';
  ELSIF (TG_OP = 'UPDATE') THEN
    task_id := NEW.id;
    operation_type := 'UPDATE';
  END IF;
  
  data := json_build_object('id', task_id::text, 'kind', operation_type);
  PERFORM pg_notify('task_changed', data::text);
  
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER task_changed
AFTER INSERT OR UPDATE OR DELETE ON tasks
FOR EACH ROW EXECUTE PROCEDURE notify_task_change();