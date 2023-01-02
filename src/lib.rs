#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod cli;
pub mod context;
pub mod database;
pub mod models;
pub mod schema;

mod constants;
mod devices;
mod helpers;

use database::establish_connection;
use diesel::SqliteConnection;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref DATABASE_CONNECTION: Mutex<SqliteConnection> =
        Mutex::new(establish_connection());
}
