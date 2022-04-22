use super::*;
mod bool32;
mod bstr;
mod ntstatus;

pub fn replacements(type_name: TypeName) -> Option<TokenStream> {
    match type_name {
        TypeName::BOOL => Some(bool32::gen()),
        TypeName::BSTR => Some(bstr::gen()),
        TypeName::NTSTATUS => Some(ntstatus::gen()),
        _ => None,
    }
}
