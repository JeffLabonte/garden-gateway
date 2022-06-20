use diesel::prelude::*;

use crate::cli::CLIArgs;

pub struct Context {
    pub database: SqliteConnection,
    pub arguments: CLIArgs,
}
