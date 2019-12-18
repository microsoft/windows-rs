use crate::*;

pub trait TakeOwnership {
    fn take_ownership(ptr: *const std::ffi::c_void) -> Self;
}

pub trait TypeInterface: TakeOwnership {
    fn type_guid() -> &'static Guid;
}

pub trait TypeName {
    fn type_name() -> &'static str;
}
