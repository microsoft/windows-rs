use crate::*;

pub trait TypeInterface: From<*mut std::ffi::c_void> {
    fn type_guid() -> &'static Guid;
}

pub trait TypeName {
    fn type_name() -> &'static str;
}

pub trait AsAbi {
    type In;
    type Out;
    
    fn as_abi_in(&self) -> Self::In;
    fn as_abi_out(&mut self) -> Self::Out;
}