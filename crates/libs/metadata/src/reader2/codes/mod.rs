mod type_def_or_ref;

use super::*;
pub use type_def_or_ref::*;

pub trait Decode<'a> {
    fn decode(scope: &'a Scope, file: usize, code: usize) -> Self;
}
