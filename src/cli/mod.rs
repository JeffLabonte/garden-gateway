use clap::Parser;

#[derive(Parser)]
struct CLIArgs {
    #[clap(short, required = true)]
    pub action: String,

    #[clap(short, long, value_parser, default_value_t = String::from(""))]
    pub key: String,

    #[clap(short, long, value_parser, default_value_t = String::from(""))]
    pub value: String,
}

pub fn get_cli_args() {
    let args = CLIArgs::parse();
    println!("{}", args.action);
    println!("{}", args.key);
    println!("{}", args.value);
}
