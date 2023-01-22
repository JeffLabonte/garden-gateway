
INSERT OR IGNORE INTO configuration_dependencies ( source_configuration_id, target_configuration_id )
VALUES 
    (
        (SELECT id FROM configurations WHERE sensor_name = 'water_pump'),
        (SELECT id FROM configurations WHERE sensor_name = 'water_detector') 
    ),
    (
        (SELECT id FROM configurations WHERE sensor_name = 'water_detector'),
        (SELECT id FROM configurations WHERE sensor_name = 'water_pump')
    );
