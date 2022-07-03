pub mod actions;
mod configs;
pub mod enums;
mod run;

use crate::cli::enums::ActionArgs;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct CLIArgs {
    #[clap(long, required = true)]
    pub action: String,

    #[clap(short, required = false, default_value_t = String::from(""))]
    pub sub_action: String,

    #[clap(short, long, value_parser, default_value_t = String::from(""))]
    pub key: String,

    #[clap(short, long, value_parser, default_value_t = String::from(""))]
    pub value: String,

    #[clap(short = 'i', long = "input", required = false, value_parser)]
    pub import_json_path: PathBuf,
}

fn is_args_valid(args: &CLIArgs) -> bool {
    let action = ActionArgs::from_string(args.action.as_str());
    match action {
        Ok(ActionArgs::Config) => match args.sub_action.as_str() {
            "list" => true,
            "set" | "remove" => {
                if args.key.is_empty() || args.value.is_empty() {
                    return false;
                }
                true
            }
            _ => false,
        },
        Ok(ActionArgs::Run) => true,
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
    use std::path::PathBuf;
    use test_case::test_case;

    #[test_case("", "", "", "", false ; "No Argument is invalid")]
    #[test_case("run", "", "", "", true ; "action: run, sub_action: empty no key or value is valid")]
    #[test_case("config", "list", "", "", true ; "action: config, sub_action: list no key or value is valid")]
    #[test_case("config", "set", "", "", false ; "action: config, sub_action: set no key or value is invalid")]
    #[test_case("config", "set", "something", "", false ; "action: config, sub_action: set no value is invalid")]
    #[test_case("config", "set", "", "something", false ; "action: config, sub_action: set no key is invalid")]
    #[test_case("config", "set", "something", "something", true ; "action: config, sub_action: set with key and value is valid")]
    #[test_case("config", "remove", "", "", false ; "action: config, sub_action: remove no key or value is invalid")]
    #[test_case("config", "remove", "something", "", false ; "action: config, sub_action: remove no value is invalid")]
    #[test_case("config", "remove", "", "something", false ; "action: config, sub_action: remove no key is invalid")]
    #[test_case("config", "remove", "something", "something", true ; "action: config, sub_action: remove with key and value is valid")]
    fn test_is_args_valid(action: &str, sub_action: &str, key: &str, value: &str, is_valid: bool) {
        let cli_args = CLIArgs {
            action: String::from(action),
            sub_action: String::from(sub_action),
            key: String::from(key),
            value: String::from(value),
            import_json_path: PathBuf::new(),
        };

        assert_eq!(is_args_valid(&cli_args), is_valid);
    }
}
