#[macro_use]
extern crate diesel;
extern crate dotenv;

mod cli;
mod database;
mod devices;

use cli::get_cli_args;
use database::establish_connection;
use devices::RelayPowerBar;

fn main() {
    let database = establish_connection();
    let arguments = get_cli_args().expect("Invalid Arguments! Please use --help");
    print!("Action: {}", arguments.action);
}
