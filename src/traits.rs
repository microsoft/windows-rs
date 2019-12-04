use crate::*;

pub trait TypeInterface {
    fn type_guid() -> &'static Guid;
    fn take_ownership(ptr: *const Void) -> Self;
}

pub trait TypeName {
    fn type_name() -> &'static str;
}
