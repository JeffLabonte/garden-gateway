mod constants;
pub mod watering_system;
pub mod relay_power;

use std::collections::HashMap;

trait Device {
    fn new(sensor_pins: HashMap<&str, u8>) -> Self;
    fn turn_on(&mut self);
    fn turn_off(&mut self);
}
