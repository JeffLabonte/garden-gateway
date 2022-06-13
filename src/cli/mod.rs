struct CLIArgs {
    pub action: String,
    pub key: String,
    pub value: String,
}

impl CLIArgs {
    pub fn new(action: String, key: String, value: String) -> Self {
        match action.as_str() {
            "config" => {
                println!("This is config")
            }
            "run" => {
                println!("Do nothing")
            }
            _ => panic!("Not supposed to happen"),
        }

        CLIArgs { action, key, value }
    }
}

pub fn parse_cli_args() {}

#[cfg(test)]
mod tests {
    use crate::cli::CLIArgs;

    #[test]
    fn cli_args__new__action_config() {
        let config_action = String::from("config");
        let empty_string = String::from("");
        let args = CLIArgs::new(config_action, empty_string.clone(), empty_string.clone());

        assert_eq!(args.action, String::from("config"));
    }
}
