use clap::Parser;

pub mod actions;

#[derive(Parser)]
pub struct CLIArgs {
    #[clap(short, required = true)]
    pub action: String,

    #[clap(short, required = false, default_value_t = String::from(""))]
    pub sub_action: String,

    #[clap(short, long, value_parser, default_value_t = String::from(""))]
    pub key: String,

    #[clap(short, long, value_parser, default_value_t = String::from(""))]
    pub value: String,
}

fn is_args_valid(args: &CLIArgs) -> bool {
    match args.action.as_str() {
        "config" => match args.sub_action.as_str() {
            "list" => true,
            "set" | "remove" => {
                if args.key.is_empty() || args.value.is_empty() {
                    return false;
                }
                true
            }
            _ => false,
        },
        "run" => true,
        _ => false,
    }
}

pub fn get_cli_args() -> Result<CLIArgs, String> {
    let args = CLIArgs::parse();
    if is_args_valid(&args) {
        return Ok(args);
    }
    Err(String::from("Invalid Arguments"))
}

#[cfg(test)]
mod tests {
    use super::{is_args_valid, CLIArgs};
    use test_case::test_case;

    #[test]
    fn run_action_args_is_valid() {
        let empty_string = String::from("");
        let cli_args = CLIArgs {
            action: String::from("run"),
            sub_action: empty_string.clone(),
            key: empty_string.clone(),
            value: empty_string.clone(),
        };

        assert_eq!(is_args_valid(&cli_args), true);
    }

    #[test]
    fn config_action_no_subaction_is_not_valid() {
        let empty_string = String::from("");
        let cli_args = CLIArgs {
            action: String::from("config"),
            sub_action: empty_string.clone(),
            key: empty_string.clone(),
            value: empty_string.clone(),
        };

        assert_eq!(is_args_valid(&cli_args), false);
    }

    #[test]
    fn config_action_list_subaction_is_valid() {
        let empty_string = String::from("");
        let cli_args = CLIArgs {
            action: String::from("config"),
            sub_action: String::from("list"),
            key: empty_string.clone(),
            value: empty_string.clone(),
        };

        assert_eq!(is_args_valid(&cli_args), true);
    }

    #[test_case("config", "set", "", "", false ; "action: config, sub_action: set no key or value")]
    #[test_case("config", "set", "something", "", false ; "action: config, sub_action: set no value")]
    #[test_case("config", "set", "", "something", false ; "action: config, sub_action: set no key")]
    #[test_case("config", "set", "something", "something", true ; "action: config, sub_action: set with key and value")]
    #[test_case("config", "remove", "", "", false ; "action: config, sub_action: remove no key or value")]
    #[test_case("config", "remove", "something", "", false ; "action: config, sub_action: remove no value")]
    #[test_case("config", "remove", "", "something", false ; "action: config, sub_action: remove no key")]
    #[test_case("config", "remove", "something", "something", true ; "action: config, sub_action: remove with key and value")]
    fn config_action_with_subactions_not_valid(
        action: &str,
        sub_action: &str,
        key: &str,
        value: &str,
        is_valid: bool,
    ) {
        let cli_args = CLIArgs {
            action: String::from(action),
            sub_action: String::from(sub_action),
            key: String::from(key),
            value: String::from(value),
        };

        assert_eq!(is_args_valid(&cli_args), is_valid);
    }
}
