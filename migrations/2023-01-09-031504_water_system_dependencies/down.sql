DELETE FROM water_system_dependencies 
WHERE source_configuration_id IN (
    SELECT id FROM configurations where sensor_name IN (
        'water_detector',
        'water_pump'
    )
);
