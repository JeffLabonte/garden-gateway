use std::thread::{self, JoinHandle};

use crate::devices::RelayPowerBar;
use crate::models::*;
use diesel::prelude::*;

pub fn run(database: SqliteConnection) -> bool {
    use crate::schema::configurations::dsl::configurations;

    let configs = configurations
        .load::<Configuration>(&database)
        .expect("Error loading configurations");

    let mut thread_handlers: Vec<JoinHandle<()>> = Vec::new();
    for config in configs {
        let bcm_pin: u8 = config.bcm_pin as u8;

        let is_success = match config.sensor_name.as_str() {
            "relay_bar" => {
                let relay_power_bar = RelayPowerBar::new(bcm_pin);
                let handle = thread::spawn(move || {});
                thread_handlers.push(handle);
                true
            }
            _ => false,
        };

        if !is_success {
            return is_success;
        }
    }

    true
}
