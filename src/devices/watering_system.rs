use rust_gpiozero::{InputDevice, OutputDevice};

use core::panic;
use std::{collections::HashMap, time::Duration};

use crate::helpers::println_now;

use super::{
    constants::{TURN_OFF_STRING, TURN_ON_STRING, WATER_PUMP},
    Device,
};

pub struct WateringSystem {
    water_pump: WaterPump,
    water_detector: WaterDetector,
}

pub struct WaterDetector {
    input_device: InputDevice,
}

pub struct WaterPump {
    gpio_device: OutputDevice,
}

impl WateringSystem {
    pub fn new(sensor_pins: HashMap<&str, u8>) -> WateringSystem {
        let water_pump_pin: u8 = *sensor_pins.get(&"water_pump_pin").unwrap();
        let water_detector_pin: u8 = *sensor_pins.get(&"water_detector_pin").unwrap();
        let water_pump = WaterPump::new(water_pump_pin);
        let water_detector = WaterDetector::new(water_detector_pin);
        WateringSystem {
            water_pump,
            water_detector,
        }
    }
}

impl Device for WateringSystem {
    // Water the plant until water is detected
    fn turn_on(&mut self) {
        while !self.water_detector.has_water() {
            self.water_pump.turn_on();
            std::thread::sleep(Duration::from_secs(1));
            self.water_pump.turn_off();
        }
    }

    // Will panic since you can't really turn it off
    fn turn_off(&mut self) {
        panic!("This device doesn't implement turn_off")
    }
}

impl WaterPump {
    pub fn new(gpio_pin: u8) -> WaterPump {
        let gpio_device = OutputDevice::new(gpio_pin);
        WaterPump { gpio_device }
    }

    pub fn turn_on(&mut self) {
        println_now(TURN_ON_STRING, WATER_PUMP);
        self.gpio_device.on();
    }

    pub fn turn_off(&mut self) {
        println_now(TURN_OFF_STRING, WATER_PUMP);
        self.gpio_device.off();
    }
}

impl WaterDetector {
    pub fn new(bcm_pin: u8) -> WaterDetector {
        let input_device: InputDevice = InputDevice::new(bcm_pin);
        WaterDetector { input_device }
    }

    pub fn has_water(&mut self) -> bool {
        self.input_device.is_active()
    }
}

