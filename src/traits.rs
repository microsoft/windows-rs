use crate::*;

pub trait QueryType: Sized {
    fn type_guid() -> &'static Guid;

    fn query<T: QueryType>(&self) -> T {
        unsafe { std::mem::transmute_copy(&query::<T>(std::mem::transmute_copy(self))) }
    }

    fn is_empty(&self) -> bool {
        unsafe {
            let some: usize = std::mem::transmute_copy(self);
            some == 0
        }
    }
}

// Required for classes and interfaces
pub trait TypeName {
    fn type_name() -> &'static str;
}

// All WinRT types (usable as generic type params so not arrays).
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

impl<T> RuntimeType for T
where
    T: RuntimeCopy,
{
    type Abi = Self;

    fn abi(&self) -> Self::Abi {
        *self
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}
