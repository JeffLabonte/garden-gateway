use rust_gpiozero::output_devices::OutputDevice;

pub trait Devices {
    fn is_what_type(&self) -> String;
}

impl Devices for RelayPowerBar {
    fn is_what_type(&self) -> String {
        String::from("RelayPowerBar")
    }
}

pub struct RelayPowerBar {
    relay_power_pin: OutputDevice,
}

impl RelayPowerBar {
    pub fn new(bcm_pin: u8) -> RelayPowerBar {
        let relay_power_pin = OutputDevice::new(bcm_pin);
        RelayPowerBar { relay_power_pin }
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
