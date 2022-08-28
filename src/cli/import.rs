use std::str::FromStr;

use crate::models::*;
use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;

use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct ImportedSchedule {
    pub cron_string: String,
    pub action: String,
    pub configurations: Vec<i32>,
}

fn read_json_schedule(file: PathBuf) -> Vec<ImportedSchedule> {
    let json_file = match File::open(file) {
        Ok(f) => f,
        Err(e) => panic!("Unable to read file: {}", e),
    };
    match serde_json::from_reader(json_file) {
        Ok(v) => v,
        Err(e) => panic!("Unable to deserialize JSON file: {}", e),
    }
}

fn is_input_unique(schedules: &Vec<ImportedSchedule>) -> bool {
    let original_schedules = schedules.clone();
    let unique_schedules: HashSet<ImportedSchedule> =
        original_schedules.clone().into_iter().collect();

    original_schedules.len() == unique_schedules.len()
}

fn is_unique_with_db(
    database: &SqliteConnection,
    imported_schedules: &Vec<ImportedSchedule>,
) -> bool {
    use crate::schema::schedules::dsl::{action, cron_string, schedules};

    for imported_schedule in imported_schedules {
        let schedule = imported_schedule.clone();
        let db_schedules = schedules
            .filter(cron_string.eq(schedule.cron_string))
            .filter(action.eq(schedule.action))
            .load::<Schedule>(database)
            .expect("Error Loading Configurations");

        if db_schedules.is_empty() {
            return true;
        }
    }

    false
}

fn is_input_valid(database: &SqliteConnection, imported_schedules: &Vec<ImportedSchedule>) -> bool {
    use crate::schema::configurations::dsl::{configurations, id};
    use diesel::dsl::exists;

    for imported_schedule in imported_schedules {
        let schedule_clone = imported_schedule.clone();
        if schedule_clone.action.is_empty() {
            return false;
        }
        for config_id in schedule_clone.configurations {
            let has_config: Result<bool, diesel::result::Error> =
                diesel::select(exists(configurations.filter(id.eq(config_id))))
                    .get_result(database);

            if !has_config.unwrap() {
                return false;
            }
        }
    }

    true
}

fn validate_input(
    database: &SqliteConnection,
    schedules: Vec<ImportedSchedule>,
) -> Result<(), &str> {
    // TODO Use a list of function to loop on
    if !is_input_unique(&schedules) {
        return Err("The Schedules you are trying to import are not unique");
    }

    if !is_unique_with_db(database, &schedules) {
        return Err("Your data that you are trying to import isn't unique with the database");
    }

    if !is_input_valid(database, &schedules) {
        return Err("Invalid data in the configurations");
    }

    Ok(())
}

fn import_schedule(
    database: &SqliteConnection,
    imported_schedules: &Vec<ImportedSchedule>,
) -> bool {
    use crate::schema::{schedule_configurations, schedules};

    for imported_schedule in imported_schedules {
        let schedule_clone: ImportedSchedule = imported_schedule.clone();
        let configurations = schedule_clone.configurations.clone();

        match cron::Schedule::from_str(schedule_clone.cron_string.as_str()) {
            Err(e) => {
                eprintln!("Something went wrong during import: {}", e);
                return false;
            }
            Ok(_) => {
                let new_schedule = NewSchedule::from_imported_schedule(schedule_clone);
                diesel::insert_into(schedules::table)
                    .values(&new_schedule)
                    .execute(database)
                    .expect("Unable to create new schedule");

                let last_inserted_schedule = schedules::dsl::schedules
                    .select(schedules::dsl::id)
                    .filter(schedules::dsl::cron_string.eq(new_schedule.cron_string))
                    .filter(schedules::dsl::action.eq(new_schedule.action))
                    .order_by(schedules::dsl::id.desc())
                    .first(database);

                match last_inserted_schedule {
                    Ok(inserted_schedule) => {
                        for configuration_id in configurations {
                            let new_schedule_configuration =
                                NewScheduleConfiguration::from_schedule_and_configuration_id(
                                    inserted_schedule,
                                    configuration_id,
                                );
                            match diesel::insert_into(schedule_configurations::table)
                                .values(&new_schedule_configuration)
                                .execute(database)
                            {
                                Ok(_) => continue,
                                Err(e) => {
                                    eprintln!("Unable to insert NewScheduleConfiguration: {}", e);
                                    return false;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Unexpected error with database: {}", e);
                        return false;
                    }
                }
            }
        }
    }

    println!("Imported Schedule Successfully");
    true
}

pub fn import_schedule_from_json(database: SqliteConnection, file: PathBuf) -> bool {
    let imported_schedules = read_json_schedule(file);
    match validate_input(&database, imported_schedules.clone()) {
        Ok(_) => import_schedule(&database, &imported_schedules),
        Err(err) => {
            println!("{}", err);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::establish_connection;
    use crate::models::NewSchedule;
    use crate::schema::{schedule_configurations, schedules};
    use diesel::result::Error;
    use test_case::test_case;

    fn setup(database: &SqliteConnection) {
        let configuration_id: i32 = 1;
        let default_schedule = NewSchedule {
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
                        schedule_id as i32,
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
                eprintln!("{}", e);
            }
        }

        ()
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

    #[test_case(true, true; "When unique expect valid")]
    #[test_case(false, false ; "When not unique expect invalid")]
    fn import_schedules_must_be_unique(is_unique: bool, is_schedules_valid: bool) {
        let mut imported_schedules = generate_imported_schedule(2);
        if is_unique {
            imported_schedules[0].configurations = vec![2];
        }

        let result: bool = is_input_unique(&imported_schedules);
        assert_eq!(result, is_schedules_valid);
    }

    #[test]
    fn imported_schedules_not_unique_with_db_should_be_invalid() {
        let database = &establish_connection();

        database.test_transaction::<_, Error, _>(|| {
            setup(database);
            let mut imported_schedules = generate_imported_schedule(1);

            let result: bool = is_unique_with_db(database, &imported_schedules);
            assert!(!result);

            imported_schedules[0].action = "turn_on".to_string();

            let result: bool = is_unique_with_db(database, &imported_schedules);
            assert!(result);

            Ok(())
        });
    }

    #[test]
    fn imported_schedules_not_valid_cron_should_fail_import() {
        let database = &establish_connection();

        database.test_transaction::<_, Error, _>(|| {
            setup(database);

            let mut imported_schedules = generate_imported_schedule(1);
            imported_schedules[0].cron_string = "faillure".to_string();

            let result: bool = import_schedule(database, &imported_schedules);

            assert!(!result);

            Ok(())
        });
    }
}
