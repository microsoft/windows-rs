use crate::*;

pub trait QueryType: Sized {
    fn type_guid() -> &'static Guid;

    fn query<T: QueryType>(&self) -> T {
        unsafe { std::mem::transmute_copy(&query::<T>(std::mem::transmute_copy(self))) }
    }
}

// Formal WinRT type categories.
pub trait BasicType {}
pub trait EnumType {}
pub trait StructType {}
pub trait InterfaceType {}
pub trait DelegateType {}
pub trait ClassType {}

// Required for classes and in some cases interfaces
pub trait TypeName {
    fn type_name() -> &'static str;
}

pub trait Signature {
    fn signature() -> &'static str;
}

// All WinRT types (usable as generic type params so not arrays).
pub trait RuntimeType {
    type Abi;

    // // All WinRT types can safely be produced from an all-zero byte-pattern.
    // fn empty() -> Self
    // where
    //     Self: Sized,
    // {
    //     // TODO: do we even need this function? Can't the local being returned be initialized with std::mem::zeroed?
    //     unsafe { std::mem::zeroed() }
    // }

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

impl TypeName for bool {
    fn type_name() -> &'static str {
        "Boolean"
    }
}

impl TypeName for i8 {
    fn type_name() -> &'static str {
        "Int8"
    }
}

impl TypeName for u8 {
    fn type_name() -> &'static str {
        "UInt8"
    }
}

impl TypeName for i16 {
    fn type_name() -> &'static str {
        "Int16"
    }
}

impl TypeName for u16 {
    fn type_name() -> &'static str {
        "UInt16"
    }
}

impl TypeName for i32 {
    fn type_name() -> &'static str {
        "Int32"
    }
}

impl TypeName for u32 {
    fn type_name() -> &'static str {
        "UInt32"
    }
}

impl TypeName for i64 {
    fn type_name() -> &'static str {
        "Int64"
    }
}

impl TypeName for u64 {
    fn type_name() -> &'static str {
        "UInt64"
    }
}

impl TypeName for f32 {
    fn type_name() -> &'static str {
        "Single"
    }
}

impl TypeName for f64 {
    fn type_name() -> &'static str {
        "Double"
    }
}

impl TypeName for Guid {
    fn type_name() -> &'static str {
        "Guid"
    }
}

impl TypeName for String {
    fn type_name() -> &'static str {
        "String"
    }
}

impl TypeName for Object {
    fn type_name() -> &'static str {
        "Object"
    }
}

impl Signature for bool {
    fn signature() -> &'static str {
        "b1"
    }
}

impl Signature for i8 {
    fn signature() -> &'static str {
        "i1"
    }
}

impl Signature for u8 {
    fn signature() -> &'static str {
        "u1"
    }
}

impl Signature for i16 {
    fn signature() -> &'static str {
        "i2"
    }
}

impl Signature for u16 {
    fn signature() -> &'static str {
        "u2"
    }
}

impl Signature for i32 {
    fn signature() -> &'static str {
        "i4"
    }
}

impl Signature for u32 {
    fn signature() -> &'static str {
        "u4"
    }
}

impl Signature for i64 {
    fn signature() -> &'static str {
        "i8"
    }
}

impl Signature for u64 {
    fn signature() -> &'static str {
        "u8"
    }
}

impl Signature for f32 {
    fn signature() -> &'static str {
        "f4"
    }
}

impl Signature for f64 {
    fn signature() -> &'static str {
        "f8"
    }
}

impl Signature for Guid {
    fn signature() -> &'static str {
        "g16"
    }
}

impl Signature for String {
    fn signature() -> &'static str {
        "string"
    }
}

impl Signature for Object {
    fn signature() -> &'static str {
        "cinterface(IInspectable)"
    }
}
