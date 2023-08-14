use std::str::{FromStr, SplitAsciiWhitespace};

pub trait BotCommand: Sized {
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
