use super::{Abi, Borrowed};

/// A "IN" param to a Windows API.
///
/// # Usage
///
/// Many functions in the `windows` crate have signatures similar to the following:
///
/// ```ignore
/// fn SomeFunction<'a, P: Into<Param<'a, IUnknown>>>(iunknown: P);
/// ```
///
/// This signature allows the parameter `iunknown` to passed any value that implements `Into<Param<'a, IUnknown>>`. Generally, if this
/// is safe to do, the `windows` crate provides an implementation. Here are the typical things that can be converted into an `Param<'a, T>`:
///
/// * References to a value of type `T` (i.e., `&'a T`).
/// * Anything that can be turned into a `&'a T`.
///   * For example, many COM interfaces have such conversions to parent interfaces. For example, if a function requires an `Param<'a, IUnknown>`,
///     you can pass a `&'a IInspectable` since `IInspectable` inherits from `IUnknown`.
/// * `None` - because `Params`s are always optional
/// * Children classes in a WinRT hierarchy.
///   * For example, `CompositionProjectedShadowCaster` has a method called `SetBrush`. This method takes any type that implements `Into<Param<'a, CompositionBrush>>`.
///     This allows you to pass a `&CompositionBrush` but also children classes like `CompositionColorBrush`.
///
/// # What is the reason for `Param` existing?
///
/// `Params`s can be thought of much like an `Cow<'a, Option<T>>`. The reason `Param` must be used instead of
/// `Cow<'a, Option<T>>` is because `Param` has the same in-memory layout as `Option<T>`. This is necessary
/// for FFI calls that expect a logically borrowed type that, in-memory, looks like an owned type.
///
/// `Param`s are composed of either an owned type or a `Borrowed<'a, T>`. If you want to know more about how the types line up at the abi layer, read the docs for `Borrowed`.
pub struct Param<'a, T: Abi> {
    inner: ParamRepr<'a, T>,
}

impl<'a, T: Abi> Param<'a, T> {
    /// Create an owned `Param`
    pub fn owned(item: T) -> Self {
        Self { inner: ParamRepr::Owned(item) }
    }

    /// Create an borrowed `Param`
    pub fn borrowed(item: Borrowed<'a, T>) -> Self {
        Self { inner: ParamRepr::Borrowed(item) }
    }

    /// Convert this `Param` into its abi representation
    pub fn abi(&self) -> T::Abi {
        self.inner.abi()
    }
}

impl<'a, T, U: Abi> From<&'a T> for Param<'a, U>
where
    &'a T: Into<Borrowed<'a, U>>,
{
    fn from(borrowed: &'a T) -> Self {
        Param::borrowed(borrowed.into())
    }
}

impl<'a, T: Abi> From<Option<&'a T>> for Param<'a, T>
where
    &'a T: Into<Borrowed<'a, T>>,
{
    fn from(item: Option<&'a T>) -> Self {
        Param::borrowed(item.into())
    }
}

/// This allows us to represent `Param` as an enum but keep that as an implementation detail
enum ParamRepr<'a, T: Abi> {
    Owned(T),
    Borrowed(Borrowed<'a, T>),
}

impl<'a, T: Abi> ParamRepr<'a, T> {
    fn abi(&self) -> T::Abi {
        let borrowed = match self {
            Self::Owned(item) => item.into(),
            Self::Borrowed(borrowed) => borrowed.clone(),
        };
        borrowed.abi()
    }
}
