use crate::models::*;
use diesel::prelude::*;

pub fn list_configs(database: SqliteConnection) -> bool {
    use crate::schema::configurations::dsl::configurations;
    let results = configurations
        .load::<Configuration>(&database)
        .expect("Error loading configurations");

    println!("Displaying {} configs", results.len());

    println!("Configurations:\n");
    for config in results {
        println!("************************\n");
        println!("Configuration ID: {}", config.id);
        println!("Sensor Name: {}", config.sensor_name);
        println!("Pin Number Configured: {}\n", config.bcm_pin);
        println!("************************\n");
    }

    true
}
