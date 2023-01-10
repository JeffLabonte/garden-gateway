CREATE TABLE configuration_dependencies (
    id INTEGER PRIMARY KEY NOT NULL,
    source_configuration_id INTEGER NOT NULL,
    target_configuration_id INTEGER NOT NULL,
    FOREIGN KEY (source_configuration_id) REFERENCES configurations (id) ON DELETE CASCADE,
    FOREIGN KEY (target_configuration_id) REFERENCES configurations (id) ON DELETE CASCADE
);
