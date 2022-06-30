use crate::devices::{Devices, RelayPowerBar};
use crate::models::*;
use diesel::prelude::*;
use tokio_cron_scheduler::{Job, JobScheduler, JobToRun};

fn get_struct_from_configuration(configuration: Configuration) -> Box<dyn Devices> {
    Box::new(RelayPowerBar::new(configuration.bcm_pin as u8))
}

fn add_job_to_scheduler(
    database: &SqliteConnection,
    scheduler: &JobScheduler,
    configuration: Configuration,
) -> () {
    use crate::schema::schedules::dsl::{configuration_id, schedules};

    let config_id = configuration.id;
    let device = get_struct_from_configuration(configuration);

    let results = schedules
        .filter(configuration_id.eq(config_id))
        .load::<Schedule>(database)
        .expect("Error loading schedules");

    for schedule in results {
        match device.is_what_type().as_str() {
            "RelayPowerBar" => match schedule.action.as_str() {
                "turn_on" => (),
                "turn_off" => (),
                _ => (),
            },
            _ => (),
        }
    }
}

pub fn run(database: SqliteConnection) -> bool {
    use crate::schema::configurations::dsl::configurations;

    let configs = configurations
        .load::<Configuration>(&database)
        .expect("Error loading configurations");

    let mut thread_handlers: Vec<JoinHandle<()>> = Vec::new();
    for config in configs {
        let bcm_pin: u8 = config.bcm_pin as u8;

        let is_success = match config.sensor_name.as_str() {
            "relay_bar" => {
                let relay_power_bar = RelayPowerBar::new(bcm_pin);
                let handle = thread::spawn(move || {});
                thread_handlers.push(handle);
                true
            }
            _ => false,
        };

        if !is_success {
            return is_success;
        }
    }

    true
}
