use crate::cli::import::ImportedSchedule;
use crate::schema::{configurations, schedules};

#[derive(Queryable)]
pub struct Configuration {
    pub id: i32,
    pub sensor_name: String,
    pub bcm_pin: i32,
}

#[derive(Insertable)]
#[table_name = "configurations"]
pub struct NewConfiguration {
    pub sensor_name: String,
    pub bcm_pin: i32,
}

#[derive(Queryable)]
pub struct Schedule {
    pub id: i32,
    pub action: String,
    pub cron_string: String,
    pub configuration_id: i32,
}

#[derive(Insertable)]
#[table_name = "schedules"]
pub struct NewSchedule {
    pub action: String,
    pub cron_string: String,
    pub configuration_id: i32,
}

impl NewSchedule {
    pub fn from_imported_schedule(imported_schedule: ImportedSchedule) -> Self {
        Self {
            action: imported_schedule.action,
            cron_string: imported_schedule.cron_string,
            configuration_id: imported_schedule.configuration_id,
        }
    }
}
