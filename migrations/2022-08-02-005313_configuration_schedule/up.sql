-- Your SQL goes here
CREATE TABLE schedule_configurations (
    id INTEGER PRIMARY KEY NOT NULL,
    configuration_id INTEGER NOT NULL,
    schedule_id INTEGER NOT NULL,
    FOREIGN KEY (configuration_id) REFERENCES configurations (id) ON DELETE CASCADE,
    FOREIGN KEY (schedule_id) REFERENCES scheudles (id) ON DELETE CASCADE
)
