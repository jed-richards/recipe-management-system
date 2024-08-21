mod command;
pub mod error;

use clap::Parser;

use command::{Command, Runnable};
use error::CommandError;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    cmd: Command,
}

fn main() -> Result<(), CommandError> {
    let args = Cli::parse();

    println!("ARGS:\n{:?}", args);

    match &args.cmd {
        Command::Recipe(cmd) => cmd.run(),
    }
}
