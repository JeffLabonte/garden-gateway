#[macro_use]
extern crate diesel;
extern crate dotenv;

mod cli;
mod database;
mod devices;

use database::establish_connection;
use devices::RelayPowerBar;

fn main() {
    let database = establish_connection();
    println!("It has worked");
}
