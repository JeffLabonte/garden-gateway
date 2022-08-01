use crate::devices::{relay_power::RelayPowerBar, Device};
use crate::models::*;
use diesel::prelude::*;
use std::collections::HashMap;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

const WATER_PUMP_SENSOR_NAME: &str = "";
const RELAY_POWER_SENSOR_NAME: &str = "relay_power";

const TURN_ON_ACTION: &str = "turn_on";
const TURN_OFF_ACTION: &str = "turn_off";

fn add_job_to_scheduler(
    database: &SqliteConnection,
    scheduler: &JobScheduler,
    configuration: Configuration,
) -> Vec<Uuid> {
    use crate::schema::configurations::dsl::configurations;
    use crate::schema::schedules::dsl::{configuration_id, schedules};

    let config_id = configuration.id;
    let results = schedules
        .filter(configuration_id.eq(config_id))
        .load::<Schedule>(database)
        .expect("Error loading schedules");

    let mut job_ids = Vec::new();
    for sched in results {
        let cron_schedule_str = sched.cron_string.as_str();
        println!("This schedule will run at {}", cron_schedule_str);
        let configuration_object: Configuration = configurations
            .find(sched.configuration_id)
            .first(database)
            .expect("Error while retrieving configuration");

        match sched.action.as_str() {
            TURN_ON_ACTION => {
                println!(
                    "Adding turn_on for configuration {}",
                    sched.configuration_id
                );
                let job = Job::new(cron_schedule_str, move |_, _| {
                    let pins = HashMap::from([
                        ("relay_power_pin", configuration.bcm_pin as u8),
                    ]);
                    let mut device = RelayPowerBar::new(pins);
                    device.turn_on();
                })
                .unwrap();
                job_ids.push(scheduler.add(job).unwrap());
            }
            TURN_OFF_ACTION => {
                println!(
                    "Adding turn_off for configuration {}",
                    sched.configuration_id
                );
                let job = Job::new(cron_schedule_str, move |_, _| {
                    let pins = HashMap::from([
                        ("relay_power_pin", configuration.bcm_pin as u8),
                    ]);
                    let mut device = RelayPowerBar::new(pins);
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

#[tokio::main]
pub async fn run(database: &SqliteConnection) -> bool {
    use crate::schema::configurations::dsl::configurations;

    let scheduler = JobScheduler::new();
    let configs = configurations
        .load::<Configuration>(database)
        .expect("Error loading configurations");

    let mut scheduler = scheduler.unwrap();
    let mut job_ids: Vec<Uuid> = Vec::new();
    for config in configs {
        for job_id in add_job_to_scheduler(database, &scheduler, config) {
            job_ids.push(job_id);
        }
    }

    let mut run_again: bool = false;
    loop {
        if job_ids.is_empty() {
            println!("No Jobs to run! Bye!");
            break;
        }

        match scheduler.tick() {
            Ok(_) => match scheduler.time_till_next_job() {
                Ok(v) => match v {
                    Some(_) => {
                        std::thread::sleep(Duration::from_millis(500));
                    }
                    None => {
                        run_again = true;
                        break;
                    }
                },
                Err(e) => {
                    println!("Couldn't retrieve the time till next job: {}", e);
                    run_again = true;
                    break;
                }
            },
            Err(e) => {
                println!("Something went wrong during runtime: {}", e);
                run_again = true;
                break;
            }
        };
    }

    run_again
}
