use crate::models::Schedule;

pub fn retrieve_schedules_from_config_id(config_id: u32) -> Vec<Schedule> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Schedule;

    #[test]
    fn test_retrieve_schedules_from_config_id() {}
}