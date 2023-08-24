mod builder;
mod command;
mod fields;
mod multipart;
mod parser;
mod tg_method;

use builder::builder_inner;
use command::bot_command_inner;
use multipart::multipart_inner;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use tg_method::tg_method_inner;

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

#[proc_macro_derive(TgMethod)]
pub fn tg_method(token_stream: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(token_stream);
    tg_method_inner(input).into()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn works() {
        let token_stream = r#"
            pub enum SendPoll {
                A {
                    a: Option<String>,
                    b: Option<u64>,
                },
                B(Option<String>, Option<u64>)
            }
        "#
        .parse()
        .unwrap();
        let input = syn::parse2(token_stream).unwrap();
        bot_command_inner(input);
    }
}
