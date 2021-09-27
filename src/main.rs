use clap::{AppSettings, Clap};

mod dice;
mod roll;

/// Help message
#[derive(Debug, Clap)]
#[clap(name = "dices-rs", about = "Small dice utility.")]
#[clap(version = "0.1.0")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {}


fn main() {
    let opts = Opts::parse();

    println!("Hello, world!");
}
