use crate::models::Schedule;

pub fn retrieve_schedules_from_config_id(config_id: u32) -> Vec<Schedule> {
    vec![]
}

#[cfg(test)]
mod tests {
    use diesel::SqliteConnection;

    use crate::{models::NewSchedule, schema::schedules};

    fn create_base_data(database: &SqliteConnection) {
        let configuraation_id: i32 = 1;
        let default_schedule = NewSchedule {
            action: "turn_off".to_string(),
            cron_string: "* * * * *".to_string(),
        };
        let result = diesel::insert_or_ignore_into(schedules::table)
            .values(default_schedule)
            .execute(database)
            .expect("Unable to insert Schedule");

        
    }
    #[test]
    fn test_retrieve_schedules_from_config_id() {}
}
