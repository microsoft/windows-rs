mod attribute_type;
mod has_attribute;
mod has_constant;
mod member_ref_parent;
mod type_def_or_ref;
mod type_or_method_def;

use super::*;
pub use attribute_type::*;
pub use has_attribute::*;
pub use has_constant::*;
pub use member_ref_parent::*;
pub use type_def_or_ref::*;
pub use type_or_method_def::*;

pub trait Decode {
    fn decode(file: usize, code: usize) -> Self;
}
