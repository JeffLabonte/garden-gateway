use crate::cli::import::ImportedSchedule;
use crate::schema::{configurations, schedule_configurations, schedules};

#[derive(Queryable, Debug)]
pub struct Configuration {
    pub id: i32,
    pub sensor_name: String,
    pub bcm_pin: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = configurations)]
pub struct NewConfiguration {
    pub sensor_name: String,
    pub bcm_pin: i32,
}

#[derive(Queryable, Debug)]
pub struct Schedule {
    pub id: i32,
    pub action: String,
    pub cron_string: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = schedules)]
pub struct NewSchedule {
    pub action: String,
    pub cron_string: String,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Schedule, foreign_key = schedule_id))]
#[diesel(belongs_to(Configuration, foreign_key = configuration_id))]
pub struct ScheduleConfiguration {
    pub id: i32,
    pub schedule_id: i32,
    pub configuration_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = schedule_configurations)]
pub struct NewScheduleConfiguration {
    pub schedule_id: i32,
    pub configuration_id: i32,
}

impl NewSchedule {
    pub fn from_imported_schedule(imported_schedule: ImportedSchedule) -> Self {
        Self {
            action: imported_schedule.action,
            cron_string: imported_schedule.cron_string,
        }
    }
}

impl NewScheduleConfiguration {
    pub fn from_schedule_and_configuration_id(schedule_id: i32, configuration_id: i32) -> Self {
        Self {
            schedule_id,
            configuration_id,
        }
    }
}
