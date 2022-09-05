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
    use crate::{models::NewSchedule, models::Schedule, schema::schedules};

    fn create_base_data(database: &SqliteConnection) {
        let configuraation_id: i32 = 1;
        let default_schedule = NewSchedule {
            action: "turn_off".to_string(),
            cron_string: "* * * * *".to_string(),
        };
        let result = diesel::insert_or_ignore_into(schedules::table)
            .values(&default_schedule)
            .execute(database);

        let schedule_result = schedules::dsl::schedules
            .order_by(schedules::dsl::id.desc())
            .first::<Schedule>(database)
            .expect("Unable to retrieve the latest Schedule");

    }

    #[test]
    fn test_retrieve_schedules_from_config_id() {}
}
