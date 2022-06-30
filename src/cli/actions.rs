use crate::context::Context;

use super::{configs::list_configs, run::run};

pub fn run_action(context: Context) -> bool {
    let arguments = context.arguments;

    match arguments.action.as_str() {
        "config" => match arguments.sub_action.as_str() {
            "list" => list_configs(context.database),
            _ => false,
        },
        "run" => run(context.database),
        _ => false,
    }
}
