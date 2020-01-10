use crate::*;

pub trait TypeGuid {
    fn type_guid() -> &'static Guid;
}

pub trait TypeName {
    fn type_name() -> &'static str;
}

pub trait AsAbi: Default {
    type In;
    type Out;

    fn as_abi_in(&self) -> Self::In;
    fn as_abi_out(&mut self) -> Self::Out;
}

impl AsAbi for u32 {
    type In = u32;
    type Out = *mut u32;

    fn as_abi_in(&self) -> Self::In {
        *self
    }
    fn as_abi_out(&mut self) -> Self::Out {
        self as Self::Out
    }
}
