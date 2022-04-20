use super::*;
mod bool32;
mod bstr;
mod ntstatus;

pub fn gen(def: &TypeDef) -> Option<TokenStream> {
    match def.type_name() {
        TypeName::BOOL => Some(bool32::gen()),
        TypeName::BSTR => Some(bstr::gen()),
        TypeName::NTSTATUS => Some(ntstatus::gen()),
        _ => None,
    }
}
