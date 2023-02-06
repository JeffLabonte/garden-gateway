use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};
use gateway::models::{
    Configuration, NewConfiguration, NewSchedule, NewScheduleConfiguration, Schedule,
};
use gateway::schema::{self, schedules};

pub fn create_configuration(
    sensor_name: String,
    device_pin: i32,
    database_connection: &mut SqliteConnection,
) -> Configuration {
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

pub fn create_schedule(
    cron: String,
    action: String,
    database_connection: &mut SqliteConnection,
) -> Schedule {
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

pub fn link_configuration_to_schedule(
    schedule_id: i32,
    configuration_id: i32,
    database_connection: &mut SqliteConnection,
) {
    let new_schedule_configuration = NewScheduleConfiguration {
        schedule_id,
        configuration_id,
    };

    diesel::insert_into(schema::schedule_configurations::table)
        .values(new_schedule_configuration)
        .execute(database_connection)
        .expect("Unable to create schedule configuration");
}
