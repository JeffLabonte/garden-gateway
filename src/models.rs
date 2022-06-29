use crate::schema::configurations;

#[derive(Queryable, Insertable)]
pub struct Configuration {
    pub id: i32,
    pub sensor_name: String,
    pub bcm_pin: i32,
}

#[derive(Queryable, Insertable)]
pub struct Schedule {
    pub id: i32,
    pub action: String,
    pub cron_string: String,
    pub configuration_id: i32,
}
