mod builder;
mod command;
mod fields;
mod multipart;
mod parser;

use builder::builder_inner;
use command::bot_command_inner;
use multipart::multipart_inner;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(BotCommand, attributes(command))]
pub fn bot_command(token_stream: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(token_stream);
    bot_command_inner(input).into()
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder(token_stream: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(token_stream);
    builder_inner(input).into()
}

#[proc_macro_derive(Multipart, attributes(multipart))]
pub fn multipart(token_stream: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(token_stream);
    multipart_inner(input).into()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test123() {
        let token_stream = r#"
            pub struct SendMediaGroup {
                chat_id: ChatId,
                message_thread_id: Option<u64>,
                #[multipart(attach)]
                media: Vec<InputFile>,
                disable_notification: Option<bool>,
                protect_content: Option<bool>,
                reply_to_message_id: Option<u64>,
                allow_sending_without_reply: Option<bool>,
            }
        "#
        .parse()
        .unwrap();
        let input = syn::parse2(token_stream).unwrap();
        multipart_inner(input);
    }
}
