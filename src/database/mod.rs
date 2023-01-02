use diesel::prelude::*;
use dotenv::dotenv;
use std::{env, thread, time::Duration};

pub mod helpers;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let mut database_url = env::var("DATABASE_URL").expect("DATBASE_URL must be set");
    if cfg!(test) {
        database_url = ":memory".to_string();
    }

    let mut connection = SqliteConnection::establish(&database_url);

    while connection.is_err() {
        thread::sleep(Duration::new(1, 0));
        connection = SqliteConnection::establish(&database_url);
    }

    connection.unwrap()
}
