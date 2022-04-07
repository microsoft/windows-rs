mod custom_attribute_type;
mod has_custom_attribute;
mod member_ref_parent;
mod type_def_or_ref;
mod type_or_method_def;

use super::*;
pub use custom_attribute_type::*;
pub use has_custom_attribute::*;
pub use member_ref_parent::*;
pub use type_def_or_ref::*;
pub use type_or_method_def::*;

pub trait Decode<'a> {
    fn decode(scope: &'a Scope, file: usize, code: usize) -> Self;
}
