use diesel::Connection;
use diesel::{result::Error, SqliteConnection};
use gateway::database::helpers::{get_all_configurations, get_database_connection};

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
        let sensor_f_configuration = create_configuration("Sensor F".to_string(), 10, connection);
        let sensor_g_configuration = create_configuration("Sensor G".to_string(), 11, connection);

        let schedule =
            create_schedule("0 0 0 * * *".to_string(), "turn_on".to_string(), connection);

        link_configuration_to_schedule(schedule.id, sensor_f_configuration.id, connection);
        link_configuration_to_schedule(schedule.id, sensor_g_configuration.id, connection);

        Ok(())
    });
}
