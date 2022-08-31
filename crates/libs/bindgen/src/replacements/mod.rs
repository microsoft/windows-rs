use super::*;
mod bstr;
mod ntstatus;

pub fn gen(type_name: TypeName) -> Option<TokenStream> {
    match type_name {
        TypeName::BSTR => Some(bstr::gen()),
        TypeName::NTSTATUS => Some(ntstatus::gen()),
        _ => None,
    }
}
