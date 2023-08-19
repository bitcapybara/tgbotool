use heck::ToLowerCamelCase;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::fields::get_fields;

pub(crate) fn tg_method_inner(input: DeriveInput) -> proc_macro2::TokenStream {
    let ident = input.ident;
    let method_name = ident.to_string().to_lower_camel_case();
    let Data::Struct(data_struct) = &input.data else {
        panic!("support struct only")
    };
    let fields = get_fields(&data_struct.fields);
    let is_multipart_method = fields
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
        .chain([quote! {false}]);
    quote! {
        impl super::TgMethod for #ident {
            fn method_name() -> String {
                #method_name.to_owned()
            }

            fn is_multipart(&self) -> bool {
                #(
                    #is_multipart_method
                )||*
            }
        }
    }
}
