use clap::Parser;

#[derive(Parser)]
struct CLIArgs {
    pub action: String,
    pub key: String,
    pub value: String,
}

pub fn get_cli_args() {
    let args = CLIArgs::parse();
    println!("{}", args.action);
    println!("{}", args.key);
    println!("{}", args.value);
}
