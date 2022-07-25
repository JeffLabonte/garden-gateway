use crate::helpers::println_now;
use rust_gpiozero::{output_devices::OutputDevice, InputDevice};
use std::time::Duration;

const TURN_ON_STRING: &str = "TURN ON";
const TURN_OFF_STRING: &str = "TURN OFF";

const RELAY_POWER_BAR: &str = "RelayPowerBar";
const WATER_PUMP: &str = "WaterPump";

pub struct RelayPowerBar {
    relay_power_device: OutputDevice,
}

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

impl WateringSystem {
    pub fn new(water_detector_pin: u8, water_pump_pin: u8) -> WateringSystem {
        WateringSystem {
            water_pump: WaterPump::new(water_pump_pin),
            water_detector: WaterDetector::new(water_detector_pin),
        }
    }

    pub fn water_until_filled(&mut self) {
        while !self.water_detector.has_water() {
            self.water_pump.turn_on();
            std::thread::sleep(Duration::from_secs(1));
            self.water_pump.turn_off();
        }
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
