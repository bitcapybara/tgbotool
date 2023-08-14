use core::panic;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse::ParseStream, parse_macro_input, Data, DeriveInput, Lit, Token};

#[derive(Debug)]
pub(crate) struct Error;

impl From<syn::Error> for Error {
    fn from(_value: syn::Error) -> Self {
        todo!()
    }
}

#[proc_macro_derive(BotCommand, attributes(command))]
pub fn bot_command(token_stream: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(token_stream);
    bot_command_inner(input).unwrap()
}

fn bot_command_inner(input: DeriveInput) -> Result<TokenStream, Error> {
    let Data::Enum(data_enum) = input.data else {
        panic!("Only enum support!");
    };
    let parse_lit = |input: ParseStream| -> Result<(Ident, String), syn::Error> {
        let key = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;
        let Lit::Str(s) = input.parse::<Lit>()? else {
            panic!("expected string attr value");
        };
        Ok((key, s.value()))
    };
    let mut command_enum = CommandEnum::default();
    for enum_attr in input.attrs {
        if !enum_attr.path().is_ident("command") {
            continue;
        }
        // #[command(bot_name = "", rename_rule = "")]
        let attrs = enum_attr
            .parse_args_with(|input: ParseStream| input.parse_terminated(parse_lit, Token![,]))?;
        for (key, value) in attrs {
            match key.to_string().as_str() {
                "bot_name" => command_enum.bot_name = Some(value),
                "rename_rule" => command_enum.rename_rule = Some(value),
                k => panic!("unsupport attr: {k}"),
            }
        }
    }
    command_enum.bot_name.get_or_insert(input.ident.to_string());

    for variant in data_enum.variants {
        let _var_ident = variant.ident;
        let mut command_variant = CommandVariant::default();
        for var_attr in variant.attrs {
            if var_attr.path().is_ident("command") {
                continue;
            }
            // #[command(rename = "")]
            let attrs = var_attr.parse_args_with(|input: ParseStream| {
                input.parse_terminated(parse_lit, Token![,])
            })?;
            for (key, value) in attrs {
                match key.to_string().as_str() {
                    "rename" => command_variant.rename = Some(value),
                    k => panic!("unsupport attr: {k}"),
                }
            }
        }
    }
    Ok(quote! {}.into())
}

#[derive(Debug, Default)]
pub(crate) struct CommandEnum {
    bot_name: Option<String>,
    rename_rule: Option<String>,
}

#[derive(Debug, Default)]
pub(crate) struct CommandVariant {
    rename: Option<String>,
}
