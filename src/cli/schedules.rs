use crate::models::*;
use diesel::prelude::*;

pub fn list_schedules(database: &SqliteConnection) -> bool {
    use crate::schema::configurations::dsl::configurations;
    use crate::schema::schedule_configurations::dsl::{schedule_configurations, schedule_id};
    use crate::schema::schedules::dsl::schedules;

    let schedule_vec = schedules
        .load::<Schedule>(database)
        .expect("Something went wrong");

    for schedule in schedule_vec {
        println!("************************\n");
        println!("Schedules: {}", schedule.id);
        let schedule_configs = schedule_configurations
            .filter(schedule_id.eq(schedule.id))
            .load::<ScheduleConfiguration>(database)
            .expect("Unable to retrieve ScheduleConfigurations");

        for schedule_config in schedule_configs {
            println!("The Schedule Config: {}", schedule_config.id);
        }
        println!("************************\n");
    }

    true
}
