#[macro_use]
extern crate diesel;
extern crate dotenv;

mod cli;
pub mod context;
mod database;
mod devices;
mod helpers;
mod models;
mod schema;

use lazy_static::lazy_static;
use std::sync::Mutex;

use cli::actions::run_action;
use cli::get_cli_args;
use database::establish_connection;
use diesel::SqliteConnection;

use crate::context::Context;

lazy_static! {
    pub static ref DATABASE_CONNECTION: Mutex<SqliteConnection> =
        Mutex::new(establish_connection());
}

fn main() {
    let context = Context {
        arguments: get_cli_args(),
    };

    if !run_action(context) {
        eprintln!("Something went wrong");
    }
}
