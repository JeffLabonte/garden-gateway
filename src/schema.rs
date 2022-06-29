table! {
    configurations (id) {
        id -> Integer,
        sensor_name -> Text,
        bcm_pin -> Integer,
    }
}

table! {
    schedules (id) {
        id -> Integer,
        action -> Text,
        configuration_id -> Integer,
        cron_string -> Text,
    }
}
