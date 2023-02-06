use std::sync::Mutex;

use lazy_static::lazy_static;
use rust_gpiozero::OutputDevice;

use crate::{
    constants::{RELAY_POWER_PIN_KEY, RELAY_POWER_SENSOR_NAME},
    helpers::println_now,
};

use super::{constants::RELAY_POWER_BAR, get_device_pin_number, Device};

#[cfg(not(test))]
use super::constants::{TURN_OFF_STRING, TURN_ON_STRING};

#[cfg(not(test))]
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
        println_now(TURN_ON_STRING, RELAY_POWER_BAR);
        RELAY_POWER_DEVICE.lock().unwrap().off();
    }

    fn turn_off(&mut self) {
        println_now(TURN_OFF_STRING, RELAY_POWER_BAR);
        RELAY_POWER_DEVICE.lock().unwrap().on();
    }
}

#[cfg(test)]
impl Device for RelayPowerBar {
    fn turn_on(&mut self) {}

    fn turn_off(&mut self) {}
}
