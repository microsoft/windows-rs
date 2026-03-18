use super::*;

mod class_layout;
mod constant;
mod field;
mod generic_param;
mod interface_impl;
mod member_ref;
mod method_def;
mod method_param;
mod module_ref;
mod nested_class;
mod type_def;
mod type_spec;

pub use constant::*;
pub use field::*;
pub use interface_impl::*;
pub use method_def::*;
pub use type_def::*;

pub fn trim_tick(name: &str) -> &str {
    windows_metadata::trim_tick(name)
}
