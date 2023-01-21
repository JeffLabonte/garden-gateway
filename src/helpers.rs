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
        get_all_configurations, get_all_schedules, get_configuration_dependencies_from_config_id,
        get_configurations_by_schedule_id, get_database_connection, get_schedules_from_config_id,
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

fn get_device_pins_from_configuration<'a>(
    configuration: Configuration,
    already_configured_devices: &mut Vec<Configuration>,
    database_connection: &mut SqliteConnection,
) -> HashMap<&'a str, u8> {
    let mut dependencies_configurations =
        get_configuration_dependencies_from_config_id(configuration.id, database_connection);

    let mut device_pins: HashMap<&str, u8> = HashMap::new();
    dependencies_configurations.push(configuration);

    for configuration in dependencies_configurations {
        let hashmap_key = match configuration.sensor_name.as_str() {
            WATER_PUMP_SENSOR_NAME => WATER_PUMP_PIN_KEY,
            WATER_DETECTOR_SENSOR_NAME => WATER_DETECTOR_PIN_KEY,
            RELAY_POWER_SENSOR_NAME => RELAY_BAR_PIN_KEY,
            _ => panic!("Sensor is not supported"),
        };

        device_pins.insert(hashmap_key, configuration.bcm_pin as u8);
        already_configured_devices.push(configuration);
    }

    device_pins
}

fn add_job_to_scheduler(scheduler: &JobScheduler, schedule: Schedule) -> Vec<Uuid> {
    // TODO Implement with new tables
    let database_connection: &mut SqliteConnection = &mut get_database_connection();

    let configurations = get_configurations_by_schedule_id(schedule.id, database_connection);
    let mut job_ids = Vec::new();

    let mut already_configured_devices: Vec<Configuration> = Vec::new();

    for configuration in &configurations {
        let cron_schedule_str = schedule.cron_string.as_str();

        let device_pins = get_device_pins_from_configuration(
            configuration,
            &mut already_configured_devices,
            database_connection,
        );
        println!("This schedule will run at {}", cron_schedule_str);

        let mut device = build_device(configuration.sensor_name, device_pins);
        match schedule.action.as_str() {
            TURN_ON_ACTION => {
                println!("Adding turn_on for configuration {}", 0);
                let job = Job::new(cron_schedule_str, move |_, _| {
                    device.turn_on();
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
    let schedules = get_all_schedules(&mut get_database_connection());

    for schedule in schedules {
        for job_id in add_job_to_scheduler(&scheduler, schedule) {
            job_ids.push(job_id);
        }
    }

    job_ids
}

#[cfg(test)]
mod test {
    use crate::constants::{WATER_DETECTOR_PIN_KEY, WATER_PUMP_PIN_KEY};
    use crate::database::helpers::get_database_connection;
    use crate::models::Configuration;
    use crate::schema::configurations;
    use diesel::prelude::*;
    use diesel::result::Error;

    use std::collections::HashMap;

    use super::get_device_pins_from_configuration;

    #[test]
    fn given_get_device_pins_from_config_when_using_water_detector_should_return_two_key() {
        get_database_connection().test_transaction::<_, Error, _>(|connection| {
            let water_detector_configuration = configurations::table
                .filter(configurations::sensor_name.eq("water_detector"))
                .first::<Configuration>(connection)
                .expect("Error Loading Configurations");

            let mut already_configured_devices: Vec<Configuration> = Vec::new();
            let device_pins: HashMap<String, u8> = get_device_pins_from_configuration(
                water_detector_configuration,
                &mut already_configured_devices,
                connection,
            );

            assert_eq!(device_pins.len(), 2);
            assert_eq!(
                device_pins
                    .keys()
                    .any(|pin_key| pin_key == WATER_DETECTOR_PIN_KEY),
                true
            );
            assert_eq!(
                device_pins
                    .keys()
                    .any(|pin_key| pin_key == WATER_PUMP_PIN_KEY),
                true
            );

            Ok(())
        });
    }
}
