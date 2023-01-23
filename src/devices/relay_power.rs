use std::collections::HashMap;

use rust_gpiozero::OutputDevice;

use crate::{constants::RELAY_POWER_PIN_KEY, helpers::println_now};

use super::{constants::RELAY_POWER_BAR, Device};

#[cfg(not(test))]
use super::constants::{TURN_OFF_STRING, TURN_ON_STRING};

#[cfg(not(test))]
#[derive(Debug)]
pub struct RelayPowerBar {
    relay_power_device: OutputDevice,
}

#[cfg(not(test))]
impl RelayPowerBar {
    pub fn new(sensor_pins: HashMap<String, u8>) -> RelayPowerBar {
        let relay_power_pin: u8 = *sensor_pins.get(RELAY_POWER_PIN_KEY).unwrap();
        let relay_power_device = OutputDevice::new(relay_power_pin);
        RelayPowerBar { relay_power_device }
    }
}

#[cfg(not(test))]
impl Device for RelayPowerBar {
    fn turn_on(&mut self) {
        println_now(TURN_ON_STRING, RELAY_POWER_BAR);
        self.relay_power_device.off();
    }

    fn turn_off(&mut self) {
        println_now(TURN_OFF_STRING, RELAY_POWER_BAR);
        self.relay_power_device.on();
    }
}

#[cfg(test)]
pub struct RelayPowerBar {}

#[cfg(test)]
impl RelayPowerBar {
    pub fn new(sensor_pins: HashMap<String, u8>) -> RelayPowerBar {
        let relay_power_pin: u8 = *sensor_pins.get(RELAY_POWER_PIN_KEY).unwrap();
        assert!(relay_power_pin > 0);
        RelayPowerBar {}
    }
}

#[cfg(test)]
impl Device for RelayPowerBar {
    fn turn_on(&mut self) {}

    fn turn_off(&mut self) {}
}
