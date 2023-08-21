use heck::ToLowerCamelCase;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn tg_method_inner(input: DeriveInput) -> proc_macro2::TokenStream {
    let ident = input.ident;
    let method_name = ident.to_string().to_lower_camel_case();
    quote! {
        impl super::TgMethod for #ident {
            fn method_name() -> String {
                #method_name.to_owned()
            }
        }
    }
}
