use core::panic;

use heck::{ToLowerCamelCase, ToPascalCase, ToSnekCase};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::ParseStream, Data, DeriveInput, GenericArgument, PathArguments, Token, Type};

use crate::parser::parse_lit_str;

pub(crate) fn bot_command_inner(input: DeriveInput) -> TokenStream {
    let Data::Enum(data_enum) = input.data else {
        panic!("Only enum support!");
    };
    // get attributes on enum
    let enum_ident = input.ident;
    let mut command_enum = CommandEnum::default();
    for enum_attr in input.attrs {
        if !enum_attr.path().is_ident("command") {
            continue;
        }
        // #[command(bot_name = "", rename_rule = "")]
        let attrs = enum_attr
            .parse_args_with(|input: ParseStream| input.parse_terminated(parse_lit_str, Token![,]))
            .unwrap();
        for (key, value) in attrs {
            match key.to_string().as_str() {
                "bot_name" => command_enum.bot_name = Some(value.value()),
                "rename_rule" => command_enum.rename_rule = Some(value.value()),
                k => panic!("unsupport attr: {k}"),
            }
        }
    }
    let rename_rule = command_enum.rename_rule;
    let bot_name = command_enum
        .bot_name
        .get_or_insert(case_conv(&enum_ident.to_string(), rename_rule.as_deref()));

    // get attributes on variants
    let mut command_names = Vec::new();
    let mut variants_parse = Vec::new();
    for variant in &data_enum.variants {
        let var_ident = &variant.ident;
        let mut command_variant = CommandVariant::default();
        for var_attr in &variant.attrs {
            if var_attr.path().is_ident("command") {
                continue;
            }
            // #[command(rename = "")]
            let attrs = var_attr
                .parse_args_with(|input: ParseStream| {
                    input.parse_terminated(parse_lit_str, Token![,])
                })
                .unwrap();
            for (key, value) in attrs {
                match key.to_string().as_str() {
                    "rename" => command_variant.rename = Some(value.value()),
                    k => panic!("unsupport attr: {k}"),
                }
            }
        }
        let name = match command_variant.rename {
            Some(name) => name,
            None => case_conv(&var_ident.to_string(), rename_rule.as_deref()),
        };
        command_names.push(format!("/{name}"));
        let mut met_option = false;
        match &variant.fields {
            syn::Fields::Named(fields) => {
                let mut fields_parse = Vec::new();
                for field in &fields.named {
                    if met_option {
                        panic!("only support option in last arg")
                    }
                    let field_ident = &field.ident;
                    let field_ty = &field.ty;
                    let option_inner_type = option_inner_type(field_ty);
                    met_option = option_inner_type.is_some();
                    let init_token = match option_inner_type {
                        Some(inner_type) => quote! {
                            #field_ident: match command::next_arg::<#inner_type>(&mut words) {
                                Ok(res) => Some(res),
                                Err(command::Error::TooFewArgs) => None,
                                Err(e) => Err(e)?
                            }
                        },
                        None => quote! {
                            #field_ident: command::next_arg::<#field_ty>(&mut words)?
                        },
                    };
                    fields_parse.push(init_token);
                }
                variants_parse.push(quote! {
                    Self::#var_ident {
                        #(#fields_parse),*
                    }
                });
            }
            syn::Fields::Unnamed(fields) => {
                let mut fields_parse = Vec::new();
                for field in &fields.unnamed {
                    let field_ty = &field.ty;
                    let option_inner_type = option_inner_type(field_ty);
                    let init_token = match option_inner_type {
                        Some(inner_type) => quote! {
                            match command::next_arg::<#inner_type>(&mut words) {
                                Ok(res) => Some(res),
                                Err(command::Error::TooFewArgs) => None,
                                Err(e) => Err(e)?
                            }
                        },
                        None => quote! {
                            command::next_arg::<#field_ty>(&mut words)?
                        },
                    };
                    fields_parse.push(init_token);
                }
                variants_parse.push(quote! {
                    Self::#var_ident(#(#fields_parse),*)
                });
            }
            syn::Fields::Unit => {
                variants_parse.push(quote! {
                    Self::#var_ident
                });
            }
        }
    }
    quote! {
        impl tgbotool::command::BotCommand for #enum_ident {
            fn parse(message: &str) -> Result<Self, tgbotool::command::Error> {
                use tgbotool::command;
                let mut words = message.split_ascii_whitespace();
                let mut command_and_bot_name = words.next().ok_or(command::Error::TooFewArgs)?.split('@');
                let command = command_and_bot_name.next().ok_or(command::Error::TooFewArgs)?;
                if let Some(cmd_bot_name) = command_and_bot_name.next() {
                    if cmd_bot_name != #bot_name {
                        return Err(command::Error::WrongBotName);
                    }
                }
                match command {
                    #(
                        #command_names => {
                            let res = #variants_parse;
                            if words.next().is_some() {
                                return Err(command::Error::TooManyArgs);
                            }
                            Ok(res)
                        }
                    )*
                    _ => Err(command::Error::UnknownCmd)
                }
            }
        }
    }
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

fn case_conv(origin: &str, rename_rule: Option<&str>) -> String {
    match rename_rule {
        Some("snake_case") => origin.to_snek_case(),
        Some("lowercase") => origin.to_lowercase(),
        Some("PascalCase") => origin.to_pascal_case(),
        Some("camalCase") => origin.to_lower_camel_case(),
        Some(_) => panic!("unsupported rename rule"),
        None => origin.to_owned(),
    }
}

fn option_inner_type(field_ty: &Type) -> Option<&Type> {
    if let Type::Path(tp) = field_ty {
        if let Some(seg) = tp.path.segments.last() {
            if seg.ident == "Option" {
                if let PathArguments::AngleBracketed(angle) = &seg.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = angle.args.last() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}
