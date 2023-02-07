use super::constants::{TURN_OFF_STRING, TURN_ON_STRING, WATER_PUMP};
use super::Device;
use crate::constants::{WATER_DETECTOR_SENSOR_NAME, WATER_PUMP_SENSOR_NAME};
use crate::devices::get_device_pin_number;
use crate::helpers::println_now;
use lazy_static::lazy_static;
use rust_gpiozero::{InputDevice, OutputDevice};
use std::sync::Mutex;
use std::time::Duration;

lazy_static! {
    static ref WATER_DETECTOR_DEVICE: Mutex<InputDevice> = Mutex::new(InputDevice::new(
        get_device_pin_number(WATER_DETECTOR_SENSOR_NAME)
    ));
    static ref WATER_PUMP_DEVICE: Mutex<OutputDevice> = Mutex::new(OutputDevice::new(
        get_device_pin_number(WATER_PUMP_SENSOR_NAME)
    ));
}

pub struct WateringSystem {
    water_pump: WaterPump,
    water_detector: WaterDetector,
}

pub struct WaterDetector {}

pub struct WaterPump {}

impl WateringSystem {
    pub fn new() -> WateringSystem {
        WateringSystem {
            water_pump: WaterPump {},
            water_detector: WaterDetector {},
        }
    }
}

#[cfg(not(test))]
impl Device for WateringSystem {
    fn turn_on(&mut self) {
        //
        // Let the water pump run untill the water detector detects water.
        //
        while !self.water_detector.has_water() {
            println_now(TURN_ON_STRING, WATER_PUMP_SENSOR_NAME);
            self.water_pump.turn_on();
            std::thread::sleep(Duration::from_secs(1));
            println_now(TURN_OFF_STRING, WATER_PUMP_SENSOR_NAME);
            self.water_pump.turn_off();
        }
    }

    fn turn_off(&mut self) {
        self.water_pump.turn_off();
    }
}

#[cfg(not(test))]
impl WaterPump {
    pub fn turn_on(&mut self) {
        println_now(TURN_ON_STRING, WATER_PUMP);
        WATER_PUMP_DEVICE.lock().unwrap().on();
    }

    pub fn turn_off(&mut self) {
        println_now(TURN_OFF_STRING, WATER_PUMP);
        WATER_PUMP_DEVICE.lock().unwrap().off();
    }
}

#[cfg(not(test))]
impl WaterDetector {
    pub fn has_water(&mut self) -> bool {
        WATER_DETECTOR_DEVICE.lock().unwrap().is_active()
    }
}

/*
*
*   Mock Structure for test purpuses
*
*   VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV
*/

#[cfg(test)]
impl Device for WateringSystem {
    fn turn_on(&mut self) {}
    fn turn_off(&mut self) {}
}

#[cfg(test)]
impl WaterPump {
    pub fn turn_on(&mut self) {}

    pub fn turn_off(&mut self) {}
}

#[cfg(test)]
impl WaterDetector {
    pub fn has_water(&mut self) -> bool {
        false
    }
}
