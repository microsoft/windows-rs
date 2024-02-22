use super::*;

#[doc(hidden)]
pub trait TypeKind {
    type TypeKind;
}

#[doc(hidden)]
pub struct ReferenceType;

#[doc(hidden)]
pub struct ValueType;

#[doc(hidden)]
pub struct CopyType;

#[doc(hidden)]
pub trait Type<T: TypeKind, C = <T as TypeKind>::TypeKind>: TypeKind + Sized + Clone {
    type Abi;
    type Default;

    /// # Safety
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self>;
    fn from_default(default: &Self::Default) -> Result<Self>;
}

impl<T> Type<T, ReferenceType> for T
where
    T: TypeKind<TypeKind = ReferenceType> + Clone,
{
    type Abi = *mut std::ffi::c_void;
    type Default = Option<Self>;

    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        if !abi.is_null() {
            Ok(std::mem::transmute_copy(&abi))
        } else {
            Err(Error::empty())
        }
    }

    fn from_default(default: &Self::Default) -> Result<Self> {
        default.as_ref().cloned().ok_or(Error::empty())
    }
}

impl<T> Type<T, ValueType> for T
where
    T: TypeKind<TypeKind = ValueType> + Clone,
{
    type Abi = std::mem::MaybeUninit<Self>;
    type Default = Self;

    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        Ok(abi.assume_init())
    }

    fn from_default(default: &Self::Default) -> Result<Self> {
        Ok(default.clone())
    }
}

impl<T> Type<T, CopyType> for T
where
    T: TypeKind<TypeKind = CopyType> + Clone,
{
    type Abi = Self;
    type Default = Self;

    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        Ok(abi)
    }

    fn from_default(default: &Self) -> Result<Self> {
        Ok(default.clone())
    }
}

impl<T: Interface> TypeKind for T {
    type TypeKind = ReferenceType;
}

impl<T> TypeKind for *mut T {
    type TypeKind = CopyType;
}

macro_rules! primitives {
    ($($t:ty),+) => {
        $(
            impl TypeKind for $t {
                type TypeKind = CopyType;
            }
        )*
    };
}

primitives!(bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, usize, isize);

#[doc(hidden)]
pub type AbiType<T> = <T as Type<T>>::Abi;

impl TypeKind for PWSTR {
    type TypeKind = CopyType;
}

impl TypeKind for PSTR {
    type TypeKind = CopyType;
}

impl TypeKind for PCWSTR {
    type TypeKind = CopyType;
}

impl TypeKind for PCSTR {
    type TypeKind = CopyType;
}

impl TypeKind for HSTRING {
    type TypeKind = ValueType;
}

impl TypeKind for BSTR {
    type TypeKind = ValueType;
}
