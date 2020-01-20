use crate::*;

// TODO: model traits after cppwinrt traits

// Formal WinRT type categories.
pub trait BasicType {}
pub trait InterfaceType {}
pub trait DelegateType {}
pub trait EnumType {}
pub trait ClassType {}
pub trait StructType {}

// Required for interfaces and delegates
pub trait TypeGuid {
    fn type_guid() -> &'static Guid;
}

// Required for classes and in some cases interfaces
pub trait TypeName {
    fn type_name() -> &'static str;
}

// All WinRT types (usable as generic type params so not arrays).
pub trait RuntimeType {
    type Abi;

    // All WinRT types can safely be produced from an all-zero byte-pattern.
    fn empty() -> Self
    where
        Self: Sized,
    {
        // TODO: do we even need this function? Can't the local being returned be initialized with std::mem::zeroed?
        unsafe { std::mem::zeroed() }
    }

    fn as_abi(&self) -> Self::Abi;
    fn as_abi_mut(&mut self) -> *mut Self::Abi;
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

impl<T> RuntimeType for T
where
    T: RuntimeCopy,
{
    type Abi = Self;

    fn as_abi(&self) -> Self::Abi {
        *self
    }

    fn as_abi_mut(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}

// RuntimeTypes (and other types likes arrays) used as input parameter types
pub trait InParamType {
    type Abi;

    fn as_abi(&mut self) -> Self::Abi;
}

impl<T> InParamType for T
where
    T: RuntimeType,
{
    type Abi = T::Abi;

    fn as_abi(&mut self) -> Self::Abi {
        <Self as RuntimeType>::as_abi(self)
    }
}
