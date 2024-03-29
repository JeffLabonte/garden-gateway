use std::panic;

use diesel::{prelude::*, sql_query};
use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};
use gateway::database::helpers::get_database_connection;
use gateway::models::{
    Configuration, NewConfiguration, NewSchedule, NewScheduleConfiguration, Schedule,
};
use gateway::schema::{self, configurations, schedules};

pub fn create_configuration(sensor_name: String, device_pin: i32) -> Configuration {
    let database_connection: &mut SqliteConnection = &mut get_database_connection();
    let new_configuration: NewConfiguration = NewConfiguration {
        sensor_name,
        bcm_pin: device_pin,
    };

    let inserted_rows = diesel::insert_or_ignore_into(schema::configurations::table)
        .values(new_configuration)
        .execute(database_connection)
        .unwrap();

    println!("BCM Pin: {}", device_pin);
    assert_eq!(inserted_rows, 1);

    schema::configurations::dsl::configurations
        .filter(schema::configurations::dsl::bcm_pin.eq(device_pin))
        .first::<Configuration>(database_connection)
        .unwrap()
}

pub fn create_schedule(cron: String, action: String) -> Schedule {
    let database_connection: &mut SqliteConnection = &mut get_database_connection();
    let new_schedule: NewSchedule = NewSchedule {
        cron_string: cron.to_string(),
        action,
    };

    diesel::insert_into(schedules::table)
        .values(new_schedule)
        .execute(database_connection)
        .expect("Unable to create schedule");

    schedules::dsl::schedules
        .filter(schedules::dsl::cron_string.eq(cron))
        .first::<Schedule>(database_connection)
        .unwrap()
}

pub fn link_configuration_to_schedule(schedule_id: i32, configuration_id: i32) {
    let database_connection: &mut SqliteConnection = &mut get_database_connection();

    let new_schedule_configuration = NewScheduleConfiguration {
        schedule_id,
        configuration_id,
    };

    diesel::insert_into(schema::schedule_configurations::table)
        .values(new_schedule_configuration)
        .execute(database_connection)
        .expect("Unable to create schedule configuration");
}

pub fn get_configuration_by_sensor_name(sensor_name: String) -> Configuration {
    let database_connection: &mut SqliteConnection = &mut get_database_connection();
    configurations::table
        .filter(configurations::dsl::sensor_name.eq(sensor_name))
        .first::<Configuration>(database_connection)
        .expect("Error loading water detector config")
}

pub fn teardown_configuration() {
    execute_truncate(String::from("configurations"));
}

pub fn teardown_schedule() {
    execute_truncate(String::from("schedules"))
}

pub fn teardown_configuration_dependencies() {
    execute_truncate(String::from("configuration_dependencies"));
}

fn execute_truncate(table: String) {
    let database_connection: &mut SqliteConnection = &mut get_database_connection();

    let result = sql_query(format!("DELETE FROM {}", table));
    result.execute(database_connection).unwrap();
}

pub fn execute_test<T>(test: T) -> ()
where
    T: FnOnce() -> () + panic::UnwindSafe,
{
    teardown_configuration_dependencies();
    teardown_configuration();
    teardown_schedule();

    let result = panic::catch_unwind(|| test());

    teardown_configuration_dependencies();
    teardown_configuration();
    teardown_schedule();

    assert!(result.is_ok());
}
