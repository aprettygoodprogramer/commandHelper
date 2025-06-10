use std::slice::GetDisjointMutError;

use clap::Parser;
mod ai;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    command_to_edit: String,
}

fn main() {
    dotenvy::dotenv().ok();
    let args = Args::parse();

    let user_text = args.command_to_edit;

    let generate_command = ai::generate_command(&user_text).unwrap();

    println!("{}", generate_command);
}
