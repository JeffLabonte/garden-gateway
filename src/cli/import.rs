use std::fs::File;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
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

pub fn import_schedule_from_json(file: PathBuf) -> bool {
    let imported_schedules = read_json_schedule(file);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn import_schedules_must_be_unique() {}
}
