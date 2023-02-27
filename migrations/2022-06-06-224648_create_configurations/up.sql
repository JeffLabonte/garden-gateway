-- Your SQL goes here
CREATE TABLE configurations (
    id INTEGER PRIMARY KEY NOT NULL UNIQUE,
    sensor_name TEXT NOT NULL,
    bcm_pin INTEGER UNIQUE NOT NULL
)
