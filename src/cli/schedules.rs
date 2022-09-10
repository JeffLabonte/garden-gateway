use crate::models::*;
use diesel::{associations::HasTable, prelude::*};

pub fn list_schedules(database: &mut SqliteConnection) -> bool {
    use crate::schema::configurations::dsl::{bcm_pin, configurations, id, sensor_name};
    use crate::schema::schedule_configurations::dsl::{schedule_configurations, schedule_id};
    use crate::schema::schedules::dsl::schedules;

    let schedule_vec = schedules
        .load::<Schedule>(database)
        .expect("Something went wrong");

    for schedule in schedule_vec {
        println!("************************\n");
        println!("Schedules: {}", schedule.id);
        println!("Cron String in UTC : {}", schedule.cron_string);
        let configs = configurations
            .inner_join(schedule_configurations::table())
            .filter(schedule_id.eq(schedule.id))
            .select((bcm_pin, sensor_name, id))
            .load::<Configuration>(database)
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
