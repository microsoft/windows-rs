pub use std::collections::{BTreeMap, BTreeSet};
pub use std::iter::FromIterator;

mod r#async;
mod gen;
mod guid;
mod iterator;
mod object;
mod parser;
mod squote;
pub mod tables;
mod to_ident;
mod type_constraints;
mod type_limits;
mod type_tree;
pub mod types;
mod workspace;

pub use gen::*;
pub use guid::*;
pub use iterator::*;
pub use object::*;
pub use parser::*;
pub use r#async::*;
pub use squote::*;
pub use to_ident::*;
pub use type_constraints::*;
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
