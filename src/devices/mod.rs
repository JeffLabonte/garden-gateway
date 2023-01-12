mod constants;
pub mod relay_power;
pub mod watering_system;

use std::collections::HashMap;

use crate::constants::{WATER_DETECTOR_SENSOR_NAME, WATER_PUMP_SENSOR_NAME};

use self::{relay_power::RelayPowerBar, watering_system::WateringSystem};

pub trait Device {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
}

pub fn build_device(
    sensor_name: String,
    sensor_pins: HashMap<&str, u8>,
) -> Box<dyn Device + Send + Sync> {
    match sensor_name.as_str() {
        constants::RELAY_POWER_BAR => Box::new(RelayPowerBar::new(sensor_pins)),
        WATER_DETECTOR_SENSOR_NAME | WATER_PUMP_SENSOR_NAME => {
            Box::new(WateringSystem::new(sensor_pins))
        }
        _ => panic!("Unknown action"),
    }
}
