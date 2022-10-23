use crate::models::*;
use crate::DATABASE_CONNECTION;
use diesel::prelude::*;
use diesel::result::Error;

pub fn get_configs() -> Result<Vec<Configuration>, Error> {
    use crate::schema::configurations::dsl::configurations;
    let database_connection: &mut SqliteConnection = &mut DATABASE_CONNECTION.lock().unwrap();
    configurations.load::<Configuration>(database_connection)
}

pub fn list_configs() -> bool {
    let results = get_configs().unwrap();
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
