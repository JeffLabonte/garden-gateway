use diesel::SqliteConnection;

use crate::context::Context;

use super::{
    configs::list_configs, import::import_schedule_from_json, run_schedule::run,
    schedules::list_schedules, Actions, SubActions,
};

pub fn run_action(mut context: Context) -> bool {
    let arguments = context.arguments;
    let database: &mut SqliteConnection = &mut context.database;

    match arguments.action {
        Actions::Config { sub_action } => match sub_action {
            SubActions::Set { key, value } => {
                print!("Setting: {} -> {}", key, value);
                false
            }
            SubActions::List {} => list_configs(database),
        },
        Actions::Schedule { sub_action } => match sub_action {
            SubActions::Set { key, value } => {
                print!("Setting: {} -> {}", key, value);
                false
            }
            SubActions::List {} => list_schedules(database), // TODO List schedules
        },
        Actions::Run {} => loop {
            match run(database) {
                true => {
                    println!("Run Completed! Let's reload");
                }
                false => {
                    println!("We are done\nStopping now!");
                    return true;
                }
            };
        },
        Actions::Import { schedule_json } => import_schedule_from_json(database, schedule_json),
        Actions::Test { sensor, action } => {
            println!("Running test");
            true
        }
    }
}
