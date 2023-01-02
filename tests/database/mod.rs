use gateway::database::helpers::get_all_configurations;

use crate::common::{create_configuration, create_schedule, link_configuration_to_schedule};

#[test]
fn given_get_all_configurations__when_has_five_configurations__then_should_return_five_configs()
{
    let sensor_a_configuration = create_configuration("Sensor A".to_string(), 1);
    let sensor_b_configuration = create_configuration("Sensor B".to_string(), 2);
    let sensor_c_configuration = create_configuration("Sensor C".to_string(), 3);
    let sensor_d_configuration = create_configuration("Sensor D".to_string(), 4);
    let sensor_e_configuration = create_configuration("Sensor E".to_string(), 5);

    let configurations = get_all_configurations();
    assert_eq!(configurations.len(), 5);

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
}

#[test]
fn given_get_confgurations_from_schedule_id__when_two_configuration_linked_config__then_should_return_two(
) {
    let sensor_f_configuration = create_configuration("Sensor F".to_string(), 10);
    let sensor_g_configuration = create_configuration("Sensor G".to_string(), 11);

    let schedule = create_schedule("0 0 0 * * *".to_string(), "turn_on".to_string());

    link_configuration_to_schedule(schedule.id, sensor_f_configuration.id);
    link_configuration_to_schedule(schedule.id, sensor_g_configuration.id);
}
