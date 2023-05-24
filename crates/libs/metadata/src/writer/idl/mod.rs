mod format;
mod parse;
mod read;
mod write;

use super::*;
pub use format::*;
pub use read::*;
pub use write::*;

pub struct IdlFile {
    references: Vec<syn::ItemUse>,
    modules: Vec<IdlModule>,
}

pub struct IdlModule {
    attributes: Vec<syn::Attribute>, // winrt/win32
    ident: syn::Ident,
    members: Vec<IdlModuleMember>,
}

pub enum IdlModuleMember {
    Module(IdlModule),
    Interface(IdlInterface),
    Struct(IdlStruct),
    Enum(IdlEnum),
    Class(IdlClass),
    // Function and Delegate
}

pub struct IdlEnum {
    item: syn::ItemEnum,
}

pub struct IdlStruct {
    item: syn::ItemStruct,
}

pub struct IdlClass {
    attributes: Vec<syn::Attribute>,
    ident: syn::Ident,
    extends: Vec<syn::Path>,
}

pub struct IdlInterface {
    attributes: Vec<syn::Attribute>,
    ident: syn::Ident,
    methods: Vec<syn::TraitItemFn>,
}
