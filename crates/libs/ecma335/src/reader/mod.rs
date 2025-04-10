use super::*;

mod blob;
mod codes;
mod file;
mod index;
mod index2;
mod row;
mod tables;
mod type_category;

pub use blob::*;
pub use codes::*;
pub use file::*;
pub use index::*;
pub use index2::*;
pub use row::*;
pub use tables::*;
pub use type_category::*;

fn trim_tick(name: &str) -> &str {
    if name.as_bytes().iter().rev().nth(1) == Some(&b'`') {
        &name[..name.len() - 2]
    } else {
        name
    }
}
