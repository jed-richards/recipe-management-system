#[derive(Debug)]
pub enum Error {
    CommandError(CommandError),
}

// TODO: make actual error (derive_more)
#[derive(Debug)]
pub struct CommandError;
