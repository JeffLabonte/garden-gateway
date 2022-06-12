use std::process::ExitStatusError;

struct CLIArgs {
    action: String,
    key: String,
    value: String,
}

impl CLIArgs {
    pub fn new(action: String, key: String, value: String) -> Result(Ok(), Err()) {
        match action.as_str() {
            "config" => {
                println!("This is config")
            }
            "run" => {
                println!("Do nothing")
            }
            _ => panic!("Not supposed to happen");
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
        let args = CLIArgs::new("config", "", "");

        assert_eq!(args, Ok)
    }
}
