mod recipe;

use crate::error::CommandError;

use clap::Subcommand;

pub trait Runnable<T> {
    fn run(&self) -> Result<T, CommandError>;
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(subcommand)]
    Recipe(recipe::RecipeCommand),
}
