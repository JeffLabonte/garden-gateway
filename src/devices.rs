use rust_gpiozero::{output_devices::OutputDevice, InputDevice};
use crate::helpers::println_now;

const TURN_ON_STRING: &str = "TURN ON";
const TURN_OFF_STRING: &str = "TURN OFF";

const RELAY_POWER_BAR: &str = "RelayPowerBar";
const WATER_PUMP: &str = "WaterPump";


pub struct RelayPowerBar {
    relay_power_pin: OutputDevice,
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
        let relay_power_pin = OutputDevice::new(bcm_pin);
        RelayPowerBar { relay_power_pin }
    }

    pub fn turn_on(&mut self) {
        println_now(TURN_ON_STRING, RELAY_POWER_BAR);
        self.relay_power_pin.off();
    }

    pub fn turn_off(&mut self) {
        println_now(TURN_OFF_STRING, RELAY_POWER_BAR);
        self.relay_power_pin.on();
    }
}

impl WaterPump {
    pub fn new(gpio_pin: u8) -> WaterPump {
        let gpio_device = OutputDevice::new(gpio_pin);
        WaterPump { gpio_device }
    }

    pub fn turn_on(&mut self) {
        println_now(TURN_ON_STRING, WATER_PUMP);
        self.gpio_pin.on();
    }

    pub fn turn_off(&mut self) {
        println_now(TURN_OFF_STRING, WATER_PUMP);
        self.gpio_pin.off();
    }
}

impl WaterDetector {
    pub fn new(bcm_pin: u8) -> WaterDetector {
        let input_device: InputDevice = InputDevice::new(bcm_pin);
        WaterDetector { input_device }
    }

    pub fn is_active(&mut self) -> bool{
        self.input_device.is_active()
    }
}