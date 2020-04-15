use crate::*;

pub unsafe trait ComInterface {
    const GUID: Guid;
}

/// Required for classes and interfaces
pub trait RuntimeName {
    const NAME: &'static str;
}

/// RuntimeType is used to constrain WinRT generic types to WinRT types
pub trait RuntimeType {
    type Abi;

    fn abi(&self) -> Self::Abi;
    fn set_abi(&mut self) -> *mut Self::Abi;
}

/// Contrains [blittable types](https://docs.microsoft.com/en-us/dotnet/framework/interop/blittable-and-non-blittable-types)
/// to those that are WinRT types so that we can implement RuntimeType for all of these at once.
///
/// # Safety
///
/// These types must have a stable representation meaning that they can be viewed as bytes
/// in a deterministic way. This usually means either core types (e.g.,`u8` and `bool`) or
/// types that are `#[repr(C)]`
pub unsafe trait RuntimeCopy: Copy {}
unsafe impl RuntimeCopy for bool {}
unsafe impl RuntimeCopy for i8 {}
unsafe impl RuntimeCopy for u8 {}
unsafe impl RuntimeCopy for i16 {}
unsafe impl RuntimeCopy for u16 {}
unsafe impl RuntimeCopy for i32 {}
unsafe impl RuntimeCopy for u32 {}
unsafe impl RuntimeCopy for i64 {}
unsafe impl RuntimeCopy for u64 {}
unsafe impl RuntimeCopy for f32 {}
unsafe impl RuntimeCopy for f64 {}
unsafe impl RuntimeCopy for Guid {}

impl<T: RuntimeCopy> RuntimeType for T {
    type Abi = Self;

    fn abi(&self) -> Self::Abi {
        *self
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}
