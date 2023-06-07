use std::sync::Mutex;

use lazy_static::lazy_static;
use rust_gpiozero::OutputDevice;

use crate::constants::RELAY_POWER_SENSOR_NAME;

use super::{get_device_pin_number, Device};

lazy_static! {
    pub static ref RELAY_POWER_DEVICE: Mutex<OutputDevice> = Mutex::new(OutputDevice::new(
        get_device_pin_number(RELAY_POWER_SENSOR_NAME)
    ));
}

#[derive(Debug)]
pub struct RelayPowerBar {}

#[cfg(not(test))]
impl Device for RelayPowerBar {
    fn turn_on(&mut self) {
        println!("Turning on Relay");
        RELAY_POWER_DEVICE.lock().unwrap().off();
    }

    fn turn_off(&mut self) {
        println!("Turning OFF Relay");
        RELAY_POWER_DEVICE.lock().unwrap().on();
    }
}

#[cfg(test)]
impl Device for RelayPowerBar {
    fn turn_on(&mut self) {}

    fn turn_off(&mut self) {}
}
