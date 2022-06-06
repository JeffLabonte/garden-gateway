#[derive(Queryable)]
pub struct Configurations {
    pub id: u32,
    pub sensor_name: String,
    pub bcm_pin: u32,
}
