use heck::ToLowerCamelCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::fields::get_fields;

pub enum MultipartType {
    Normal,
    Attach,
}

pub(crate) fn multipart_inner(input: DeriveInput) -> TokenStream {
    let Data::Struct(struct_data) = input.data else {
        panic!("current only support struct");
    };
    let struct_ident = input.ident;
    let fields = get_fields(&struct_data.fields);
    let methods = fields.iter().map(|f| {
        let fident = f.ident;
        let fident_str = fident.to_string();
        let raw_fident = fident;
        let fident = if f.is_option || f.is_vec {
            quote! {#fident}
        } else {
            quote! {this.#fident}
        };
        let normal = quote! {
            form = form.text(#fident_str, json::to_string(&#fident)?);
        };
        let normal_str = quote! {
            form = form.text(#fident_str, #fident);
        };
        let normal_multipart = quote! {
            form = form.part(#fident_str, #fident.into());
        };
        let attach_multipart = quote! {
            for part in Into::<Vec<crate::methods::FilePart>>::into(#fident) {
                match part {
                    crate::methods::FilePart::Simple(part) => form = form.part(#fident_str, part),
                    crate::methods::FilePart::Complex((name, part)) => form = form.part(name, part),
                }
            }
        };
        match (f.is_option, f.is_str) {
            (false, false) => match &f.multipart {
                Some(multipart) => match multipart {
                    MultipartType::Normal => normal_multipart,
                    MultipartType::Attach => attach_multipart,
                },
                None => normal,
            },
            (true, true) => quote! {
                if let Some(#raw_fident) = this.#raw_fident {
                    #normal_str
                }
            },
            (true, false) => match &f.multipart {
                Some(mulpart) => match mulpart {
                    MultipartType::Normal => quote! {
                        if let Some(#raw_fident) = this.#raw_fident {
                            #normal_multipart
                        }
                    },
                    MultipartType::Attach => quote! {
                        if let Some(#raw_fident) = this.#raw_fident {
                            #attach_multipart
                        }
                    },
                },
                None => quote! {
                    if let Some(#raw_fident) = this.#raw_fident {
                        #normal
                    }
                },
            },
            (false, true) => normal_str,
        }
    });
    let mut is_multi_method_content = fields
        .iter()
        .filter_map(|f| {
            let fident = f.ident;
            f.multipart.as_ref().map(|_| {
                if f.is_option {
                    quote! {
                        matches!(self.#fident, Some(super::SendFile::UploadInput { .. }))
                    }
                } else {
                    quote! {
                        matches!(self.#fident, super::SendFile::UploadInput { .. })
                    }
                }
            })
        })
        .peekable();
    let is_multi_method = if is_multi_method_content.peek().is_some() {
        quote! {
            fn is_multipart(&self) -> bool {
                #(
                    #is_multi_method_content
                )||*
            }
        }
    } else {
        quote! {}
    };
    let method_name = struct_ident.to_string().to_lower_camel_case();
    quote! {
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

        impl super::TgMultipartMethod for #struct_ident {
            fn method_name() -> String {
                #method_name.to_owned()
            }

            #is_multi_method
        }
    }
}
