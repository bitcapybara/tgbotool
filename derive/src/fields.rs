use syn::{
    punctuated::Punctuated, Expr, ExprLit, Fields, GenericArgument, Ident, Lit, Meta, Token, Type,
};

use crate::multipart::MultipartType;

pub(crate) struct TgField<'a> {
    pub(crate) ident: &'a Ident,
    pub(crate) ty: &'a Type,
    pub(crate) is_str: bool,
    pub(crate) is_option: bool,
    pub(crate) inner_ty: Option<&'a Type>,
    pub(crate) build_value: Option<String>,
    pub(crate) build_skip: bool,
    pub(crate) multipart: Option<MultipartType>,
    pub(crate) is_vec: bool,
}

impl<'a> TgField<'a> {
    fn inner_ty(mut self, ty: &'a Type) -> Self {
        self.inner_ty = Some(ty);
        self
    }

    fn vec(mut self) -> Self {
        self.is_vec = true;
        self
    }

    fn option(mut self) -> Self {
        self.is_option = true;
        self
    }

    fn str(mut self) -> Self {
        self.is_str = true;
        self
    }
}

pub(crate) fn get_fields(struct_fields: &Fields) -> Vec<TgField<'_>> {
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
        let mut build_skip = false;
        let mut multipart = None;
        for attr in &field.attrs {
            // ignore #[doc("...")]
            if attr.path().is_ident("doc") {
                continue;
            }

            let metas = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();
            for meta in metas {
                match meta {
                    Meta::Path(path) => {
                        if attr.path().is_ident("builder") && path.is_ident("skip") {
                            build_skip = true;
                            continue;
                        }
                        if attr.path().is_ident("multipart") {
                            if path.is_ident("normal") {
                                multipart = Some(MultipartType::Normal);
                            } else if path.is_ident("attach") {
                                multipart = Some(MultipartType::Attach);
                            } else {
                                panic!("unsupported multipart attr")
                            }
                            continue;
                        }
                    }
                    Meta::NameValue(kv) => {
                        if attr.path().is_ident("builder") && kv.path.is_ident("value") {
                            let Expr::Lit(ExprLit {
                                lit: Lit::Str(s), ..
                            }) = kv.value
                            else {
                                panic!("unsupported builder attr")
                            };
                            build_value = Some(s.value());
                        }
                    }
                    Meta::List(_) => panic!("meta list unsupported"),
                }
            }
        }
        let Some(segment) = &field_ty_path.path.segments.last() else {
            panic!("get last segment failed")
        };
        let mut tg_field = TgField {
            ident: field_ident,
            ty: field_ty,
            is_str: false,
            is_option: false,
            inner_ty: None,
            build_value,
            build_skip,
            multipart,
            is_vec: false,
        };
        tg_field = if segment.ident == "Option" {
            tg_field.option()
        } else if segment.ident == "Vec" {
            tg_field.vec()
        } else {
            tg_field
        };
        tg_field = match &segment.arguments {
            syn::PathArguments::None => {
                if segment.ident == "String" {
                    tg_field.str()
                } else {
                    tg_field
                }
            }
            syn::PathArguments::AngleBracketed(args) => {
                let Some(GenericArgument::Type(ty)) = &args.args.last() else {
                    continue;
                };
                // <String>
                let Type::Path(ty_path) = &ty else {
                    continue;
                };
                tg_field = tg_field.inner_ty(ty);
                if ty_path.path.is_ident("String") {
                    tg_field.str()
                } else {
                    tg_field
                }
            }
            syn::PathArguments::Parenthesized(_) => panic!("unsupport parenthesized field type"),
        };
        fields.push(tg_field);
    }
    fields
}
