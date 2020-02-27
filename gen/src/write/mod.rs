mod generic_guard;
mod namespace;
mod rust_writer;
mod writer;

use proc_macro2::{Ident, TokenStream};
use quote::format_ident;

use crate::read::{MethodCategory, TypeDef};
use crate::signatures::MethodSig;
pub use rust_writer::RustWriter;
pub use writer::Writer;

pub fn write_ident(name: &str) -> Ident {
    if name == "Self" {
        format_ident!("{}_", name)
    } else {
        format_ident!("r#{}", name)
    }
}

#[derive(PartialEq)]
enum InterfaceCategory {
    Abi,
    Instance,
    DefaultInstance,
    Static,
    Activatable,
    DefaultActivatable,
}

struct Interface {
    definition: TypeDef,
    generics: Vec<Vec<TokenStream>>,
    overridable: bool,
    exclusive: bool,
    limited: bool, // We don't just elide from the list because we need to deal with classes who's default interface is limited.
    category: InterfaceCategory,
    identifier: TokenStream,
    // version: (u16,u16),
}

struct Method<'a> {
    name: String,
    sig: MethodSig,
    category: MethodCategory,
    interface: &'a Interface,
    limited: bool, // We don't just elide these since we still need placeholders for vtable order.
}
