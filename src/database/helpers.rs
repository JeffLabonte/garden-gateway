use crate::models::Configuration;
use crate::schema::configurations;
use crate::DATABASE_CONNECTION;
use crate::{
    models::{Schedule, ScheduleConfiguration},
    schema::schedule_configurations::dsl::configuration_id,
    schema::{schedule_configurations, schedules},
};
use diesel::prelude::*;

pub fn retrieve_schedules_from_config_id(config_id: i32) -> Vec<Schedule> {
    let database_connection: &mut SqliteConnection = &mut get_database_connection();
    let schedule_config_vec = schedule_configurations::table
        .filter(configuration_id.eq(config_id))
        .load::<ScheduleConfiguration>(database_connection)
        .expect("Error Loading Schedule Configurations");

    let schedules_ids = schedule_config_vec
        .iter()
        .map(|schedule_config| schedule_config.schedule_id)
        .collect::<Vec<i32>>();

    let scheds = schedules::table
        .filter(schedules::dsl::id.eq_any(schedules_ids))
        .load::<Schedule>(database_connection)
        .expect("Error Loading Schedules");

    scheds
}

pub fn get_configurations_by_schedule_id(
    schedule_id: i32,
    database_connection: &mut SqliteConnection,
) -> Vec<Configuration> {
    use crate::schema::configurations::dsl::id;

    let schedule_config_vec = schedule_configurations::table
        .filter(schedule_configurations::dsl::schedule_id.eq(schedule_id))
        .load::<ScheduleConfiguration>(database_connection)
        .expect("Error Loading Schedule Configurations");

    println!("Schedule Configurations: {:?}", schedule_config_vec);

    let config_ids = schedule_config_vec
        .iter()
        .map(|schedule_config| schedule_config.configuration_id)
        .collect::<Vec<i32>>();

    println!("Config ids: {:?}", config_ids);
    let configs = configurations::table
        .filter(id.eq_any(config_ids))
        .load::<Configuration>(database_connection)
        .expect("Error Loading Configurations");

    configs
}

pub fn get_database_connection() -> std::sync::MutexGuard<'static, SqliteConnection> {
    DATABASE_CONNECTION.lock().unwrap()
}

pub fn get_all_configurations(database_connection: &mut SqliteConnection) -> Vec<Configuration> {
    use crate::schema::configurations::dsl::configurations;

    configurations
        .load::<Configuration>(database_connection)
        .expect("Error loading configurations")
}
