use crate::models::*;
use diesel::prelude::*;


pub fn list_schedules(database: &SqliteConnection) -> bool {
    use crate::schema::configurations::dsl::configurations;
    use crate::schema::schedules::dsl::schedules;

}
