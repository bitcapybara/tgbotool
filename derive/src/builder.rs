use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput};

use crate::fields::get_fields;

pub(crate) fn builder_inner(input: DeriveInput) -> TokenStream {
    let Data::Struct(struct_data) = input.data else {
        panic!("current only support struct")
    };
    let struct_ident = input.ident;
    let struct_vis = input.vis;
    let fields = get_fields(&struct_data.fields);
    let builder_ident = syn::Ident::new(&format!("{struct_ident}Builder"), Span::call_site());
    let field_init = fields.iter().map(|f| {
        let fident = f.ident;
        let ftype = f.ty;
        quote! {
            #fident: #ftype
        }
    });
    let field_build = fields.iter().map(|f| {
        let field_ident = f.ident;
        quote! {
             #field_ident: self.#field_ident
        }
    });
    let new_fn_args = fields
        .iter()
        .filter(|f| !f.is_option && f.build_value.is_none())
        .map(|f| {
            let fident = f.ident;
            let ftype = f.ty;
            if f.is_str {
                quote! {
                    #fident: &str
                }
            } else {
                quote! {
                    #fident: #ftype
                }
            }
        });
    let new_init_args = fields.iter().map(|f| {
        let fident = f.ident;
        if !f.is_option && f.is_str {
            match &f.build_value {
                Some(value) => quote! {
                    #fident: #value.to_owned()
                },
                None => quote! {
                    #fident: #fident.to_owned()
                },
            }
        } else if !f.is_option {
            quote! {
                #fident
            }
        } else {
            quote! {
                #fident: Default::default()
            }
        }
    });
    let methods = fields
        .iter()
        .filter(|f| f.is_option && !f.build_skip)
        .map(|f| {
            let fident = f.ident;
            let Some(inner_ty) = f.inner_ty else {
                return quote!();
            };
            if f.is_str {
                quote! {
                    #struct_vis fn #fident(mut self, #fident: &str) -> Self {
                        self.#fident = Some(#fident.to_owned());
                        self
                    }
                }
            } else {
                quote! {
                    #struct_vis fn #fident(mut self, #fident: #inner_ty) -> Self {
                        self.#fident = Some(#fident);
                        self
                    }
                }
            }
        });
    quote! {
        #struct_vis struct #builder_ident {
            #(
                #field_init
            ),*
        }

        impl #builder_ident {
            #struct_vis fn new(#(#new_fn_args),*) -> Self {
                Self {
                    #(
                        #new_init_args
                    ),*
                }
            }

            #(
                #methods
            )*

            #struct_vis fn build(self) -> #struct_ident {
                #struct_ident {
                    #(
                       #field_build
                    ),*
                }
            }
        }
    }
}
