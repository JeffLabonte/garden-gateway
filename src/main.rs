#[macro_use]
extern crate diesel;
extern crate dotenv;

mod cli;
mod context;
mod database;
mod devices;
mod import;
mod models;
mod schema;

use cli::actions::run_action;
use cli::get_cli_args;
use database::establish_connection;

use crate::context::Context;

fn main() {
    let context = Context {
        database: establish_connection(),
        arguments: get_cli_args(),
    };
    let is_success = run_action(context);
    if !is_success {
        eprintln!("Something went wrong");
    }
}
