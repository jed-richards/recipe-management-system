use clap::Args;

use crate::{command::Runnable, error::CommandError};

#[derive(Args, Debug)]
pub struct ListArgs;

// TODO: make GET /recipes request
impl Runnable<()> for ListArgs {
    /// List recipes
    fn run(&self) -> Result<(), CommandError> {
        println!("Running List");

        Ok(())
    }
}
