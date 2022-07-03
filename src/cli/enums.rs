const CONFIG_STR: &'static str = "config";
const RUN_STR: &'static str = "run";
const IMPORT_STR: &'static str = "import";

pub enum ActionArgs {
    Config,
    Run,
    Import,
}

impl ActionArgs {
    pub fn from_string(value: &str) -> Result<ActionArgs, &'static str> {
        match value {
            CONFIG_STR => Ok(ActionArgs::Config),
            RUN_STR => Ok(ActionArgs::Run),
            IMPORT_STR => Ok(ActionArgs::Import),
            _ => Err("Argument not implemented"),
        }
    }
}
