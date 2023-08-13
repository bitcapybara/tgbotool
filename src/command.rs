pub trait BotCommand: Sized {
    fn bot_name() -> String;
    fn parse(message: &str) -> Result<Self, Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("")]
    TooFewArgs,
    #[error("")]
    TooManyArgs,
    #[error("")]
    WrongBotName,
    #[error("")]
    UnknownCmd,
    #[error("")]
    ParseError(String),
}
