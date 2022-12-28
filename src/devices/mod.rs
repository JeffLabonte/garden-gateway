mod constants;
pub mod relay_power;
pub mod watering_system;

use std::collections::HashMap;

use self::{relay_power::RelayPowerBar, watering_system::WateringSystem};

pub trait Device {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
}

pub fn build_device(action: &str, sensor_pins: HashMap<&str, u8>) -> Box<dyn Device> {
    match action {
        constants::TURN_ON_STRING => Box::new(RelayPowerBar::new(sensor_pins)),
        constants::WATER_PUMP => Box::new(WateringSystem::new(sensor_pins)),
        _ => panic!("Unknown action"),
    }
}
