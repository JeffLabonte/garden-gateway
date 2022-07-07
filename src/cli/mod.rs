pub mod actions;
mod configs;
mod run;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "Garden Gateway CLI")]
#[clap(about = "CLI to interact with Garden Gateway code.")]
pub struct CLIArgs {
    // #[clap(
    //     long,
    //     help = "Must be one of the following: config, run, import",
    //     required = true
    // )]
    // pub action: String,

    // #[clap(short = 's', required = false, default_value_t = String::from(""))]
    // pub sub_action: String,

    // #[clap(long = "key", help = "Key to pair with Value to set into Configurations", value_parser, default_value_t = String::from(""))]
    // pub key: String,

    // #[clap(long = "value", help = "Value to set on Key into Configurations", value_parser, default_value_t = String::from(""))]
    // pub value: String,

    // #[clap(
    //     short = 'i',
    //     long = "input",
    //     required = false,
    //     default_value = "/",
    //     value_parser
    // )]
    // pub import_json_path: PathBuf,
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
    Run {},

    #[clap(arg_required_else_help = true)]
    Import {
        #[clap(value_parser)]
        json_file: PathBuf,
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
