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

    /// The parameter type used in `_Impl` trait method signatures.
    ///
    /// For `CopyType` (primitives, enums, GUID, etc.) this is `T` directly — the type itself,
    /// since these are cheap to copy and `Ref` adds no value.  For `CloneType` (HSTRING, etc.)
    /// and `InterfaceType` (COM interfaces) this is `Ref<'a, T>` — a borrowed wrapper that
    /// matches the ABI representation.
    type Generic<'a>: 'a
    where
        Self: 'a;

    fn is_null(abi: &Self::Abi) -> bool;
    unsafe fn assume_init_ref(abi: &Self::Abi) -> &Self;
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self>;
    fn from_default(default: &Self::Default) -> Result<Self>;
}

impl<T> Type<T, InterfaceType> for T
where
    T: TypeKind<TypeKind = InterfaceType> + Clone,
{
    type Abi = *mut core::ffi::c_void;
    type Default = Option<Self>;
    type Generic<'a>
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
}

impl<T> Type<T, CloneType> for T
where
    T: TypeKind<TypeKind = CloneType> + Clone,
{
    type Abi = core::mem::MaybeUninit<Self>;
    type Default = Self;
    type Generic<'a>
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
}

impl<T> Type<T, CopyType> for T
where
    T: TypeKind<TypeKind = CopyType> + Clone,
{
    type Abi = Self;
    type Default = Self;
    type Generic<'a>
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

/// The parameter type for a generic type `T` used in `_Impl` trait method signatures.
///
/// For `CopyType` (primitives, enums, GUID, etc.) this resolves to `T` directly — the type
/// itself, since these are cheap to copy and no wrapper is needed.  For all other types
/// (`CloneType` such as `HSTRING`, and `InterfaceType` such as COM interfaces) this resolves
/// to [`Ref<'a, T>`], a borrowed wrapper that matches the ABI layout.
///
/// # Example
///
/// ```rust,ignore
/// impl IMap_Impl<i32, f32> for MyMap_Impl {
///     fn HasKey(&self, key: Generic<i32>) -> Result<bool> {
///         // key is just i32 — no Ref wrapper needed
///         Ok(self.map.contains_key(&key))
///     }
/// }
/// ```
pub type Generic<'a, T> = <T as Type<T>>::Generic<'a>;

/// Converts a [`Generic<T>`](Generic) reference into a reference to the corresponding
/// [`Default`](Type::Default) representation.
///
/// `Generic<T>` and `T::Default` always share the same memory layout, so this is a
/// zero-cost reinterpretation:
/// - For `CopyType` (primitives, GUID, enums): `Generic<'_> = T = Default`, no-op.
/// - For `CloneType` (HSTRING, etc.) and `InterfaceType` (COM interfaces):
///   `Generic<'_> = Ref<'_, T>`, which is `#[repr(transparent)]` over `T::Abi`, and
///   `T::Abi` has the same layout as `T::Default`.
pub fn generic_as_default<'a, 'b, T: Type<T>>(
    param: &'a <T as Type<T>>::Generic<'b>,
) -> &'a T::Default {
    unsafe { &*(param as *const <T as Type<T>>::Generic<'b> as *const T::Default) }
}
