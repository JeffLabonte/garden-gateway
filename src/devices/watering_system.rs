use rust_gpiozero::{InputDevice, OutputDevice};

use std::{collections::HashMap, time::Duration};

use crate::{
    constants::{WATER_DETECTOR_PIN_KEY, WATER_PUMP_PIN_KEY},
    helpers::println_now,
};

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
    pub fn new(sensor_pins: HashMap<String, u8>) -> WateringSystem {
        let water_pump_pin: u8 = *sensor_pins.get(WATER_PUMP_PIN_KEY).unwrap();
        let water_detector_pin: u8 = *sensor_pins.get(WATER_DETECTOR_PIN_KEY).unwrap();
        let water_pump = WaterPump::new(water_pump_pin);
        let water_detector = WaterDetector::new(water_detector_pin);
        WateringSystem {
            water_pump,
            water_detector,
        }
    }
}

impl Device for WateringSystem {
    fn turn_on(&mut self) {
        //
        // Let the water pump run untill the water detector detects water.
        //
        while !self.water_detector.has_water() {
            self.water_pump.turn_on();
            std::thread::sleep(Duration::from_secs(1));
            self.water_pump.turn_off();
        }
    }

    fn turn_off(&mut self) {
        self.water_pump.turn_off();
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
