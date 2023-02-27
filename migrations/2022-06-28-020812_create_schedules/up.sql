-- Your SQL goes herea
CREATE TABLE schedules (
    id INTEGER PRIMARY KEY NOT NULL UNIQUE,
    action TEXT NOT NULL,
    cron_string TEXT NOT NULL
)
