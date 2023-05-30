mod format;
mod parse;
mod read;
mod write;

use super::*;
pub use format::*;
pub use read::*;
pub use write::*;

// The value of the IDL-specific memory representation is that it allows for constructs that are not modeled in the abstract Module
// tree such as the use declarations and if we get rid of it we'd always "format" IDL by stripping out any of that into a single
// canonical form which would not be very friendly to developers.
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
