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

impl CLIError {
    pub fn get_key_error() -> Self {
        CLIError {
            name: String::from("key"),
            message: String::from("You cannot provide an empty key"),
        }
    }

    pub fn get_value_error() -> Self {
        CLIError {
            name: String::from("value"),
            message: String::from("You cannot provide an empty value"),
        }
    }

    pub fn get_invalid_action_error() -> Self {
        CLIError {
            name: String::from("invalid_action"),
            message: String::from("This action is not supported"),
        }
    }
}

impl CLIArgs {
    pub fn is_valid(&mut self) -> bool {
        match self.action.as_str() {
            "config" => {
                if self.key.is_empty() {
                    self.errors.push(CLIError::get_key_error());
                } else if self.value.is_empty() {
                    self.errors.push(CLIError::get_value_error());
                }
            }
            "run" => {
                println!("We will run something");
            }
            _ => self.errors.push(CLIError::get_invalid_action_error()),
        }

        self.errors.is_empty() // No Errors, this is valid
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::CLIArgs;

    #[test]
    fn cli_args_new_action_config() {
        let config_action = String::from("run");
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
