use super::*;

mod blob;
mod codes;
mod file;
mod item_index;
mod row;
mod tables;
mod type_category;
mod type_index;

pub use blob::*;
pub use codes::*;
pub use file::*;
pub use item_index::*;
pub use row::*;
pub use tables::*;
pub use type_category::*;
pub use type_index::*;

fn trim_tick(name: &str) -> &str {
    if name.as_bytes().iter().rev().nth(1) == Some(&b'`') {
        &name[..name.len() - 2]
    } else {
        name
    }
}
