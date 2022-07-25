use rust_gpiozero::output_devices::OutputDevice;
use crate::helpers::println_now;

const TURN_ON_STRING: &str = "TURN ON";
const TURN_OFF_STRING: &str = "TURN OFF";

const RELAY_POWER_BAR: &str = "RelayPowerBar";
const WATER_PUMP: &str = "Water Pump";

pub struct RelayPowerBar {
    relay_power_pin: OutputDevice,
}

pub struct WaterPump {
    gpio_pin: OutputDevice,
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
        let gpio_pin = OutputDevice::new(gpio_pin);
        WaterPump { gpio_pin }
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
