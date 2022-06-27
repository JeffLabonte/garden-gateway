use crate::models::*;
use diesel::prelude::*;

pub fn run(database: SqliteConnection) -> bool {
    let configurations = configurations
        .load::<Configuration>(&database)
        .expect("Error loading configurations");

    true
}
