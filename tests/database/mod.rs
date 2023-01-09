use diesel::result::Error;
use diesel::Connection;
use gateway::database::helpers::{
    get_all_configurations, get_all_schedules, get_configurations_by_schedule_id,
    get_database_connection, get_schedules_from_config_id,
};

use crate::common::{create_configuration, create_schedule, link_configuration_to_schedule};

#[test]
fn given_get_all_configurations_when_has_five_configurations_then_should_return_five_configs() {
    get_database_connection().test_transaction::<_, Error, _>(|connection| {
        let configurations = get_all_configurations(connection);
        assert_eq!(configurations.len(), 2);

        let sensor_a_configuration = create_configuration("Sensor A".to_string(), 11, connection);
        let sensor_b_configuration = create_configuration("Sensor B".to_string(), 12, connection);
        let sensor_c_configuration = create_configuration("Sensor C".to_string(), 13, connection);
        let sensor_d_configuration = create_configuration("Sensor D".to_string(), 14, connection);
        let sensor_e_configuration = create_configuration("Sensor E".to_string(), 15, connection);

        let configurations = get_all_configurations(connection);
        println!("{:?}", configurations);
        assert_eq!(configurations.len(), 7);

        assert_eq!(
            configurations
                .iter()
                .any(|config| config.id == sensor_a_configuration.id),
            true
        );

        assert_eq!(
            configurations
                .iter()
                .any(|config| config.id == sensor_b_configuration.id),
            true
        );

        assert_eq!(
            configurations
                .iter()
                .any(|config| config.id == sensor_c_configuration.id),
            true
        );

        assert_eq!(
            configurations
                .iter()
                .any(|config| config.id == sensor_d_configuration.id),
            true
        );

        assert_eq!(
            configurations
                .iter()
                .any(|config| config.id == sensor_e_configuration.id),
            true
        );
        Ok(())
    })
}

#[test]
fn given_get_confgurations_from_schedule_id_when_two_configuration_linked_config_then_should_return_two(
) {
    get_database_connection().test_transaction::<_, Error, _>(|connection| {
        let sensor_a_configuration = create_configuration("Sensor A".to_string(), 10, connection);
        let sensor_b_configuration = create_configuration("Sensor B".to_string(), 11, connection);

        let schedule =
            create_schedule("0 0 0 * * *".to_string(), "turn_on".to_string(), connection);

        link_configuration_to_schedule(schedule.id, sensor_a_configuration.id, connection);
        link_configuration_to_schedule(schedule.id, sensor_b_configuration.id, connection);

        let configurations = get_configurations_by_schedule_id(schedule.id, connection);
        assert_eq!(configurations.len(), 2);

        assert_eq!(
            configurations
                .iter()
                .any(|config| config.id == sensor_a_configuration.id),
            true
        );

        assert_eq!(
            configurations
                .iter()
                .any(|config| config.id == sensor_b_configuration.id),
            true
        );

        Ok(())
    });
}

#[test]
fn given_get_schedules_from_config_id_when_multiple_schedule_on_one_device_should_return_multiple_schedules(
) {
    get_database_connection().test_transaction::<_, Error, _>(|connection| {
        let config = create_configuration("Sensor A".to_string(), 10, connection);

        let schedule_a =
            create_schedule("* * * * * *".to_string(), "turn_on".to_string(), connection);
        let schedule_b = create_schedule(
            "* * * * * * 1".to_string(),
            "turn_on".to_string(),
            connection,
        );

        link_configuration_to_schedule(schedule_a.id, config.id, connection);
        link_configuration_to_schedule(schedule_b.id, config.id, connection);

        let schedules = get_schedules_from_config_id(config.id, connection);

        assert_eq!(schedules.len(), 2);

        assert_eq!(
            schedules
                .iter()
                .any(|schedule| schedule.id == schedule_a.id),
            true
        );

        assert_eq!(
            schedules
                .iter()
                .any(|schedule| schedule.id == schedule_b.id),
            true
        );

        Ok(())
    });
}

#[test]
fn given_get_all_schedules_when_multiple_schedules_should_return_all_schedules() {
    get_database_connection().test_transaction::<_, Error, _>(|connection| {
        let schedule_a = create_schedule(
            "* * * * * * *".to_string(),
            "turn_off".to_string(),
            connection,
        );

        let schedule_b = create_schedule(
            "* * * * * * *".to_string(),
            "turn_on".to_string(),
            connection,
        );

        let schedules = get_all_schedules(connection);

        assert_eq!(schedules.len(), 2);

        assert_eq!(
            schedules
                .iter()
                .any(|schedule| schedule.id == schedule_a.id),
            true
        );
        assert_eq!(
            schedules
                .iter()
                .any(|schedule| schedule.id == schedule_b.id),
            true
        );
        Ok(())
    });
}
