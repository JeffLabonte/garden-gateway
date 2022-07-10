use std::{str::FromStr, time::Duration};

use crate::devices::RelayPowerBar;
use crate::models::*;
use diesel::prelude::*;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

fn add_job_to_scheduler(
    database: &SqliteConnection,
    scheduler: &JobScheduler,
    configuration: Configuration,
) -> Vec<Uuid> {
    use crate::schema::schedules::dsl::{configuration_id, schedules};

    let config_id = configuration.id;
    let results = schedules
        .filter(configuration_id.eq(config_id))
        .load::<Schedule>(database)
        .expect("Error loading schedules");

    let mut job_ids = Vec::new();
    for sched in results {
        let cron_schedule = cron::Schedule::from_str(sched.cron_string.as_str()).unwrap();
        println!("This schedule will run at {}", cron_schedule);
        match sched.action.as_str() {
            "turn_on" => {
                println!("Adding turn_off for configuration {}", sched.configuration_id);
                let job = Job::new(cron_schedule, move |_, _| {
                    let mut device = RelayPowerBar::new(configuration.bcm_pin as u8);
                    device.turn_on()
                })
                .unwrap();
                job_ids.push(scheduler.add(job).unwrap());
            }
            "turn_off" => {
                println!("Adding turn_on for configuration {}", sched.configuration_id);
                let job = Job::new(cron_schedule, move |_, _| {
                    let mut device = RelayPowerBar::new(configuration.bcm_pin as u8);
                    device.turn_off()
                })
                .unwrap();
                job_ids.push(scheduler.add(job).unwrap());
            }
            _ => panic!("Action not implemented"),
        };
    }

    job_ids
}

#[tokio::main]
pub async fn run(database: SqliteConnection) -> bool {
    use crate::schema::configurations::dsl::configurations;

    let scheduler = JobScheduler::new();
    let configs = configurations
        .load::<Configuration>(&database)
        .expect("Error loading configurations");

    let scheduler = scheduler.unwrap();
    let mut job_ids: Vec<Uuid> = Vec::new();
    for config in configs {
        for job_id in add_job_to_scheduler(&database, &scheduler, config) {
            job_ids.push(job_id);
        }
    }
    
    loop {
        if job_ids.is_empty() {
            println!("No Jobs to run! Bye!");
            return true;
        }
        let now = chrono::Local::now();
        match scheduler.tick() {
            Ok(_) => std::thread::sleep(Duration::from_millis(1000)),
            Err(e) => {
                eprintln!("Something went wrong during runtime: {}", e);
                return false;
            }
        };
    }
}
