use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};
use gateway::database::helpers::get_database_connection;
use gateway::models::Configuration;

fn create_configuration(sensor_name: String, device_pin: i32) -> Configuration {
    let database_connection: &mut SqliteConnection = &mut get_database_connection();
    let new_configuration: NewConfiguration = NewConfiguration {
        sensor_name,
        bcm_pin: device_pin,
    };

    let inserted_rows = diesel::insert_into(schema::configurations::table)
        .values(new_configuration)
        .execute(database_connection)
        .unwrap();

    assert_eq!(inserted_rows, 1);

    schema::configurations::dsl::configurations
        .filter(schema::configurations::dsl::bcm_pin.eq(device_pin))
        .first::<Configuration>(database_connection)
        .unwrap()
}

fn create_schedule(cron: String, action: String) -> Schedule {
    let database_connection: &mut SqliteConnection = &mut get_database_connection();
    let new_schedule = NewSchedule {
        cron_string: cron.to_string(),
        action,
    };

    diesel::insert_into(schedules::table)
        .values(new_schedule)
        .execute(database_connection)
        .expect("Unable to create schedule");

    crate::schema::schedules::dsl::schedules
        .filter(crate::schema::schedules::dsl::cron_string.eq(cron))
        .first::<Schedule>(database_connection)
        .unwrap()
}

fn link_configuration_to_schedule(schedule_id: i32, configuration_id: i32) {
    use crate::schema::schedule_configurations;

    let database_connection: &mut SqliteConnection = &mut get_database_connection();
    let new_schedule_configuration = crate::models::NewScheduleConfiguration {
        schedule_id,
        configuration_id,
    };

    diesel::insert_into(schedule_configurations::table)
        .values(new_schedule_configuration)
        .execute(database_connection)
        .expect("Unable to create schedule configuration");
}
