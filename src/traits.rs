use crate::*;

pub trait TypeInterface: From<*mut std::ffi::c_void> {
    fn type_guid() -> &'static Guid;
}

pub trait TypeName {
    fn type_name() -> &'static str;
}
