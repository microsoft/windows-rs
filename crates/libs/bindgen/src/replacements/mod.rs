use super::*;
mod bstr;

pub fn gen(type_name: TypeName) -> Option<TokenStream> {
    match type_name {
        TypeName::BSTR => Some(bstr::gen()),
        _ => None,
    }
}
