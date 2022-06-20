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
                if args.key.is_empty() && args.value.is_empty() {
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
