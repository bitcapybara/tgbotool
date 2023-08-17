use proc_macro2::Ident;
use syn::{parse::ParseStream, Lit, LitStr, Token};

pub(crate) fn parse_lit_str(input: ParseStream) -> Result<(Ident, LitStr), syn::Error> {
    let key = input.parse::<Ident>()?;
    input.parse::<Token![=]>()?;
    let Lit::Str(s) = input.parse::<Lit>()? else {
        panic!("expected a lit string attr value")
    };
    Ok((key, s))
}
