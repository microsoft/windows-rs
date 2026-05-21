use super::*;

mod constant;
mod field;
mod interface_impl;
mod method_def;
mod type_def;

pub use constant::*;
pub use field::*;
pub use interface_impl::*;
pub use method_def::*;
pub use type_def::*;

pub fn trim_tick(name: &str) -> &str {
    windows_metadata::trim_tick(name)
}
