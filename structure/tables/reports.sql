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
    report_type UUID NOT NULL REFERENCES report_types(id),
    status UUID NOT NULL REFERENCES report_statuses(id),
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    creator UUID NOT NULL REFERENCES users(id),
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
