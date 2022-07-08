use std::fs::File;
use std::path::PathBuf;
use std::collections::HashSet;
use crate::models::*;

use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Clone, Hash, PartialEq, Eq)]
struct ImportedSchedule {
    pub cron_string: String,
    pub action: String,
    pub configuration_id: i32,
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

fn is_input_unique(schedules: Vec<ImportedSchedule>) -> bool {
    let original_schedules = schedules.clone();
    let unique_schedules: HashSet<ImportedSchedule> = schedules.into_iter().collect();
    
    return original_schedules.len() == unique_schedules.len();
}

fn is_unique_with_db(database: SqliteConnection, imported_schedules: Vec<ImportedSchedule>) -> bool {
    use crate::schema::schedules::dsl::{schedules, cron_string, configuration_id, action};

    for imported_schedule in imported_schedules {
        let db_schedules = schedules
            .filter(
                cron_string.eq(imported_schedule.cron_string),
            )
            .filter(
                configuration_id.eq(imported_schedule.configuration_id),
            )
            .filter(
                action.eq(imported_schedule.action),
            )
            .load::<Schedule>(&database)
            .expect("Error Loading Configurations");
        
        if db_schedules.is_empty() {
            return true;
        }
    }

    false
}

fn validate_input(database: SqliteConnection, schedules: Vec<ImportedSchedule>) -> bool {
    if ! is_input_unique(schedules.clone()) {
        return false;
    }
    
    if ! is_unique_with_db(database, schedules.clone()) {
        return false;
    }

    true
}

pub fn import_schedule_from_json(database: SqliteConnection, file: PathBuf) -> bool {
    let imported_schedules = read_json_schedule(file);
    let is_valid = validate_input(database, imported_schedules);

    if is_valid == false {
        return is_valid;
    }

    false
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;
    use crate::database::establish_connection;
    use crate::schema::schedules;
    use crate::models::NewSchedule;
    
    fn setup() -> () {
        let database = establish_connection();

        let default_schedule = NewSchedule {
            action: "turn_off".to_string(),
            cron_string: "* * * * *".to_string(),
            configuration_id: 1,
        };

        diesel::insert_into(schedules::table)
            .values(&default_schedule)
            .execute(&database)
            .expect("Error Saving Schedule");
    }
    
    fn teardown() -> () {
        use crate::schema::schedules::dsl::schedules;

        let database = establish_connection();

        diesel::delete(schedules)
            .execute(&database)
            .expect("Error Deleting schedules");
    }

    fn run_test<T>(test: T) -> () 
        where T: FnOnce() -> () {
        setup();

        test();

        teardown();
    }

    fn generate_default_imported_schedule() -> ImportedSchedule {
        ImportedSchedule {
            cron_string: String::from("* * * * *"),
            action: String::from("turn_off"),
            configuration_id: 1,
        }
    }

    fn generate_imported_schedule(size: u32) -> Vec<ImportedSchedule> {
        let mut imported_schedules: Vec<ImportedSchedule> = Vec::new();
        for _ in 0..size {
            let imported_schedule = generate_default_imported_schedule();
            imported_schedules.push(imported_schedule);
        }
        return imported_schedules;
    }
    
    #[test_case(true, true; "When unique expect valid")]
    #[test_case(false, false ; "When not unique expect invalid")]
    fn import_schedules_must_be_unique(is_unique: bool, is_schedules_valid: bool) {
        let mut imported_schedules = generate_imported_schedule(2);

        if is_unique == true {
            imported_schedules[0].configuration_id = 2;
        }

        let database = establish_connection();

        let result: bool = validate_input(database, imported_schedules);
        assert_eq!(result, is_schedules_valid);
    }
    
    #[test]
    fn imported_schedules_not_unique_with_db_should_be_invalid() {
        run_test(|| {
            let mut imported_schedules = generate_imported_schedule(1);

            let result: bool = is_unique_with_db(establish_connection(), imported_schedules.clone());
            assert_eq!(result, false);

            imported_schedules[0].action = "turn_on".to_string();
            
            let result: bool = is_unique_with_db(establish_connection(), imported_schedules.clone());
            assert_eq!(result, true);

        });
    }
}
