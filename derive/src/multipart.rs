use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::{fields::get_fields, parser::parse_lit_str};

#[derive(Default)]
pub struct MultipartAttr {
    field: Option<String>,
}

pub(crate) fn multipart_inner(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let Data::Struct(struct_data) = input.data else {
        panic!("current only support struct");
    };
    let struct_ident = input.ident;
    let mut multi_attr = MultipartAttr::default();
    for struct_attr in input.attrs {
        if !struct_attr.path().is_ident("multipart") {
            continue;
        }
        // #[multipart(field = "photo")]
        let kv = struct_attr.parse_args_with(parse_lit_str)?;
        if kv.0 != "field" {
            continue;
        }
        multi_attr.field = Some(kv.1.value());
    }
    let fields = get_fields(&struct_data.fields)?;
    let methods = fields.iter().map(|f| {
        let fident = f.ident;
        let fident_str = fident.to_string();
        match (f.is_option, f.is_str) {
            (false, false) => {
                let Some(field) = &multi_attr.field else {
                    return quote! {
                        form = form.text(#fident_str, json::to_string(&this.#fident)?);
                    };
                };
                if fident == field {
                    quote! {
                        form = form.part(#fident_str, this.#fident.into_part());
                    }
                } else {
                    quote! {
                        form = form.text(#fident_str, json::to_string(&this.#fident)?);
                    }
                }
            }
            (true, true) => quote! {
                if let Some(#fident) = this.#fident {
                    form = form.text(#fident_str, #fident);
                }
            },
            (true, false) => {
                let Some(field) = &multi_attr.field else {
                    return quote! {
                        if let Some(#fident) = this.#fident {
                            form = form.text(#fident_str, json::to_string(&#fident)?);
                        }
                    };
                };
                if fident == field {
                    quote! {
                        if let Some(send_file) = this.#fident {
                            form = form.part(#fident_str, send_file.into_part());
                        }
                    }
                } else {
                    quote! {
                        if let Some(#fident) = this.#fident {
                            form = form.text(#fident_str, json::to_string(&#fident)?);
                        }
                    }
                }
            }
            (false, true) => quote! {
                form = form.text(#fident_str, #fident);
            },
        }
    });
    Ok(quote! {
        impl TryFrom<#struct_ident> for reqwest::multipart::Form {
            type Error = serde_json::Error;
            fn try_from(this: #struct_ident) -> Result<Self, Self::Error> {
                use serde_json as json;
                let mut form = reqwest::multipart::Form::new();

                #(
                    #methods
                )*

                Ok(form)
            }
        }
    }
    .into())
}
