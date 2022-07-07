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
    let json_file = File::open(file).expect("No file there");
    let schedules: Vec<ImportedSchedule> = serde_json::from_reader(json_file).expect("It failed");
    false
}
