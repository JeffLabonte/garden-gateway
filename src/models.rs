use super::schema::configurations;

#[derive(Queryable, Insertable)]
pub struct Configurations {
    pub id: u32,
    pub sensor_name: &'a str,
    pub bcm_pin: u32,
}
