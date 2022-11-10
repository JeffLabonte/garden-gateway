use crate::DATABASE_CONNECTION;
use crate::{
    models::{Schedule, ScheduleConfiguration},
    schema::schedule_configurations::dsl::configuration_id,
    schema::{schedule_configurations, schedules},
};
use diesel::prelude::*;

pub fn retrieve_schedules_from_config_id(config_id: i32) -> Vec<Schedule> {
    let database_connection: &mut SqliteConnection = &mut DATABASE_CONNECTION.lock().unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::SqliteConnection;

    use crate::diesel::RunQueryDsl;
    use crate::models::{Configuration, NewConfiguration, NewScheduleConfiguration};
    use crate::schema::configurations;
    use crate::{models::NewSchedule, models::Schedule, schema::schedules};

    fn create_base_data() {
        let database: &mut SqliteConnection = &mut DATABASE_CONNECTION.lock().unwrap();
        let default_schedule = NewSchedule {
            action: "turn_off".to_string(),
            cron_string: "* * * * *".to_string(),
        };
        let default_configuration = NewConfiguration {
            bcm_pin: 1,
            sensor_name: "Test Sensor".to_string(),
        };

        diesel::insert_or_ignore_into(schedules::table)
            .values(&default_schedule)
            .execute(database)
            .expect("Unable to insert the new Schedule");

        diesel::insert_or_ignore_into(configurations::table)
            .values(&default_configuration)
            .execute(database)
            .expect("Unable to insert the new Configuration");

        let last_inserted_schedule = schedules::dsl::schedules
            .order_by(schedules::dsl::id.desc())
            .first::<Schedule>(database)
            .expect("Unable to retrieve the latest Schedule");

        let last_inserted_config = configurations::dsl::configurations
            .order_by(configurations::dsl::id.desc())
            .first::<Configuration>(database)
            .expect("Unable to retrieve the lastest Configuration");

        let schedule_configuration = NewScheduleConfiguration {
            schedule_id: last_inserted_schedule.id,
            configuration_id: last_inserted_config.id,
        };

        diesel::insert_or_ignore_into(schedule_configurations::table)
            .values(schedule_configuration)
            .execute(database)
            .expect("Unable to insert the new Schedule Configuration");
    }

    fn get_last_inserted_config() -> Configuration {
        let database: &mut SqliteConnection = &mut DATABASE_CONNECTION.lock().unwrap();
        configurations::dsl::configurations
            .order_by(configurations::dsl::id.desc())
            .first::<Configuration>(database)
            .expect("Unable to retrieve the lastest Configuration")
    }

    #[test]
    fn test_retrieve_schedules_from_config_id() {
        create_base_data();

        let last_inserted_config = get_last_inserted_config();
        let schedules = retrieve_schedules_from_config_id(last_inserted_config.id);

        assert_eq!(schedules.len(), 1);
    }
}
