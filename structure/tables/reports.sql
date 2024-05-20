CREATE TABLE report_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE report_statuses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    report_type UUID NOT NULL REFERENCES report_types(id) ON DELETE CASCADE,
    status UUID NOT NULL REFERENCES report_statuses(id) ON DELETE CASCADE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    creator UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    machine UUID REFERENCES machines(id) ON DELETE CASCADE,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    edited TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_edited_time
BEFORE UPDATE ON reports
FOR EACH ROW
EXECUTE PROCEDURE update_edited_column();

CREATE TABLE report_documents (
    report_id UUID NOT NULL REFERENCES reports(id),
    uri VARCHAR(512) NOT NULL,
    name VARCHAR(255),
    description TEXT,
    PRIMARY KEY (report_id, uri)
);

CREATE OR REPLACE FUNCTION delete_report_documents() RETURNS TRIGGER AS $$
BEGIN
  DELETE FROM report_documents WHERE report_id = OLD.id;
  RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_report_documents
BEFORE DELETE ON reports
FOR EACH ROW EXECUTE PROCEDURE delete_report_documents();

CREATE OR REPLACE FUNCTION notify_report_change() RETURNS TRIGGER AS $$
DECLARE
  data json;
  report_id UUID;
  operation_type text;
BEGIN
  IF (TG_OP = 'DELETE') THEN
    report_id := OLD.id;
    operation_type := 'DELETE';
  ELSIF (TG_OP = 'INSERT') THEN
    report_id := NEW.id;
    operation_type := 'INSERT';
  ELSIF (TG_OP = 'UPDATE') THEN
    report_id := NEW.id;
    operation_type := 'UPDATE';
  END IF;
  
  data := json_build_object('id', report_id::text, 'kind', operation_type);
  PERFORM pg_notify('report_changed', data::text);
  
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER report_changed
AFTER INSERT OR UPDATE OR DELETE ON reports
FOR EACH ROW EXECUTE PROCEDURE notify_report_change();