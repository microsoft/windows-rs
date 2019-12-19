use crate::*;

pub trait TypeInterface: From<*const std::ffi::c_void> {
    fn type_guid() -> &'static Guid;
}

pub trait TypeName {
    fn type_name() -> &'static str;
}

pub trait TypeAbi {
    type Abi;
    fn empty_abi() -> Self::Abi;
}

impl TypeAbi for u8 {
    type Abi = Self;
    fn empty_abi() -> Self::Abi {
        0
    }
}
