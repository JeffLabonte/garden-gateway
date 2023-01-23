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
    sensor_name: &str,
    sensor_pins: HashMap<String, u8>,
) -> Box<dyn Device + Send + Sync> {
    match sensor_name {
        constants::RELAY_POWER_BAR => Box::new(RelayPowerBar::new(sensor_pins)),
        WATER_DETECTOR_SENSOR_NAME | WATER_PUMP_SENSOR_NAME => {
            Box::new(WateringSystem::new(sensor_pins))
        }
        _ => panic!("Unknown action"),
    }
}

#[cfg(test)]
mod test {
    use std;
    use std::collections::HashMap;

    use mockall::predicate::*;
    use mockall::*;

    use crate::constants::RELAY_POWER_PIN_KEY;

    use super::build_device;
    use super::constants::RELAY_POWER_BAR;
    use super::relay_power::RelayPowerBar;

    #[automock]
    pub trait Device {
        fn new(sensor_pins: HashMap<String, u8>) -> RelayPowerBar;
        fn turn_on(&mut self);
        fn turn_off(&mut self);
    }

    #[test]
    fn test_given_build_device_when_passing_valid_parameter_w_relay_powerbar_should_return_box_w_relay_powerbar(
    ) {
        let sensor_name = RELAY_POWER_BAR;
        let mut sensor_pins: HashMap<String, u8> = HashMap::new();
        sensor_pins
            .entry(RELAY_POWER_PIN_KEY.to_string())
            .or_insert(17 as u8);
        let relay_power_bar = build_device(sensor_name, sensor_pins);
    }
}
