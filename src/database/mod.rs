use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::{env, thread, time::Duration};

pub mod helpers;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

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

    run_migrations(connection.unwrap())
}

fn run_migrations(mut database_connection: SqliteConnection) -> SqliteConnection {
    database_connection
        .run_pending_migrations(MIGRATIONS)
        .unwrap();

    database_connection
}
