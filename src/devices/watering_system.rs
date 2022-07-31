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