-- This file should undo anything in `up.sql`
DELETE FROM configurations
WHERE sensor_name = 'relay_power';
