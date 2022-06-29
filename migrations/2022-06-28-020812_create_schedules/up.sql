-- Your SQL goes herea
CREATE TABLE schedules (
    id INTEGER PRIMARY KEY NOT NULL,
    action TEXT NOT NULL,
    cron_string TEXT NOT NULL,
    configuration_id INTEGER NOT NULL,
    FOREIGN KEY (configuration_id) REFERENCES configurations (id) ON DELETE CASCADE
)
