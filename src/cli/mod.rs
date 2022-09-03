pub mod actions;
pub mod configs;
pub mod import;
mod run_schedule;
pub mod schedules;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "Garden Gateway CLI")]
#[clap(about = "CLI to interact with Garden Gateway code.")]
pub struct CLIArgs {
    #[clap(subcommand)]
    pub action: Actions,
}

#[derive(Subcommand)]
pub enum Actions {
    #[clap(arg_required_else_help = true)]
    Config {
        #[clap(subcommand)]
        sub_action: SubActions,
    },

    #[clap(arg_required_else_help = true)]
    Schedule {
        #[clap(subcommand)]
        sub_action: SubActions,
    },

    #[clap(help = "Run the code with the schedule and configurations")]
    Run {},

    #[clap(arg_required_else_help = true)]
    #[clap(help = "Schedule to import")]
    Import {
        #[clap(value_parser)]
        schedule_json: PathBuf,
    },

    #[clap(help = "Allow you to test your electronic setup")]
    Test {
        #[clap(short = 's')]
        sensor: String,
        action: String,
    },
}

#[derive(Subcommand)]
pub enum SubActions {
    Set {
        #[clap(short = 'k', required = true, value_parser)]
        key: String,
        #[clap(short = 'v', required = true, value_parser)]
        value: String,
    },

    List {},
}

pub fn get_cli_args() -> CLIArgs {
    CLIArgs::parse()
}
