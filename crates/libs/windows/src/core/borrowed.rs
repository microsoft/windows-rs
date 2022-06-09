/// A logically borrowed type that still retains the in-memory representation of the underlying type.
///
/// `Borrowed`s can be thought of much like an `Option<&T>`. The reason `Borrowed` must be used instead of 
/// `Option<&T>` is because `Borrowed` has the same in-memory layout as `T`. This is necessary
/// for FFI calls that expect a logically borrowed type that, in-memory, looks like an owned type.
///
/// # Usage
///
/// Many functions in the `windows` crate have signatures similar to the following:
///
/// ```rust
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
/// ```
///  the trait bound `SomeType: Into<Borrowed<'_, SomeType>>` is not satisfied
/// ```
#[repr(transparent)]
pub struct Borrowed<'a, T> {
    item: Option<core::mem::ManuallyDrop<T>>,
    lifetime: core::marker::PhantomData<&'a ()>,
}

impl<'a, T: super::Abi> Borrowed<'a, T> {
    /// Create a new `Borrowed` value.
    ///
    /// Normally, it is not necessary to use this function. Generally, there is a `From` implementation
    /// that allows you to call `.into` to safely create a `Borrowed` value.
    pub fn new(item: Option<&'a T>) -> Self {
        // SAFETY: `T` is safe to alias since that is a condition of implmenting `Abi`
        let item = item.map(|i| core::mem::ManuallyDrop::new(unsafe { core::mem::transmute_copy(i) }));
        Self { item, lifetime: core::marker::PhantomData }
    }

    /// Get the abi representation for this param
    pub fn abi(&self) -> T::Abi {
        match &self.item {
            Some(inner) => inner.abi(),
            // SAFETY: the `Abi` trait guarantees that all zeros is a valid representation
            // of its associated `Abi` type.
            None => unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        }
    }
}

impl<'a, T> Borrowed<'a, T> {
    /// Get an optional reference to the underlying value
    pub fn as_ref(&self) -> Option<&T> {
        self.item.as_ref().map(|s| &**s)
    }
}

impl <'a, T: 'a , U> From<&'a U> for Borrowed<'a, T> where T: super::Abi, &'a U: Into<&'a T> {
    fn from(item: &'a U) -> Self {
        Borrowed::new(Some(item.into()))
    }
}

impl <'a, T: super::Abi> From<Option<&'a T>> for Borrowed<'a, T> where  &'a T: Into<Borrowed<'a, T>> {
    fn from(item: Option<&'a T>) -> Self {
        item.map(|i| i.into()).unwrap_or_else(|| Borrowed::new(None))
    }
}
