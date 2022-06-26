use crate::context::Context;
use crate::models::*;
use diesel::prelude::*;

fn list_configs(database: SqliteConnection) -> bool {
    use crate::schema::configurations::dsl::configurations;
    let result = configurations
        .load::<Configuration>(&database)
        .expect("Error loading configurations");

    println!("Displaying {} configs", result.len());

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
