use rust_gpiozero::output_devices::OutputDevice;

pub struct RelayPowerBar {
    relay_power_pin: OutputDevice,
}

impl RelayPowerBar {
    pub fn new(bcm_pin: u8) -> RelayPowerBar {
        let relay_power_pin = OutputDevice::new(bcm_pin);
        RelayPowerBar { relay_power_pin }
    }

    pub fn turn_on(&mut self) {
        println!("Turning on pin to RelayPowerBar");
        if !self.relay_power_pin.is_active() {
            self.relay_power_pin.off();
        }
    }

    pub fn turn_off(&mut self) {
        println!("Turning off pin to RelayPowerBar");
        if self.relay_power_pin.is_active() {
            self.relay_power_pin.on();
        }
    }
}
