use crate::models::*;
use crate::DATABASE_CONNECTION;
use diesel::{associations::HasTable, prelude::*};

pub fn list_schedules() -> bool {
    use crate::schema::configurations::dsl::{bcm_pin, configurations, id, sensor_name};
    use crate::schema::schedule_configurations::dsl::{schedule_configurations, schedule_id};
    use crate::schema::schedules::dsl::schedules;

    let mut database_connection: &mut SqliteConnection = &mut DATABASE_CONNECTION.lock().unwrap();
    let schedule_vec = schedules
        .load::<Schedule>(database_connection)
        .expect("Something went wrong");

    for schedule in schedule_vec {
        println!("************************\n");
        println!("Schedules: {}", schedule.id);
        println!("Cron String in UTC : {}", schedule.cron_string);
        let configs = configurations
            .inner_join(schedule_configurations::table())
            .filter(schedule_id.eq(schedule.id))
            .select((bcm_pin, sensor_name, id))
            .load::<Configuration>(database_connection)
            .expect("Unable to load schedules_configurations");

        for conf in configs {
            println!("Config ID: {}", conf.id);
            println!("Pin: {}", conf.bcm_pin);
            println!("Sensor Name: {}", conf.sensor_name);
        }
        println!("************************\n");
    }

    true
}
