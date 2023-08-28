use std::str::{FromStr, SplitAsciiWhitespace};

pub trait BotCommand: Sized {
    fn parse(message: &str) -> Result<Self, Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Too few args")]
    TooFewArgs,
    #[error("Too many args")]
    TooManyArgs,
    #[error("Wrong bot name")]
    WrongBotName,
    #[error("Unknown command")]
    UnknownCmd,
    #[error("Parse error: {0}")]
    ParseError(String),
}

pub fn next_arg<T>(words: &mut SplitAsciiWhitespace) -> Result<T, Error>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    words
        .next()
        .ok_or(Error::TooFewArgs)?
        .parse::<T>()
        .map_err(|e| Error::ParseError(e.to_string()))
}
