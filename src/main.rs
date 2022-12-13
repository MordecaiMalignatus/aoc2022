use clap::Parser;
use clap::Subcommand;
use anyhow::Result;

mod day1;
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
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Some(Commands::Day1) => day1::run(),
        None => day1::run(),
    }
}
