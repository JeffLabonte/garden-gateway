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
        let now = chrono::Local::now();
        println!("{} - Running: Turning ON pin to RelayPowerBar", now.format("%b %-d, %-I:%M:%s").to_string());
        self.relay_power_pin.off();
    }

    pub fn turn_off(&mut self) {
        let now = chrono::Local::now();
        println!("{} - Running: Turning OFF pin to RelayPowerBar", now.format("%b %-d, %-I:%M:%s").to_string());
        self.relay_power_pin.on();
    }
}
