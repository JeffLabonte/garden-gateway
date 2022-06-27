use crate::context::Context;
use crate::models::*;
use diesel::prelude::*;

fn list_configs(database: SqliteConnection) -> bool {
    use crate::schema::configurations::dsl::configurations;
    let results = configurations
        .load::<Configuration>(&database)
        .expect("Error loading configurations");

    println!("Displaying {} configs", results.len());

    println!("Configurations:");
    for config in results {
        println!("\n************************\n");
        println!("Sensor Name: {}", config.sensor_name);
        println!("Pin Number Configured: {}\n", config.bcm_pin);
        println!("************************");
    }

    true
}

pub fn run_action(context: Context) -> bool {
    let arguments = context.arguments;

    match arguments.action.as_str() {
        "config" => match arguments.sub_action.as_str() {
            "list" => list_configs(context.database),
            _ => false,
        },
        _ => false,
    }
}
