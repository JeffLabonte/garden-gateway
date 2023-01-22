// @generated automatically by Diesel CLI.

diesel::table! {
    configuration_dependencies (id) {
        id -> Integer,
        source_configuration_id -> Integer,
        target_configuration_id -> Integer,
    }
}

diesel::table! {
    configurations (id) {
        id -> Integer,
        sensor_name -> Text,
        bcm_pin -> Integer,
    }
}

diesel::table! {
    schedule_configurations (id) {
        id -> Integer,
        configuration_id -> Integer,
        schedule_id -> Integer,
    }
}

diesel::table! {
    schedules (id) {
        id -> Integer,
        action -> Text,
        cron_string -> Text,
    }
}

diesel::joinable!(schedule_configurations -> configurations (configuration_id));
diesel::joinable!(schedule_configurations -> schedules (schedule_id));

diesel::allow_tables_to_appear_in_same_query!(
    configuration_dependencies,
    configurations,
    schedule_configurations,
    schedules,
);
