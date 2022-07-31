mod constants;
pub mod watering_system;

use crate::helpers::println_now;
use rust_gpiozero::{output_devices::OutputDevice, InputDevice};

use self::constants::{WATER_PUMP, RELAY_POWER_BAR, TURN_ON_STRING, TURN_OFF_STRING};

pub struct RelayPowerBar {
    relay_power_device: OutputDevice,
}


impl RelayPowerBar {
    pub fn new(bcm_pin: u8) -> RelayPowerBar {
        let relay_power_device = OutputDevice::new(bcm_pin);
        RelayPowerBar { relay_power_device }
    }

    pub fn turn_on(&mut self) {
        println_now(TURN_ON_STRING, RELAY_POWER_BAR);
        self.relay_power_device.off();
    }

    pub fn turn_off(&mut self) {
        println_now(TURN_OFF_STRING, RELAY_POWER_BAR);
        self.relay_power_device.on();
    }
}
