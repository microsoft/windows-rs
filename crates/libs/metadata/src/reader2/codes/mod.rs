mod type_def_or_ref;
mod custom_attribute_type;
mod member_ref_parent;

use super::*;
pub use type_def_or_ref::*;
pub use custom_attribute_type::*;
pub use member_ref_parent::*;

pub trait Decode<'a> {
    fn decode(scope: &'a Scope, file: usize, code: usize) -> Self;
}
