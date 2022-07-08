use std::str::FromStr;

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

fn is_input_unique(schedules:&Vec<ImportedSchedule>) -> bool {
    let original_schedules = schedules.clone();
    let unique_schedules: HashSet<ImportedSchedule> = original_schedules.clone().into_iter().collect();
    
    return original_schedules.len() == unique_schedules.len();
}

fn is_unique_with_db(database: &SqliteConnection, imported_schedules: &Vec<ImportedSchedule>) -> bool {
    use crate::schema::schedules::dsl::{schedules, cron_string, configuration_id, action};

    for imported_schedule in imported_schedules {
        let schedule = imported_schedule.clone();
        let db_schedules = schedules
            .filter(cron_string.eq(schedule.cron_string))
            .filter(configuration_id.eq(schedule.configuration_id))
            .filter(action.eq(schedule.action))
            .load::<Schedule>(database)
            .expect("Error Loading Configurations");
        
        if db_schedules.is_empty() {
            return true;
        }
    }

    false
}

fn validate_input(database: &SqliteConnection, schedules: Vec<ImportedSchedule>) -> bool {
    if ! is_input_unique(&schedules) {
        return false;
    }
    
    if ! is_unique_with_db(database, &schedules) {
        return false;
    }

    true
}

fn import_schedule(database: &SqliteConnection, imported_schedules: &Vec<ImportedSchedule>) -> bool {
    for imported_schedule in imported_schedules {
        let schedule_clone: ImportedSchedule = imported_schedule.clone();
        match cron::Schedule::from_str(schedule_clone.cron_string.as_str()) {
            Err(e) => {
                eprintln!("Something went wrong during import: {}", e);
                return false;
            },
            Ok(_) => {
                println!("Yes");
            },

        }
    }

    true

}

pub fn import_schedule_from_json(database: SqliteConnection, file: PathBuf) -> bool {
    let imported_schedules = read_json_schedule(file);
    let is_valid = validate_input(&database, imported_schedules.clone());

    if is_valid == false {
        return is_valid;
    }
    
    import_schedule(&database, &imported_schedules)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;
    use crate::database::establish_connection;
    use crate::schema::schedules;
    use crate::models::NewSchedule;
    
    fn setup(database: &SqliteConnection) -> () {
        let default_schedule = NewSchedule {
            action: "turn_off".to_string(),
            cron_string: "* * * * *".to_string(),
            configuration_id: 1,
        };

        match diesel::insert_into(schedules::table)
            .values(&default_schedule)
            .execute(database){
            Ok(_) => (),
            Err(_) => (),
        }
    }
    
    fn teardown(database: &SqliteConnection) -> () {
        use crate::schema::schedules::dsl::schedules;

        match diesel::delete(schedules)
            .execute(database)
        {
            Ok(_) => (),
            Err(_) => (),
        }
    }

    fn run_test<T>(test: T) -> () 
        where T: FnOnce(&SqliteConnection) -> () {
        let database = establish_connection();

        setup(&database);

        test(&database);

        teardown(&database);
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

        let result: bool = validate_input(&database, imported_schedules);
        assert_eq!(result, is_schedules_valid);
    }
    
    #[test]
    fn imported_schedules_not_unique_with_db_should_be_invalid() {
        run_test(|database: &SqliteConnection| {
            let mut imported_schedules = generate_imported_schedule(1);

            let result: bool = is_unique_with_db(database, &imported_schedules);
            assert_eq!(result, false);

            imported_schedules[0].action = "turn_on".to_string();
            
            let result: bool = is_unique_with_db(database, &imported_schedules);
            assert_eq!(result, true);

        });
    }

    #[test]
    fn imported_schedules_not_valid_cron_should_fail_import() {
        run_test(|database: &SqliteConnection| {
            let mut imported_schedules = generate_imported_schedule(1);
            imported_schedules[0].cron_string = "faillure".to_string();
    
            let result: bool = import_schedule(database, &imported_schedules);

            assert_eq!(result, false);
        });
    }
}
