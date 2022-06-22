/// A logically borrowed type that still retains the in-memory representation of the underlying type.
///
/// `Borrowed`s can be thought of much like an `Option<&T>`. The reason `Borrowed` must be used instead of
/// `Option<&T>` is because `Borrowed` has the same in-memory layout as `Option<T>`. This is necessary
/// for FFI calls that expect a logically borrowed type that, in-memory, looks like an owned type.
///
/// # Usage
///
/// Many functions in the `windows` crate have signatures similar to the following:
///
/// ```ignore
/// fn SomeFunction<'a, P: Into<Borrowed<'a, IUnknown>>>(iunknown: P);
/// ```
///
/// This signature allows the parameter `iunknown` to passed any value that implements `Into<Borrowed<'a, IUnknown>>`. In other
/// words, `SomeFunction` takes values of any type that can turn themselves into a borrowed `IUnknown`. Generally, if this
/// is safe to do, the `windows` crate provides an implementation. Here are the typical things that can be converted into an `Borrowed<'a, T>`:
///
/// * References to a value of type `T` (i.e., `&'a T`).
/// * Anything that can be turned into a `&'a T`.
///   * For example, many COM interfaces have such conversions to parent interfaces. For example, if a function requires an `Borrowed<'a, IUnknown>`,
///     you can pass a `&'a IInspectable` since `IInspectable` inherits from `IUnknown`.
/// * `None` - because `Borrowed`s are always optional
///
/// It's important to note that owned values **cannot** be passed as `Borrowed`s. After all, `Borrowed` types are borrowed and not owned. Just as you cannot
/// pass a `T` into a function that expects a `&T`, you cannot pass a `T` into a function that expects a `Borrowed<'a, T>`. If you get something similar to
/// the following error, it's likely because you are trying to pass an owned value instead of a borrowed value:
///
/// ```ignore
///  the trait bound `SomeType: Into<Borrowed<'_, SomeType>>` is not satisfied
/// ```
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
        unsafe { <T as super::Abi>::from_abi_ref(&self.item).ok() }
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
        // SAFETY: it is safe to alias `Borrow<'a, T>`
        unsafe { core::mem::transmute_copy(self) }
    }
}
