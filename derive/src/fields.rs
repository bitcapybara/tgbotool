use proc_macro2::Ident;
use syn::{Fields, GenericArgument, Type};

use crate::parser::parse_lit_str;

pub(crate) struct TgField<'a> {
    pub(crate) ident: &'a Ident,
    pub(crate) ty: &'a Type,
    pub(crate) is_str: bool,
    pub(crate) is_option: bool,
    pub(crate) inner_ty: Option<&'a Type>,
    pub(crate) build_value: Option<String>,
}

impl<'a> TgField<'a> {
    fn inner_ty(mut self, ty: &'a Type) -> Self {
        self.inner_ty = Some(ty);
        self
    }
}

pub(crate) fn get_fields(struct_fields: &Fields) -> Result<Vec<TgField<'_>>, syn::Error> {
    let mut fields = Vec::new();
    for field in struct_fields {
        let Some(field_ident) = &field.ident else {
            panic!("field ident not found")
        };
        let field_ty = &field.ty;
        let Type::Path(field_ty_path) = &field.ty else {
            panic!("current only support path type")
        };
        let mut build_value = None;
        for attr in &field.attrs {
            if !attr.path().is_ident("builder") {
                continue;
            }
            let (ident, value) = attr.parse_args_with(parse_lit_str)?;
            if ident != "value" {
                panic!("builder need value key")
            }
            build_value = Some(value.value());
        }
        let Some(segment) = &field_ty_path.path.segments.last() else {
            panic!("get last segment failed")
        };
        let tg_field = |is_option: bool, is_str: bool| -> TgField {
            TgField {
                ident: field_ident,
                ty: field_ty,
                is_str,
                is_option,
                inner_ty: None,
                build_value,
            }
        };
        match &segment.arguments {
            syn::PathArguments::None => {
                if segment.ident == "String" {
                    fields.push(tg_field(false, true));
                } else {
                    fields.push(tg_field(false, false));
                }
            }
            syn::PathArguments::AngleBracketed(args) => {
                if let Some(GenericArgument::Type(ty)) = &args.args.last() {
                    // <String>
                    if let Type::Path(ty_path) = &ty {
                        let f = if segment.ident == "Option" {
                            if ty_path.path.is_ident("String") {
                                tg_field(true, true)
                            } else {
                                tg_field(true, false)
                            }
                        } else {
                            tg_field(false, false)
                        }
                        .inner_ty(ty);
                        fields.push(f);
                    }
                }
            }
            syn::PathArguments::Parenthesized(_) => panic!("unsupport parenthesized field type"),
        }
    }
    Ok(fields)
}
