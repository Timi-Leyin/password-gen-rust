use clap::Parser;
mod constants;
mod args;
mod utils;
mod command;
fn main() {
    let args = args::Args::parse();
    command::command(&args);
}
