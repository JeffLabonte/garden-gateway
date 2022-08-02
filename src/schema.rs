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
        cron_string -> Text,
        configuration_id -> Integer,
    }
}

table! {
    schedule_configurations (id) {
        id -> Integer,
        schedule_id -> Integer,
        configuration_id -> Integer,
    }
}

joinable!(schedule_configurations -> schedules (schedule_id));
joinable!(schedule_configurations -> configurations (configuration_id));

allow_tables_to_appear_in_same_query!(
    configurations,
    schedules,
    schedule_configurations,
);