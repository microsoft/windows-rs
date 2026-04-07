use super::*;

#[doc(hidden)]
pub trait TypeKind {
    type TypeKind;
}

#[doc(hidden)]
pub struct InterfaceType;

#[doc(hidden)]
pub struct CloneType;

#[doc(hidden)]
pub struct CopyType;

#[doc(hidden)]
pub trait Type<T: TypeKind, C = <T as TypeKind>::TypeKind>: TypeKind + Sized + Clone {
    type Abi;
    type Default;
    type Param<'a>: 'a
    where
        Self: 'a;

    fn is_null(abi: &Self::Abi) -> bool;
    unsafe fn assume_init_ref(abi: &Self::Abi) -> &Self;
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self>;
    fn from_default(default: &Self::Default) -> Result<Self>;
    unsafe fn abi_to_param<'a>(abi: &'a Self::Abi) -> Self::Param<'a>
    where
        Self: 'a;
    fn param_as_default<'a, 'b>(param: &'a Self::Param<'b>) -> &'a Self::Default
    where
        Self: 'b;
}

impl<T> Type<T, InterfaceType> for T
where
    T: TypeKind<TypeKind = InterfaceType> + Clone,
{
    type Abi = *mut core::ffi::c_void;
    type Default = Option<Self>;
    type Param<'a>
        = Ref<'a, Self>
    where
        Self: 'a;

    fn is_null(abi: &Self::Abi) -> bool {
        abi.is_null()
    }

    unsafe fn assume_init_ref(abi: &Self::Abi) -> &Self {
        unsafe { core::mem::transmute::<&*mut core::ffi::c_void, &T>(abi) }
    }

    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        unsafe {
            if !abi.is_null() {
                Ok(core::mem::transmute_copy(&abi))
            } else {
                Err(Error::empty())
            }
        }
    }

    fn from_default(default: &Self::Default) -> Result<Self> {
        default.as_ref().cloned().ok_or(Error::empty())
    }

    unsafe fn abi_to_param<'a>(abi: &'a Self::Abi) -> Ref<'a, Self>
    where
        Self: 'a,
    {
        unsafe { core::mem::transmute_copy(abi) }
    }

    fn param_as_default<'a, 'b>(param: &'a Ref<'b, Self>) -> &'a Option<Self>
    where
        Self: 'b,
    {
        param
    }
}

impl<T> Type<T, CloneType> for T
where
    T: TypeKind<TypeKind = CloneType> + Clone,
{
    type Abi = core::mem::MaybeUninit<Self>;
    type Default = Self;
    type Param<'a>
        = Ref<'a, Self>
    where
        Self: 'a;

    fn is_null(_: &Self::Abi) -> bool {
        false
    }

    unsafe fn assume_init_ref(abi: &Self::Abi) -> &Self {
        unsafe { abi.assume_init_ref() }
    }

    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        unsafe { Ok(abi.assume_init()) }
    }

    fn from_default(default: &Self::Default) -> Result<Self> {
        Ok(default.clone())
    }

    unsafe fn abi_to_param<'a>(abi: &'a Self::Abi) -> Ref<'a, Self>
    where
        Self: 'a,
    {
        unsafe { core::mem::transmute_copy(abi) }
    }

    fn param_as_default<'a, 'b>(param: &'a Ref<'b, Self>) -> &'a Self
    where
        Self: 'b,
    {
        param
    }
}

impl<T> Type<T, CopyType> for T
where
    T: TypeKind<TypeKind = CopyType> + Clone,
{
    type Abi = Self;
    type Default = Self;
    type Param<'a>
        = Self
    where
        Self: 'a;

    fn is_null(_: &Self::Abi) -> bool {
        false
    }

    unsafe fn assume_init_ref(abi: &Self::Abi) -> &Self {
        abi
    }

    unsafe fn from_abi(abi: Self::Abi) -> Result<Self> {
        Ok(abi)
    }

    fn from_default(default: &Self) -> Result<Self> {
        Ok(default.clone())
    }

    unsafe fn abi_to_param<'a>(abi: &'a Self) -> Self
    where
        Self: 'a,
    {
        abi.clone()
    }

    fn param_as_default<'a, 'b>(param: &'a Self) -> &'a Self
    where
        Self: 'b,
    {
        param
    }
}

impl<T: Interface> TypeKind for T {
    type TypeKind = InterfaceType;
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

/// The parameter type used in `_Impl` trait methods for a given WinRT type.
///
/// For copy types (primitives), this is the type itself. For other types, this is
/// [`Ref`], which allows borrowing across the ABI boundary without taking ownership.
pub type ImplParam<'a, T> = <T as Type<T>>::Param<'a>;
