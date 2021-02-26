pub use squote::{format_ident, quote, Ident, Literal, TokenStream};
pub use std::collections::{BTreeMap, BTreeSet};
pub use std::iter::FromIterator;

mod r#async;
mod gen;
mod guid;
mod hex_reader;
mod object;
mod parser;
pub mod tables;
mod to_ident;
mod to_snake;
mod type_limits;
mod type_tree;
pub mod types;
mod workspace;

pub use gen::*;
pub use guid::*;
pub use hex_reader::*;
pub use object::*;
pub use parser::*;
pub use r#async::*;
pub use to_ident::*;
pub use to_snake::*;
pub use type_limits::*;
pub use type_tree::*;
pub use workspace::*;

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
