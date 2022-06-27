#[macro_use]
extern crate diesel;
extern crate dotenv;

mod cli;
pub mod context;
mod database;
mod devices;
pub mod models;
pub mod schema;

use std::process;

use cli::actions::run_action;
use cli::get_cli_args;
use database::establish_connection;

use crate::context::Context;

fn main() {
    let context = Context {
        database: establish_connection(),
        arguments: match get_cli_args() {
            Ok(x) => x,
            Err(e) => {
                eprint!("{}. Please use --help", e);
                process::exit(0x01)
            }
        },
    };
    let is_success = run_action(context);
    if !is_success {
        eprintln!("Something went wrong");
    }
}
