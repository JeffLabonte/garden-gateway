use crate::models::Schedule;

pub fn retrieve_schedules_from_config_id(config_id: u32) -> Vec<Schedule> {
    vec![]
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{NewSchedule, NewScheduleConfiguration},
        schema::{schedule_configurations, schedules},
    };
    use diesel::SqliteConnection;

    fn setup(database: &SqliteConnection) {
        let configuration_id: i32 = 1;
        let default_schedule: NewSchedule = NewSchedule {
            action: "turn_off".to_string(),
            cron_string: "* * * * *".to_string(),
        };

        match diesel::insert_or_ignore_into(schedules::table)
            .values(&default_schedule)
            .execute(database)
        {
            Ok(schedule_id) => {
                let default_schedule_configurations: NewScheduleConfiguration =
                    NewScheduleConfiguration::from_schedule_and_configuration_id(
                        schedule_id,
                        configuration_id,
                    );
                let result = diesel::insert_into(schedule_configurations::table)
                    .values(default_schedule_configurations)
                    .execute(database);

                match result {
                    Ok(_) => (),
                    Err(e) => eprintln!("{}", e),
                };
            }
            Err(e) => {
                eprint!("{}", e);
            }
        }

        ()
    }

    #[test]
    fn test_retrieve_schedules_from_config_id() {}
}
