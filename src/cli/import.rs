use std::fs::File;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
struct ImportedSchedule {
    pub cron_string: String,
    pub action: String,
    pub configuration_id: i32,
}

pub fn import_schedule_from_json(file: PathBuf) -> bool {
    let json_file = match File::open(file) {
        Ok(f) => f,
        Err(e) => panic!("Unable to read file: {}", e),
    };
    let schedules: Vec<ImportedSchedule> = match serde_json::from_reader(json_file) {
        Ok(v) => v,
        Err(e) => panic!("Unable to deserialize JSON file: {}", e),
    };
    false
}
