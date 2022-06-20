use diesel::prelude::*;

use crate::cli::CLIArgs;

pub struct Context {
    database: SqliteConnection,
    arguments: CLIArgs,
}
