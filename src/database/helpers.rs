use crate::models::Schedule;
use diesel::prelude::*;

pub fn retrieve_schedules_from_config_id(config_id: u32) -> Vec<Schedule> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::SqliteConnection;

    use crate::diesel::RunQueryDsl;
    use crate::models::{NewConfiguration, Configuration, NewScheduleConfiguration};
    use crate::schema::{configurations, schedule_configurations};
    use crate::{models::NewSchedule, models::Schedule, schema::schedules};

    fn create_base_data(database: &SqliteConnection) {
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

        let schedule_configuration =  NewScheduleConfiguration {
            schedule_id: last_inserted_schedule.id,
            configuration_id: last_inserted_config.id,
        };

        diesel::insert_or_ignore_into(schedule_configurations::table)
            .values(schedule_configuration)
            .execute(database)
            .expect("Unable to insert the new Schedule Configuration");
    }

    #[test]
    fn test_retrieve_schedules_from_config_id() {}
}
