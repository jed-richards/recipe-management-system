use clap::Args;

use crate::{command::Runnable, error::CommandError};

#[derive(Args, Debug)]
pub struct DeleteArgs {
    id: u32,
}

// TODO: make DELETE /recipes/{id} request
impl Runnable<()> for DeleteArgs {
    /// Delete recipe
    fn run(&self) -> Result<(), CommandError> {
        println!("Running Delete");

        Ok(())
    }
}
