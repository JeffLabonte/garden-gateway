#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod cli;
pub mod context;
pub mod database;
pub mod exception;
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

#[cfg(test)]
pub mod test {
    use diesel::{sql_query, RunQueryDsl, SqliteConnection};

    use crate::database::helpers::get_database_connection;

    pub fn teardown() {
        let database_connection: &mut SqliteConnection = &mut get_database_connection();
        match sql_query("DELETE FROM schedules").execute(database_connection) {
            Ok(_) => match sql_query("DELETE FROM configurations").execute(database_connection) {
                Ok(_) => (),
                Err(error) => panic!("{error}"),
            },
            Err(error) => panic!("{error}"),
        };
    }
}
