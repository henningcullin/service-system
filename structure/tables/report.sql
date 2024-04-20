CREATE TABLE report_type (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
);

CREATE TABLE report_status (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
);

CREATE TABLE report (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    report_type UUID NOT NULL REFERENCES report_type(id),
    status UUID NOT NULL REFERENCES report_status(id),
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    creator UUID NOT NULL REFERENCES user(id),
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    edited TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_edited_time
BEFORE UPDATE ON report
FOR EACH ROW
EXECUTE PROCEDURE update_edited_column();

CREATE TABLE report_document (
    report_id UUID NOT NULL REFERENCES report(id),
    uri VARCHAR(512) NOT NULL,
    name VARCHAR(255),
    description TEXT,
    PRIMARY KEY (report_id, uri)
);
