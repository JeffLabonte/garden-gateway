use gateway::database::helpers::get_configuration_dependencies_from_config_id;
use gateway::database::helpers::{
    get_all_configurations, get_all_schedules, get_configurations_by_schedule_id,
    get_schedules_from_config_id,
};

use crate::common::get_configuration_by_sensor_name;
use crate::common::{
    create_configuration, create_schedule, link_configuration_to_schedule, teardown_configuration,
    teardown_schedule,
};

#[test]
fn given_get_all_configurations_when_has_five_configurations_then_should_return_five_configs() {
    let configurations = get_all_configurations();
    assert_eq!(configurations.len(), 3);

    let sensor_a_configuration = create_configuration("Sensor A".to_string(), 61);
    let sensor_b_configuration = create_configuration("Sensor B".to_string(), 62);
    let sensor_c_configuration = create_configuration("Sensor C".to_string(), 63);
    let sensor_d_configuration = create_configuration("Sensor D".to_string(), 64);
    let sensor_e_configuration = create_configuration("Sensor E".to_string(), 65);

    let configurations = get_all_configurations();
    println!("{:?}", configurations);
    assert_eq!(configurations.len(), 8);

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

    teardown_configuration();
}

#[test]
fn given_get_confgurations_from_schedule_id_when_two_configuration_linked_config_then_should_return_two(
) {
    let sensor_a_configuration = create_configuration("Sensor A".to_string(), 60);
    let sensor_b_configuration = create_configuration("Sensor B".to_string(), 61);

    let schedule = create_schedule("0 0 0 * * *".to_string(), "turn_on".to_string());

    link_configuration_to_schedule(schedule.id, sensor_a_configuration.id);
    link_configuration_to_schedule(schedule.id, sensor_b_configuration.id);

    let configurations = get_configurations_by_schedule_id(schedule.id);
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

    teardown_configuration();
}

#[test]
fn given_get_schedules_from_config_id_when_multiple_schedule_on_one_device_should_return_multiple_schedules(
) {
    let config = create_configuration("Sensor A".to_string(), 10);

    let schedule_a = create_schedule("* * * * * *".to_string(), "turn_on".to_string());
    let schedule_b = create_schedule("* * * * * * 1".to_string(), "turn_on".to_string());

    link_configuration_to_schedule(schedule_a.id, config.id);
    link_configuration_to_schedule(schedule_b.id, config.id);

    let schedules = get_schedules_from_config_id(config.id);

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

    teardown_configuration();
}

#[test]
fn given_get_all_schedules_when_multiple_schedules_should_return_all_schedules() {
    let schedule_a = create_schedule("* * * * * * *".to_string(), "turn_off".to_string());

    let schedule_b = create_schedule("* * * * * * *".to_string(), "turn_on".to_string());

    let schedules = get_all_schedules();

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

    teardown_schedule();
}

#[test]
fn given_get_configuration_dependencies_from_config_id() {
    let water_detector_config = get_configuration_by_sensor_name("water_detector".to_string());

    let configuration_deps =
        get_configuration_dependencies_from_config_id(water_detector_config.id);

    assert_eq!(configuration_deps.len(), 1);

    let configuration_dep = &configuration_deps[0];

    assert_eq!(configuration_dep.sensor_name, "water_pump".to_string());
}
