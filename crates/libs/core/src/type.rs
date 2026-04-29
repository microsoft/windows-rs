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
    type Param<'a>: 'a
    where
        Self: 'a;

    fn is_null(abi: &Self::Abi) -> bool;
    unsafe fn assume_init_ref(abi: &Self::Abi) -> &Self;
    unsafe fn from_abi(abi: Self::Abi) -> Result<Self>;
    fn from_default(default: &Self::Default) -> Result<Self>;

    /// Converts a raw ABI parameter into the corresponding `Param` type for use
    /// in vtable upcall adapters that forward to `_Impl` methods.
    ///
    /// # Safety
    ///
    /// The ABI value must be valid for the lifetime `'a`.
    unsafe fn abi_to_param(abi: &Self::Abi) -> Self::Param<'_>;

    /// Converts a `Param` reference to a reference to the `Default` representation.
    ///
    /// This is used in generic collection implementations (e.g. `IMap_Impl`, `IVector_Impl`)
    /// to obtain a `&K::Default` from a `Generic<'_, K>` in order to perform map/vector
    /// operations that operate on the default type.
    fn param_as_default<'a>(param: &'a Self::Param<'_>) -> &'a Self::Default;
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

    unsafe fn abi_to_param(abi: &Self::Abi) -> Ref<'_, Self> {
        unsafe { core::mem::transmute_copy(abi) }
    }

    fn param_as_default<'a>(param: &'a Ref<'_, Self>) -> &'a Option<Self> {
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

    unsafe fn abi_to_param(abi: &Self::Abi) -> Ref<'_, Self> {
        unsafe { core::mem::transmute_copy(abi) }
    }

    fn param_as_default<'a>(param: &'a Ref<'_, Self>) -> &'a Self {
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

    unsafe fn abi_to_param(abi: &Self) -> Self {
        abi.clone()
    }

    fn param_as_default<'a>(param: &'a Self) -> &'a Self {
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
///     fn HasKey(&self, key: Generic<'_, i32>) -> Result<bool> {
///         // key is just i32 — no Ref wrapper needed
///         Ok(self.map.contains_key(&key))
///     }
/// }
/// ```
pub type Generic<'a, T> = <T as Type<T>>::Param<'a>;
