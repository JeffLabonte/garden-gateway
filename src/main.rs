use gateway::{
    cli::{actions::run_action, get_cli_args},
    context::Context,
};

fn main() {
    let context = Context {
        arguments: get_cli_args(),
    };

    if !run_action(context) {
        eprintln!("Something went wrong");
    }
}
