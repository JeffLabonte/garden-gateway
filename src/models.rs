use crate::schema::configurations;

#[derive(Queryable, Insertable)]
pub struct Configuration {
    pub id: i32,
    pub sensor_name: String,
    pub bcm_pin: i32,
}
