use crate::constants;
use clap::Parser;
#[derive(Parser, Debug)]
#[command(author=constants::AUTHOR, version=constants::VERSION, about=constants::INFO, long_about = constants::ABOUT)]
pub struct Args {
    #[arg(short, long, default_value_t = 1)]
    pub level: u16,
    #[arg(short, long, default_value_t = 8)]
    pub max: u16,
}
