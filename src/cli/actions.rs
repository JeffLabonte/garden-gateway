use crate::context::Context;

use super::{
    configs::list_configs, import::import_schedule_from_json, run::run, Actions, SubActions,
};

pub fn run_action(context: Context) -> bool {
    let arguments = context.arguments;

    match arguments.action {
        Actions::Config { sub_action } => match sub_action {
            SubActions::Set { key, value } => {
                print!("Setting: {} -> {}", key, value);
                false
            }
            SubActions::List {} => list_configs(&context.database),
        },
        Actions::Schedule { sub_action } => match sub_action {
            SubActions::Set { key, value } => {
                print!("Setting: {} -> {}", key, value);
                false
            }
            SubActions::List {} => false, // TODO List schedules
        },
        Actions::Run {} => loop {
            match run(&context.database) {
                true => {
                    println!("Run Completed! Let's reload");
                }
                false => {
                    println!("We are done\nStopping now!");
                    return true;
                }
            };
        },
        Actions::Import { schedule_json } => {
            import_schedule_from_json(context.database, schedule_json)
        }
        Actions::Test { sensor, action } => {
            println!("Running test");
            true
        }
    }
}
