use super::*;
mod bool32;
mod bstr;
mod handle;
mod ntstatus;
mod pstr;
mod pwstr;

pub fn gen(def: &TypeDef) -> Option<TokenStream> {
    match def.type_name() {
        TypeName::BOOL => Some(bool32::gen()),
        TypeName::PWSTR => Some(pwstr::gen()),
        TypeName::PSTR => Some(pstr::gen()),
        TypeName::BSTR => Some(bstr::gen()),
        TypeName::NTSTATUS => Some(ntstatus::gen()),
        TypeName::HANDLE => Some(handle::gen()),
        _ => None,
    }
}
