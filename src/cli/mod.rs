use clap::Parser;

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

fn validate_action(args: CLIArgs) -> CLIArgs {
    match args.action.as_str() {
        "config" => match args.sub_action.as_str() {
            "list" => args,
            "set" => args,
            "remove" => args,
            _ => panic!("You need to "),
        },
        "run" => args,
        _ => panic!("Only config and run action are allowed"),
    }
}

pub fn get_cli_args() -> CLIArgs {
    let args = CLIArgs::parse();
    validate_action(args)
}
