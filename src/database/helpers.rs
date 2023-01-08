use crate::models::Configuration;
use crate::schema::configurations;
use crate::DATABASE_CONNECTION;
use crate::{
    models::Schedule,
    schema::{schedule_configurations, schedules},
};
use diesel::prelude::*;

pub fn get_schedules_from_config_id(
    config_id: i32,
    database_connection: &mut SqliteConnection,
) -> Vec<Schedule> {
    schedules::table
        .inner_join(
            schedule_configurations::table
                .on(schedule_configurations::dsl::schedule_id.eq(schedules::dsl::id)),
        )
        .filter(schedule_configurations::dsl::configuration_id.eq(config_id))
        .select(schedules::all_columns)
        .load::<Schedule>(database_connection)
        .expect("Unable to load schedules")
}

pub fn get_configurations_by_schedule_id(
    schedule_id: i32,
    database_connection: &mut SqliteConnection,
) -> Vec<Configuration> {
    configurations::table
        .inner_join(
            schedule_configurations::table
                .on(schedule_configurations::dsl::configuration_id.eq(configurations::dsl::id)),
        )
        .filter(schedule_configurations::dsl::schedule_id.eq(schedule_id))
        .select(configurations::all_columns)
        .load::<Configuration>(database_connection)
        .expect("Error Loading Configurations")
}

pub fn get_all_configurations(database_connection: &mut SqliteConnection) -> Vec<Configuration> {
    use crate::schema::configurations::dsl::configurations;

    configurations
        .load::<Configuration>(database_connection)
        .expect("Error loading configurations")
}

pub fn get_all_schedules(database_connection: &mut SqliteConnection) -> Vec<Schedule> {
    use crate::schema::schedules::dsl::schedules;

    schedules
        .load::<Schedule>(database_connection)
        .expect("Unable to retrieve schedules")
}

pub fn get_database_connection() -> std::sync::MutexGuard<'static, SqliteConnection> {
    DATABASE_CONNECTION.lock().unwrap()
}
