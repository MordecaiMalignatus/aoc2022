use clap::Parser;
use clap::Subcommand;
use anyhow::Result;

mod day1;
mod day2;
mod util;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Clone, Subcommand)]
enum Commands {
    Day1,
    Day2
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Some(Commands::Day1) => day1::run(),
        Some(Commands::Day2) => day2::run(),
        None => day2::run(),
    }
}
