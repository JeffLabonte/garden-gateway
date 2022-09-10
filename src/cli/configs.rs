use crate::models::*;
use diesel::prelude::*;
use diesel::result::Error;

pub fn get_configs(database: &mut SqliteConnection) -> Result<Vec<Configuration>, Error> {
    use crate::schema::configurations::dsl::configurations;
    configurations.load::<Configuration>(database)
}

pub fn list_configs(database: &mut SqliteConnection) -> bool {
    let results = get_configs(database).unwrap();
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
