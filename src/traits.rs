use crate::*;

// WinRT classes and interfaces implement this trait. WinRT classes implement it by returning the Guid
// of the default interface. Static WinRT classes don't implement this trait. WinRT generic interfaces
// don't yet retunr the correct value from type_guid because Rust's const functions are too limited.
pub trait TypeGuid {
    fn type_guid() -> &'static Guid;
}

// Required for classes and interfaces
pub trait TypeName {
    fn type_name() -> &'static str;
}

// RuntimeType is used to constrain WinRT generic types to WinRT types
pub trait RuntimeType {
    type Abi;

    fn abi(&self) -> Self::Abi;
    fn set_abi(&mut self) -> *mut Self::Abi;
}

// Contrains blittable types to those that are WinRT types so that
// we can implement RuntimeType for all of these at once.
pub trait RuntimeCopy: Copy {}
impl RuntimeCopy for bool {}
impl RuntimeCopy for i8 {}
impl RuntimeCopy for u8 {}
impl RuntimeCopy for i16 {}
impl RuntimeCopy for u16 {}
impl RuntimeCopy for i32 {}
impl RuntimeCopy for u32 {}
impl RuntimeCopy for i64 {}
impl RuntimeCopy for u64 {}
impl RuntimeCopy for f32 {}
impl RuntimeCopy for f64 {}
impl RuntimeCopy for Guid {}

impl<T: RuntimeCopy> RuntimeType for T {
    type Abi = Self;

    fn abi(&self) -> Self::Abi {
        *self
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}
