use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Data, DeriveInput, GenericArgument, Type};

pub struct BuidlerField<'a> {
    ident: &'a Ident,
    ty: &'a Type,
    new: bool,
    is_str: bool,
}

pub(crate) fn builder_inner(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let Data::Struct(struct_data) = input.data else {
        panic!("current only support struct")
    };
    let struct_ident = input.ident;
    let struct_vis = input.vis;
    let mut methods = Vec::new();
    let mut fields = Vec::new();
    for field in &struct_data.fields {
        let Some(field_ident) = &field.ident else {
            panic!("field ident not found")
        };
        let Type::Path(field_ty) = &field.ty else {
            panic!("current only support path type")
        };
        let Some(segment) = &field_ty.path.segments.last() else {
            panic!("get last segment failed")
        };
        match &segment.arguments {
            syn::PathArguments::None => {
                if segment.ident == "String" {
                    fields.push(BuidlerField {
                        ident: field_ident,
                        ty: &field.ty,
                        new: true,
                        is_str: true,
                    });
                } else {
                    fields.push(BuidlerField {
                        ident: field_ident,
                        ty: &field.ty,
                        new: true,
                        is_str: false,
                    });
                }
            }
            syn::PathArguments::AngleBracketed(args) => {
                fields.push(BuidlerField {
                    ident: field_ident,
                    ty: &field.ty,
                    new: false,
                    is_str: false,
                });
                for arg in &args.args {
                    // <String>
                    let GenericArgument::Type(ty) = arg else {
                        continue;
                    };
                    let Type::Path(ty_path) = &ty else {
                        continue;
                    };
                    if segment.ident == "Option" {
                        if ty_path.path.is_ident("String") {
                            methods.push(quote! {
                                #struct_vis fn #field_ident(mut self, #field_ident: &str) -> Self {
                                    self.#field_ident = Some(#field_ident.to_owned());
                                    self
                                }
                            });
                        } else {
                            methods.push(quote! {
                               #struct_vis fn #field_ident(mut self, #field_ident: #ty) -> Self {
                                    self.#field_ident = Some(#field_ident);
                                    self
                                }
                            });
                        }
                    } else {
                        methods.push(quote! {
                            #struct_vis fn #field_ident(mut self, #field_ident: #field_ty) -> Self {
                                self.#field_ident = #field_ident;
                                self
                            }
                        });
                    }
                }
            }
            syn::PathArguments::Parenthesized(_) => panic!("unsupport parenthesized field type"),
        }
    }
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
    let new_fn_args = fields.iter().filter(|f| f.new).map(|f| {
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
        if f.new && f.is_str {
            quote! {
                #fident: #fident.to_owned()
            }
        } else if f.new {
            quote! {
                #fident
            }
        } else {
            quote! {
                #fident: Default::default()
            }
        }
    });
    Ok(quote! {
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
    .into())
}
