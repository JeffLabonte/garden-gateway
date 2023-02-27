use super::*;
use gateway::database::helpers::get_configuration_dependencies_from_config_id;
use gateway::database::helpers::{
    get_all_configurations, get_all_schedules, get_configurations_by_schedule_id,
    get_schedules_from_config_id,
};

use crate::common::execute_test;
use crate::common::{create_configuration, create_schedule, link_configuration_to_schedule};

#[test]
fn given_get_all_configurations_when_has_five_configurations_then_should_return_five_configs() {
    execute_test(|| {
        let configurations = get_all_configurations();
        assert_eq!(configurations.len(), 0);

        let sensor_a_configuration = create_configuration("Sensor A".to_string(), 61);
        let sensor_b_configuration = create_configuration("Sensor B".to_string(), 62);
        let sensor_c_configuration = create_configuration("Sensor C".to_string(), 63);
        let sensor_d_configuration = create_configuration("Sensor D".to_string(), 64);
        let sensor_e_configuration = create_configuration("Sensor E".to_string(), 65);

        let configurations = get_all_configurations();
        println!("{:?}", configurations);
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
    })
}

#[test]
fn given_get_all_schedules_when_multiple_schedules_should_return_all_schedules() {
    execute_test(|| {
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
    });
}
