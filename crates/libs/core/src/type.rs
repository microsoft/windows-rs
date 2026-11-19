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
pub trait Type<C = <Self as TypeKind>::TypeKind>: TypeKind + Sized + Clone {
    type Abi;
    type Default: Clone;

    /// The parameter type used in `_Impl` trait method signatures.
    ///
    /// For `CopyType` (primitives, enums, GUID, etc.) this is `T` directly - the type itself,
    /// since these are cheap to copy and `InRef` adds no value.  For `CloneType` (HSTRING, etc.)
    /// and `InterfaceType` (COM interfaces) this is `InRef<'a, T>` - a borrowed wrapper that
    /// matches the ABI representation.
    type Ref<'a>: 'a
    where
        Self: 'a;

    fn as_ref(default: &Self::Default) -> Option<&Self>;
    fn from_default(default: &Self::Default) -> Result<Self> {
        Self::as_ref(default).cloned().ok_or(Error::empty())
    }

    /// Like [`from_default`](Self::from_default) but consumes the value, transferring ownership
    /// instead of cloning. For interface types this moves the existing reference out rather than
    /// taking an extra `AddRef`, which lets buffered iterators yield owned elements without a
    /// per-element `AddRef`/`Release` round trip.
    fn from_default_owned(default: Self::Default) -> Result<Self>;
}

impl<T> Type<InterfaceType> for T
where
    T: TypeKind<TypeKind = InterfaceType> + Clone,
{
    type Abi = *mut core::ffi::c_void;
    type Default = Option<Self>;
    type Ref<'a>
        = InRef<'a, Self>
    where
        Self: 'a;

    fn as_ref(default: &Self::Default) -> Option<&Self> {
        default.as_ref()
    }

    fn from_default_owned(default: Self::Default) -> Result<Self> {
        default.ok_or(Error::empty())
    }
}

impl<T> Type<CloneType> for T
where
    T: TypeKind<TypeKind = CloneType> + Clone,
{
    type Abi = core::mem::ManuallyDrop<Self>;
    type Default = Self;
    type Ref<'a>
        = InRef<'a, Self>
    where
        Self: 'a;

    fn as_ref(default: &Self::Default) -> Option<&Self> {
        Some(default)
    }

    fn from_default_owned(default: Self::Default) -> Result<Self> {
        Ok(default)
    }
}

impl<T> Type<CopyType> for T
where
    T: TypeKind<TypeKind = CopyType> + Clone,
{
    type Abi = Self;
    type Default = Self;
    type Ref<'a>
        = Self
    where
        Self: 'a;

    fn as_ref(default: &Self::Default) -> Option<&Self> {
        Some(default)
    }

    fn from_default_owned(default: Self) -> Result<Self> {
        Ok(default)
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

primitives!(
    bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, usize, isize
);

#[doc(hidden)]
pub type AbiType<T> = <T as Type>::Abi;

/// The parameter type for a generic type `T` used in `_Impl` trait method signatures.
///
/// Copy types resolve to `T`; strings and interfaces use a borrowed ABI-compatible wrapper.
pub type Ref<'a, T> = <T as Type>::Ref<'a>;

/// Converts a [`Ref<T>`](Ref) reference into a reference to the corresponding
/// [`Default`](Type::Default) representation.
///
/// `Ref<T>` and `T::Default` always share the same memory layout, so this is a
/// zero-cost reinterpretation:
/// - For `CopyType` (primitives, GUID, enums): `Ref<'_> = T = Default`, no-op.
/// - For `CloneType` (HSTRING, etc.) and `InterfaceType` (COM interfaces):
///   `Ref<'_> = InRef<'_, T>`, which is `#[repr(transparent)]` over `T::Abi`, and
///   `T::Abi` has the same layout as `T::Default`.
pub fn ref_as_default<'a, 'b, T: Type>(param: &'a <T as Type>::Ref<'b>) -> &'a T::Default {
    unsafe { &*(param as *const <T as Type>::Ref<'b> as *const T::Default) }
}
