struct CLIError {
    pub name: String,
    pub message: String,
}

struct CLIArgs {
    pub action: String,
    pub key: String,
    pub value: String,
    pub errors: Vec<CLIError>,
}

impl CLIArgs {
    pub fn is_valid(&mut self) -> bool {
        false
    }
}

pub fn parse_cli_args() {}

#[cfg(test)]
mod tests {
    use crate::cli::CLIArgs;

    #[test]
    fn cli_args_new_action_config() {
        let config_action = String::from("config");
        let empty_string = String::from("");
        let mut args = CLIArgs {
            action: config_action,
            key: empty_string.clone(),
            value: empty_string.clone(),
            errors: vec![],
        };
        let _is_valid = args.is_valid();

        assert_eq!(_is_valid, true);
    }
}
