mod constants;
pub mod watering_system;
pub mod relay_power;

use crate::helpers::println_now;
use rust_gpiozero::{output_devices::OutputDevice};

use self::constants::{RELAY_POWER_BAR, TURN_ON_STRING, TURN_OFF_STRING};

trait Device {
    fn new(bcm_pin: u8) -> Self;
    fn turn_on(&mut self);
    fn turn_off(&mut self);
}
