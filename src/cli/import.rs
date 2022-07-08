use std::fs::File;
use std::path::PathBuf;
use std::collections::HashSet;

use serde::Deserialize;

#[derive(Deserialize, Clone, Hash, PartialEq, Eq)]
struct ImportedSchedule {
    pub cron_string: String,
    pub action: String,
    pub configuration_id: u32,
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

fn validate_input(schedules: Vec<ImportedSchedule>) -> bool {
    is_input_unique(schedules)
}

pub fn import_schedule_from_json(file: PathBuf) -> bool {
    let imported_schedules = read_json_schedule(file);
    false
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    fn generate_imported_schedule(size: u32) -> Vec<ImportedSchedule> {
        let mut imported_schedules: Vec<ImportedSchedule> = Vec::new();
        for _ in 0..size {
            let imported_schedule = ImportedSchedule{
                cron_string: String::from("* * * * *"),
                action: String::from("turn_off"),
                configuration_id: 1,
            };
            imported_schedules.push(imported_schedule);
        }
        return imported_schedules;
    }
    
    #[test_case(true, true ; "When unique expect valid")]
    #[test_case(false, false ; "When not unique expect invalid")]
    fn import_schedules_must_be_unique(is_unique: bool, is_schedules_valid: bool) {
        let mut imported_schedules = generate_imported_schedule(2);

        if is_unique == true {
            imported_schedules[0].configuration_id = 2;
        }

        let result: bool = validate_input(imported_schedules);
        assert_eq!(result, is_schedules_valid);
    }
}
