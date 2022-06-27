/// A logically borrowed type that still retains the in-memory representation of the underlying type.
///
/// Note: this type is normally only used as a field to a struct. For function parameters, the `Param`
/// type, which builds on top of `Borrowed` is used.
///
/// `Borrowed`s can be thought of much like an `Option<&T>`. The reason `Borrowed` must be used instead of
/// `Option<&T>` is because `Borrowed` has the same in-memory layout as `Option<T>`. This is necessary
/// for FFI calls that expect a logically borrowed type that, in-memory, looks like an owned type.
///
/// It's important to note that owned values **cannot** be turn into `Borrowed`s. After all, `Borrowed` types are borrowed and not owned. Just as you cannot
/// turn pass a `T` into function and get a `&T` back, you cannot pass a `T` into a function that ultimately expects a `Borrowed<'a, T>`.
#[repr(transparent)]
pub struct Borrowed<'a, T: super::Abi> {
    item: T::Abi,
    lifetime: core::marker::PhantomData<&'a ()>,
}

impl<'a, T: super::Abi> Borrowed<'a, T> {
    /// Create a new `Borrowed` value.
    ///
    /// Normally, it is not necessary to use this function. Generally, there is a `From` implementation
    /// that allows you to call `.into` to safely create a `Borrowed` value.
    pub fn new(item: Option<&'a T>) -> Self {
        // SAFETY: The `Abi` trait ensures `T::Abi` is safe to zero initialize
        let item = item.map(|i| i.abi()).unwrap_or_else(|| unsafe { core::mem::MaybeUninit::zeroed().assume_init() });
        Self { item, lifetime: core::marker::PhantomData }
    }

    /// Create a new null `Borrowed` value.
    pub fn none() -> Self {
        // SAFETY: The `Abi` trait ensures `T::Abi` is safe to zero initialize
        let item = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
        Self { item, lifetime: core::marker::PhantomData }
    }

    /// Get the abi representation for this param
    ///
    /// Note: the return value is only guranteed to be valid for the lifetime of `&self`
    pub fn abi(&self) -> T::Abi {
        // SAFETY: The `Abi` trait ensures `T::Abi` is safe to memcopy
        unsafe { core::mem::transmute_copy(&self.item) }
    }

    /// Get an optional reference to the underlying value
    pub fn as_ref(&self) -> Option<&T> {
        // SAFETY: since we `item` was created from a valid `T` we can be sure that it's possible to convert back
        // for types that cannot be zero the check from `Abi::abi_is_possibly_valid` will catch that.
        unsafe { <T as super::Abi>::from_abi_ref(&self.item).ok() }
    }
}

impl<'a, T: super::Abi + Copy> Borrowed<'a, T> {
    /// Creates an `Borrowed` type from a `Copy` type meaning we don't need to borrow `item` like in `Borrowed::new`
    pub(crate) fn new_from_owned(item: T) -> Self {
        Self { item: item.abi(), lifetime: core::marker::PhantomData }
    }
}

impl<'a, T: 'a, U> From<&'a U> for Borrowed<'a, T>
where
    T: super::Abi,
    &'a U: Into<&'a T>,
{
    fn from(item: &'a U) -> Self {
        Borrowed::new(Some(item.into()))
    }
}

macro_rules! primitive_types {
    ($($t:ty),+) => {
        $(
            impl<'a> From<$t> for Borrowed<'a, $t>
            {
                fn from(item: $t) -> Self {
                    Borrowed::new_from_owned(item)
                }
            }
        )*
    };
}

primitive_types!(bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, usize, isize);

impl<'a, T: super::Abi> From<Option<&'a T>> for Borrowed<'a, T>
where
    &'a T: Into<Borrowed<'a, T>>,
{
    fn from(item: Option<&'a T>) -> Self {
        item.map(|i| i.into()).unwrap_or_else(|| Borrowed::new(None))
    }
}

impl<'a, T: super::Abi + core::fmt::Debug> core::fmt::Debug for Borrowed<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}

impl<'a, T: super::Abi> Clone for Borrowed<'a, T> {
    fn clone(&self) -> Self {
        // SAFETY: it is safe to alias `Borrow<'a, T>` for the lifetime `'a`
        unsafe { core::mem::transmute_copy(self) }
    }
}
