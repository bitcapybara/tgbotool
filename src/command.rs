pub trait BotCommand: Sized {
    fn bot_name() -> String;
    fn parse(message: &str) -> Option<Self>;
}
