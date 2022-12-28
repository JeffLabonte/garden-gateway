use std::collections::HashMap;

use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

use crate::{
    constants::{TURN_OFF_ACTION, TURN_ON_ACTION},
    database::helpers::{get_all_configurations, retrieve_schedules_from_config_id},
    devices::relay_power::RelayPowerBar,
    devices::Device,
    models::Configuration,
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

fn add_job_to_scheduler(scheduler: &JobScheduler, configuration: Configuration) -> Vec<Uuid> {
    // TODO Implement with new tables
    let schedules = retrieve_schedules_from_config_id(configuration.id);

    let mut job_ids = Vec::new();
    for schedule in schedules {
        let cron_schedule_str = schedule.cron_string.as_str();
        println!("This schedule will run at {}", cron_schedule_str);

        match schedule.action.as_str() {
            TURN_ON_ACTION => {
                println!("Adding turn_on for configuration {}", 0);
                let job = Job::new(cron_schedule_str, move |_, _| {
                    let pins = HashMap::from([("relay_power_pin", configuration.bcm_pin as u8)]);
                    // TODO Use new device
                    //let mut device = &RelayPowerBar::new(pins);
                    //device.turn_on();
                })
                .unwrap();
                job_ids.push(scheduler.add(job).unwrap());
            }
            TURN_OFF_ACTION => {
                println!("Adding turn_off for configuration {}", 0);
                let job = Job::new(cron_schedule_str, move |_, _| {
                    let pins = HashMap::from([("relay_power_pin", configuration.bcm_pin as u8)]);
                    // let mut device: &RelayPowerBar = &RelayPowerBar::new(pins);
                    // device.turn_off();
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
    let configurations: Vec<Configuration> = get_all_configurations();

    for config in configurations {
        for job_id in add_job_to_scheduler(&scheduler, config) {
            job_ids.push(job_id);
        }
    }

    job_ids
}
