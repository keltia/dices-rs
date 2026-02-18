use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("invalid command {0}")]
    InvalidCommand(String),
    #[error("unknown command {0}")]
    UnknownCommand(String),
    #[error("recursion reached for {0}")]
    MaxRecursionReached(String),
}
