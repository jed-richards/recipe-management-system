mod create;
mod delete;
mod get;
mod list;
mod update;

use clap::Subcommand;

use crate::command::Runnable;

#[derive(Subcommand, Debug)]
pub enum RecipeCommand {
    Get(get::GetArgs),
    List(list::ListArgs),
    Create(create::CreateArgs),
    Delete(delete::DeleteArgs),
    Update(update::UpdateArgs),
}

impl Runnable<()> for RecipeCommand {
    fn run(&self) -> Result<(), crate::error::CommandError> {
        match self {
            RecipeCommand::Get(cmd) => cmd.run(),
            RecipeCommand::List(cmd) => cmd.run(),
            RecipeCommand::Create(cmd) => cmd.run(),
            RecipeCommand::Delete(cmd) => cmd.run(),
            RecipeCommand::Update(cmd) => cmd.run(),
        }
    }
}
