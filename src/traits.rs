use crate::*;

pub trait TypeGuid {
    fn type_guid() -> &'static Guid;
}

pub trait TypeName {
    fn type_name() -> &'static str;
}

// pub trait CloneAs<T> {
//     fn clone_as(&self) -> T;
// }

pub trait AsAbi: Default {
    type In;
    type Out;

    fn as_abi_in(&self) -> Self::In;
    fn as_abi_out(&mut self) -> Self::Out;

    // TODO: this should probably take self by value
    fn detach_abi(self) -> Self::In {
        self.as_abi_in()
    }
}

impl AsAbi for bool {
    type In = bool;
    type Out = *mut bool;
    fn as_abi_in(&self) -> Self::In {
        *self
    }
    fn as_abi_out(&mut self) -> Self::Out {
        self as Self::Out
    }
}

impl AsAbi for char {
    // TODO: this needs to be u16
    type In = char;
    type Out = *mut char;
    fn as_abi_in(&self) -> Self::In {
        *self
    }
    fn as_abi_out(&mut self) -> Self::Out {
        self as Self::Out
    }
}

impl AsAbi for i8 {
    type In = i8;
    type Out = *mut i8;
    fn as_abi_in(&self) -> Self::In {
        *self
    }
    fn as_abi_out(&mut self) -> Self::Out {
        self as Self::Out
    }
}

impl AsAbi for u8 {
    type In = u8;
    type Out = *mut u8;
    fn as_abi_in(&self) -> Self::In {
        *self
    }
    fn as_abi_out(&mut self) -> Self::Out {
        self as Self::Out
    }
}

impl AsAbi for i16 {
    type In = i16;
    type Out = *mut i16;
    fn as_abi_in(&self) -> Self::In {
        *self
    }
    fn as_abi_out(&mut self) -> Self::Out {
        self as Self::Out
    }
}

impl AsAbi for u16 {
    type In = u16;
    type Out = *mut u16;
    fn as_abi_in(&self) -> Self::In {
        *self
    }
    fn as_abi_out(&mut self) -> Self::Out {
        self as Self::Out
    }
}

impl AsAbi for i32 {
    type In = i32;
    type Out = *mut i32;
    fn as_abi_in(&self) -> Self::In {
        *self
    }
    fn as_abi_out(&mut self) -> Self::Out {
        self as Self::Out
    }
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

impl AsAbi for i64 {
    type In = i64;
    type Out = *mut i64;
    fn as_abi_in(&self) -> Self::In {
        *self
    }
    fn as_abi_out(&mut self) -> Self::Out {
        self as Self::Out
    }
}

impl AsAbi for u64 {
    type In = u64;
    type Out = *mut u64;
    fn as_abi_in(&self) -> Self::In {
        *self
    }
    fn as_abi_out(&mut self) -> Self::Out {
        self as Self::Out
    }
}

impl AsAbi for f32 {
    type In = f32;
    type Out = *mut f32;
    fn as_abi_in(&self) -> Self::In {
        *self
    }
    fn as_abi_out(&mut self) -> Self::Out {
        self as Self::Out
    }
}

impl AsAbi for f64 {
    type In = f64;
    type Out = *mut f64;
    fn as_abi_in(&self) -> Self::In {
        *self
    }
    fn as_abi_out(&mut self) -> Self::Out {
        self as Self::Out
    }
}
