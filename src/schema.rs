table! {
    configurations (id) {
        id -> Integer,
        sensor_name -> Text,
        bcm_pin -> Integer,
    }
}

table! {
    schedule_configurations (id) {
        id -> Integer,
        configuration_id -> Integer,
        schedule_id -> Integer,
    }
}

table! {
    schedules (id) {
        id -> Integer,
        action -> Text,
        cron_string -> Text,
    }
}

joinable!(schedule_configurations -> configurations (configuration_id));

allow_tables_to_appear_in_same_query!(
    configurations,
    schedule_configurations,
    schedules,
);
