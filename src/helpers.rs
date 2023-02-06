use std::collections::HashMap;

use diesel::SqliteConnection;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

use crate::{
    constants::{
        RELAY_POWER_PIN_KEY, RELAY_POWER_SENSOR_NAME, TURN_OFF_ACTION, TURN_ON_ACTION,
        WATER_DETECTOR_PIN_KEY, WATER_DETECTOR_SENSOR_NAME, WATER_PUMP_PIN_KEY,
        WATER_PUMP_SENSOR_NAME,
    },
    database::helpers::{
        get_all_schedules, get_configuration_dependencies_from_config_id,
        get_configurations_by_schedule_id, get_database_connection,
    },
    devices::build_device,
    models::{Configuration, Schedule},
};

const DATETIME_FORMAT: &str = "%b %-d, %-I:%M:%s";

pub fn println_now(action: &str, board: &str) {
    let now = chrono::Local::now();
    let now_utc = chrono::Utc::now();
    println!(
        "{} - Running: {} pin to {}",
        now.format(DATETIME_FORMAT),
        action,
        board,
    );
    print!("Time in UTC: {}", now_utc.format(DATETIME_FORMAT));
}

fn get_hashmap_key_from_sensor_name(sensor_name: String) -> String {
    match sensor_name.as_str() {
        WATER_PUMP_SENSOR_NAME => WATER_PUMP_PIN_KEY.to_string(),
        WATER_DETECTOR_SENSOR_NAME => WATER_DETECTOR_PIN_KEY.to_string(),
        RELAY_POWER_SENSOR_NAME => RELAY_POWER_PIN_KEY.to_string(),
        _ => panic!("Sensor is not supported"),
    }
}

fn add_job_to_scheduler(scheduler: &JobScheduler, schedule: Schedule) -> Vec<Uuid> {
    // TODO Implement with new tables
    let database_connection: &mut SqliteConnection = &mut get_database_connection();

    let configurations = get_configurations_by_schedule_id(schedule.id, database_connection);
    let mut job_ids = Vec::new();

    for configuration in configurations {
        let cron_schedule_str = schedule.cron_string.clone();
        let sensor_name = configuration.sensor_name.clone();

        println!("This schedule will run at {}", cron_schedule_str);

        let mut device = build_device(&sensor_name);
        match schedule.action.as_str() {
            TURN_ON_ACTION => {
                println!("Adding turn_on for configuration {}", 0);
                let job = Job::new(cron_schedule_str.as_str(), move |_, _| {
                    device.turn_on();
                })
                .unwrap();
                job_ids.push(scheduler.add(job).unwrap());
            }
            TURN_OFF_ACTION => {
                println!("Adding turn_off for configuration {}", 0);
                let job = Job::new(cron_schedule_str.as_str(), move |_, _| {
                    device.turn_off();
                })
                .unwrap();
                job_ids.push(scheduler.add(job).unwrap());
            }
            _ => panic!("Action not implemented"),
        };
    }

    job_ids
}

pub fn populate_job_ids(scheduler: &JobScheduler) -> Vec<Uuid> {
    let mut job_ids: Vec<Uuid> = Vec::new();
    let schedules = get_all_schedules(&mut get_database_connection());

    for schedule in schedules {
        for job_id in add_job_to_scheduler(&scheduler, schedule) {
            job_ids.push(job_id);
        }
    }

    job_ids
}
