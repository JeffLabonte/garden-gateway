use std::collections::HashSet;

use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::database::helpers::get_database_connection;
use crate::exception::constants::UNIQUE_CONSTRAINT_DB_EXCEPTION_MESSAGE;
use crate::models::Schedule;

use super::import::ImportedSchedule;

pub fn is_input_valid(imported_schedules: &[ImportedSchedule]) -> Result<(), String> {
    use crate::schema::configurations::dsl::{configurations, id};
    use diesel::dsl::exists;
    let database_connection: &mut SqliteConnection = &mut get_database_connection();

    for imported_schedule in imported_schedules {
        let schedule_clone = imported_schedule.clone();
        if schedule_clone.action.is_empty() {
            return Err(String::from("Hey"));
        }
        for config_id in schedule_clone.configurations {
            let has_config: Result<bool, diesel::result::Error> =
                diesel::select(exists(configurations.filter(id.eq(config_id))))
                    .get_result(database_connection);

            if !has_config.unwrap() {
                return Err(String::from("Oh"));
            }
        }
    }

    Ok(())
}

pub fn is_unique_with_db(imported_schedules: &[ImportedSchedule]) -> Result<(), String> {
    use crate::schema::schedules::dsl::{action, cron_string, schedules};
    let database_connection: &mut SqliteConnection = &mut get_database_connection();

    for imported_schedule in imported_schedules {
        let schedule = imported_schedule.clone();
        let db_schedules = schedules
            .filter(cron_string.eq(schedule.cron_string))
            .filter(action.eq(schedule.action))
            .load::<Schedule>(database_connection)
            .expect("Error Loading Configurations");

        if db_schedules.is_empty() {
            return Ok(());
        }
    }

    Err(UNIQUE_CONSTRAINT_DB_EXCEPTION_MESSAGE.to_string())
}

pub fn is_input_unique(schedules: &[ImportedSchedule]) -> Result<(), String> {
    let original_schedules = schedules.to_owned();
    let unique_schedules: HashSet<ImportedSchedule> =
        original_schedules.clone().into_iter().collect();

    match original_schedules.len() == unique_schedules.len() {
        true => Ok(()),
        false => Err(String::from("THis")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::helpers::get_database_connection;
    use crate::models::{NewSchedule, NewScheduleConfiguration};
    use crate::schema::{schedule_configurations, schedules};
    use diesel::sql_query;
    use test_case::test_case;

    fn setup() {
        let database_connection: &mut SqliteConnection = &mut get_database_connection();

        let configuration_id: i32 = 1;
        let default_schedule = NewSchedule {
            action: "turn_off".to_string(),
            cron_string: "* * * * *".to_string(),
        };
        match diesel::insert_or_ignore_into(schedules::table)
            .values(&default_schedule)
            .execute(database_connection)
        {
            Ok(schedule_id) => {
                // TODO This isn't the ID in there but the number of row inserted
                let default_schedule_configurations: NewScheduleConfiguration =
                    NewScheduleConfiguration::from_schedule_and_configuration_id(
                        schedule_id as i32,
                        configuration_id,
                    );
                let result = diesel::insert_into(schedule_configurations::table)
                    .values(default_schedule_configurations)
                    .execute(database_connection);

                match result {
                    Ok(_) => (),
                    Err(e) => eprintln!("{e}"),
                };
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }

        ()
    }

    fn teardown() {
        let database_connection: &mut SqliteConnection = &mut get_database_connection();
        match sql_query("DELETE FROM schedules").execute(database_connection) {
            Ok(_) => (),
            Err(error) => panic!("{error}"),
        };
    }

    fn generate_default_imported_schedule() -> ImportedSchedule {
        ImportedSchedule {
            cron_string: String::from("* * * * *"),
            action: String::from("turn_off"),
            configurations: vec![1],
        }
    }

    fn generate_imported_schedule(size: u32) -> Vec<ImportedSchedule> {
        let mut imported_schedules: Vec<ImportedSchedule> = Vec::new();
        for _ in 0..size {
            let imported_schedule = generate_default_imported_schedule();
            imported_schedules.push(imported_schedule);
        }

        imported_schedules
    }

    #[test_case(true, Ok(()); "When unique expect valid")]
    #[test_case(false, Err(String::from("Hey")) ; "When not unique expect invalid")]
    fn import_schedules_must_be_unique(
        is_schedule_unique: bool,
        expected_result: Result<(), String>,
    ) {
        let mut imported_schedules = generate_imported_schedule(2);
        if is_schedule_unique {
            imported_schedules[0].configurations = vec![2];
        }

        let result: Result<(), String> = is_input_unique(&imported_schedules);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn imported_schedules_not_unique_with_db_should_be_invalid() {
        setup();
        let mut imported_schedules = generate_imported_schedule(1);

        let result: Result<(), String> = is_unique_with_db(&imported_schedules);
        assert_eq!(
            result,
            Err(UNIQUE_CONSTRAINT_DB_EXCEPTION_MESSAGE.to_string())
        );

        imported_schedules[0].action = "turn_on".to_string();

        let result: Result<(), String> = is_unique_with_db(&imported_schedules);
        assert_eq!(result, Ok(()));
        teardown();
    }

    //#[test]
    //fn imported_schedules_not_valid_cron_should_fail_import() {
    //    setup();

    //    let mut imported_schedules = generate_imported_schedule(1);
    //    imported_schedules[0].cron_string = "faillure".to_string();

    //    let result: Result<(), String> = import_schedule(&imported_schedules);

    //    assert_eq!(result, Err(String::from("Oh")));
    //    teardown();
    //}
}
