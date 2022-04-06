mod type_def_or_ref;
mod custom_attribute_type;
mod member_ref_parent;
mod has_custom_attribute;

use super::*;
pub use type_def_or_ref::*;
pub use custom_attribute_type::*;
pub use member_ref_parent::*;
pub use has_custom_attribute::*;

pub trait Decode<'a> {
    fn decode(scope: &'a Scope, file: usize, code: usize) -> Self;
}
