use clap::Args;

use crate::{command::Runnable, error::CommandError};

// TODO: add fields for creating a Recipe
#[derive(Args, Debug)]
pub struct CreateArgs;

// TODO: make POST /recipes request
impl Runnable<()> for CreateArgs {
    /// Create recipe
    fn run(&self) -> Result<(), CommandError> {
        println!("Running Create");

        Ok(())
    }
}
