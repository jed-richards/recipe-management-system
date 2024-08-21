use clap::Args;

use crate::{command::Runnable, error::CommandError};

#[derive(Args, Debug)]
pub struct GetArgs {
    id: u32,
}

// TODO: make GET /recipes/{id} request
impl Runnable<()> for GetArgs {
    /// Get Recipe by id
    fn run(&self) -> Result<(), CommandError> {
        println!("Running Get");

        Ok(())
    }
}
