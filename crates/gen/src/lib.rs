pub use squote::{format_ident, quote, Ident, Literal, TokenStream};
pub use std::collections::{BTreeMap, BTreeSet};
pub use std::iter::FromIterator;

mod gen;
mod guid;
mod hex_reader;
mod parser;
pub mod tables;
mod to_ident;
mod to_snake;
mod types;
mod workspace;
mod type_tree;
mod type_limits;

pub use gen::*;
pub use guid::*;
pub use hex_reader::*;
pub use parser::*;
pub use to_ident::*;
pub use to_snake::*;
pub use types::*;
pub use workspace::*;
pub use type_tree::*;
pub use type_limits::*;

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
    }};
}
