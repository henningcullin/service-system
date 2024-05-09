CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    level INT NOT NULL UNIQUE CHECK (level >= 0),
    has_password BOOLEAN NOT NULL DEFAULT TRUE,
    user_view BOOLEAN NOT NULL DEFAULT FALSE,
    user_create BOOLEAN NOT NULL DEFAULT FALSE,
    user_edit BOOLEAN NOT NULL DEFAULT FALSE,
    user_delete BOOLEAN NOT NULL DEFAULT FALSE,
    machine_view BOOLEAN NOT NULL DEFAULT FALSE,
    machine_create BOOLEAN NOT NULL DEFAULT FALSE,
    machine_edit BOOLEAN NOT NULL DEFAULT FALSE,
    machine_delete BOOLEAN NOT NULL DEFAULT FALSE,
    task_view BOOLEAN NOT NULL DEFAULT FALSE,
    task_create BOOLEAN NOT NULL DEFAULT FALSE,
    task_edit BOOLEAN NOT NULL DEFAULT FALSE,
    task_delete BOOLEAN NOT NULL DEFAULT FALSE,
    report_view BOOLEAN NOT NULL DEFAULT FALSE,
    report_create BOOLEAN NOT NULL DEFAULT FALSE,
    report_edit BOOLEAN NOT NULL DEFAULT FALSE,
    report_delete BOOLEAN NOT NULL DEFAULT FALSE,
    facility_view BOOLEAN NOT NULL DEFAULT FALSE,
    facility_create BOOLEAN NOT NULL DEFAULT FALSE,
    facility_edit BOOLEAN NOT NULL DEFAULT FALSE,
    facility_delete BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO roles (
    name, level, has_password, user_view, user_create, user_edit, user_delete, 
    machine_view, machine_create, machine_edit, machine_delete, task_view, task_create, task_edit, task_delete, 
    report_view, report_create, report_edit, report_delete, facility_view, facility_create, facility_edit, facility_delete
) VALUES (
    'Super', 0, FALSE, TRUE, TRUE, TRUE, TRUE, 
    TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, 
    TRUE, TRUE, TRUE,  TRUE, TRUE, TRUE, TRUE, TRUE
);
