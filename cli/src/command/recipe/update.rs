use clap::Args;

use crate::{command::Runnable, error::CommandError};

#[derive(Args, Debug)]
pub struct UpdateArgs {
    id: u32,
    // TODO: add optional fields for update
}

// TODO: make DELETE /recipes/{id} request
impl Runnable<()> for UpdateArgs {
    /// Delete recipe
    fn run(&self) -> Result<(), CommandError> {
        println!("Running Update");

        Ok(())
    }
}
