use crate::context::Context;

use super::{
    configs::list_configs, import::import_schedule_from_json, run_schedule::run,
    schedules::list_schedules, Actions, SubActions,
};

pub fn run_action(context: Context) -> bool {
    let arguments = context.arguments;

    match arguments.action {
        Actions::Config { sub_action } => match sub_action {
            SubActions::Set { key, value } => {
                print!("Setting: {key} -> {value}");
                false
            }
            SubActions::List {} => list_configs(),
        },
        Actions::Schedule { sub_action } => match sub_action {
            SubActions::Set { key, value } => {
                print!("Setting: {key} -> {value}");
                false
            }
            SubActions::List {} => list_schedules(),
        },
        Actions::Run {} => loop {
            match run() {
                true => {
                    println!("Run Completed! Let's reload");
                }
                false => {
                    println!("We are done\nStopping now!");
                    return true;
                }
            };
        },
        Actions::Import { schedule_json } => import_schedule_from_json(schedule_json),
        Actions::Test {
            sensor: _,
            action: _,
        } => {
            println!("Running test");
            true
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        cli::{Actions, CLIArgs},
        context::Context,
    };

    use super::run_action;

    #[test]
    fn test_given_run_action_when_run_in_arguments_should_execute_run() {
        let arguments: CLIArgs = CLIArgs {
            action: Actions::Run {},
        };
        let mock_context = Context { arguments };

        let run_sucessfully: bool = run_action(mock_context);

        assert_eq!(run_sucessfully, true);
    }
}
