use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Data, DeriveInput, Expr, ExprPath, Lit, Meta, Token};

use crate::fields::get_fields;

#[derive(Default)]
pub struct MultipartAttr {
    field: Option<String>,
    converter: Option<ExprPath>,
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
        // #[multipart(field = "photo", converter = "part")]
        let metas = struct_attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
        for meta in metas {
            let kv = meta.require_name_value()?;
            if kv.path.is_ident("field") {
                let Expr::Lit(expr_lit) = &kv.value else {
                    panic!("field sttr value should be string")
                };
                let Lit::Str(s) = &expr_lit.lit else {
                    panic!("field sttr value should be string")
                };
                multi_attr.field = Some(s.value());
            } else if kv.path.is_ident("converter") {
                let Expr::Path(expr_path) = &kv.value else {
                    panic!("converter is path")
                };
                multi_attr.converter = Some(expr_path.clone());
            } else {
                panic!("unsupported attribute")
            }
        }
    }
    let fields = get_fields(&struct_data.fields);
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
                let Some(converter) = &multi_attr.converter else {
                    panic!("converter not specify")
                };
                if fident == field {
                    quote! {
                        form = form.part(#fident_str, this.#fident.#converter());
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
                    let Some(converter) = &multi_attr.converter else {
                            panic!("converter attr not found");
                        };
                    quote! {
                        if let Some(send_file) = this.#fident {
                            form = form.part(#fident_str, send_file.#converter());
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
