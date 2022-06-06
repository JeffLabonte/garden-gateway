use rust_gpiozero::output_devices::OutputDevice;

pub struct RelayPowerBar {
    relay_power_pin: OutputDevice,
}

impl RelayPowerBar {
    pub fn new(&mut self, bcm_pin: u8) -> RelayPowerBar {
        self.relay_power_pin = OutputDevice::new(bcm_pin);
    }

    pub fn turn_on(&mut self) {
        if !self.relay_power_pin.is_active() {
            self.relay_power_pin.on();
        }
    }

    pub fn turn_off(&mut self) {
        if self.relay_power_pin.is_active() {
            self.relay_power_pin.off();
        }
    }
}
