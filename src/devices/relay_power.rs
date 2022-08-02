use std::collections::HashMap;

use rust_gpiozero::OutputDevice;

use crate::helpers::println_now;

use super::{constants::{TURN_ON_STRING, RELAY_POWER_BAR, TURN_OFF_STRING}, Device};


pub struct RelayPowerBar {
    relay_power_device: OutputDevice,
}


impl Device for RelayPowerBar {
    fn new(sensor_pins: HashMap<&str, u8>) -> RelayPowerBar {
        let relay_power_pin : u8 = *sensor_pins.get(&"relay_power_pin").unwrap();
        let relay_power_device = OutputDevice::new(relay_power_pin);
        RelayPowerBar { relay_power_device }
    }

    fn turn_on(&mut self) {
        println_now(TURN_ON_STRING, RELAY_POWER_BAR);
        self.relay_power_device.off();
    }

    fn turn_off(&mut self) {
        println_now(TURN_OFF_STRING, RELAY_POWER_BAR);
        self.relay_power_device.on();
    }
}