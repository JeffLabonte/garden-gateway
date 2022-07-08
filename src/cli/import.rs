use std::fs::File;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
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

fn validate_input(schedules: Vec<ImportedSchedule>) -> std::result::Result<bool, &'static str> {

    for schedule in schedules {
        let schedules_copy: Vec<ImportedSchedule> = Vec::new();
        schedules_copy = schedules.clone();
    }
    Ok(false)
}

pub fn import_schedule_from_json(file: PathBuf) -> bool {
    let imported_schedules = read_json_schedule(file);
    false
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn import_schedules_must_be_unique() {
        let imported_schedules = generate_imported_schedule(2);

        validate_input(imported_schedules);
    }
}
