#[macro_use]
extern crate diesel;
extern crate dotenv;

mod cli;
pub mod context;
mod database;
mod devices;
pub mod models;
pub mod schema;

use cli::actions::run_action;
use cli::get_cli_args;
use database::establish_connection;
use devices::RelayPowerBar;

use crate::context::Context;

fn main() {
    let context = Context {
        database: establish_connection(),
        arguments: get_cli_args().expect("Invalid Arguments! Please use --help"),
    };
    let is_success = run_action(context);
    println!("{}", is_success);
}
