use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("invalid command {0}")]
    InvalidCommand(String),
    #[error("unknown command {0}")]
    UnknownCommand(String),
    #[error("parsing diceset in {0}")]
    ParsingDiceset(String),
    #[error("only builtins are executable")]
    OnlyBuiltins,
}
