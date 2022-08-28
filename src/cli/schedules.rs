use crate::models::*;
use diesel::prelude::*;

pub fn list_schedules(database: &SqliteConnection) -> bool {
    use crate::schema::configurations::dsl::configurations;
    use crate::schema::schedules::dsl::schedules;

    let schedule_vec = schedules
        .load::<Schedule>(database)
        .expect("Something went wrong");

    for schedule in schedule_vec {
        println!("************************\n");
        println!("Schedules: {}", schedule.id);
        println!("************************\n");
    }

    return true;
}
