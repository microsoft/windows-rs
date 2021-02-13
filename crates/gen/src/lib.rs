pub use squote::{format_ident, quote, Ident, Literal, TokenStream};
pub use std::collections::{BTreeSet, BTreeMap};

mod gen;
mod to_ident;
mod to_snake;
mod tables;
mod parser;
mod types;
mod guid;
mod workspace;
mod hex_reader;

pub use gen::*;
pub use to_ident::*;
pub use to_snake::*;
pub use tables::*;
pub use parser::*;
pub use types::*;
pub use guid::*;
pub use workspace::*;
pub use hex_reader::*;

// Ideally this would be defined (and used by) the nested macros crate, but this isn't yet supported by Rust.
#[macro_export]
macro_rules! unexpected {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        panic!("{}", &name[..name.len() - 3]);
    }}
}
