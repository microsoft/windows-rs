use super::{Abi, Borrowed};
use std::ops::Deref;

/// An in parameter for a function.
///
/// `InParam`s can be thought of much like an `Option<&T>` where `&T` lives for as long as the function call
/// the parameter is being passed into. The reason `InParam` must be used instead of `Option<&T>` (and the
/// reason why `InParam` exists) is because `InParam` has the same in-memory layout as `T`. This is necessary
/// for FFI calls that expect a logically borrowed type that, in-memory, looks like an owned type. See the
/// `Borrowed` type which `InParam` is built on top of for more info.
///
/// # Usage
///
/// Many functions in the `windows` crate have signatures similar to the following:
///
/// ```rust
/// fn SomeFunction<'a, P: Into<InParam<'a, IUnknown>>>(iunknown: P);
/// ```
///
/// This signature allows the parameter `iunknown` to passed any value that implements `Into<InParam<'a, IUnknown>>`. In other
/// words, `SomeFunction` takes values of any type that can turn themselves into an `IUnknown` in parameter. Generally, if this
/// is safe to do, the `windows` crate provides an implementation. Here are the typical things that can be converted into an `InParam<'a, T>`:
///
/// * References to a value of type `T` (i.e., `&'a T`).
/// * Anything that can be turned into a `&'a T`.
///   * For example, many COM interfaces have such conversions to parent interfaces. For example, if a function requires an `InParam<'a, IUnknown>`,
///     you can pass a `&'a IInspectable` since `IInspectable` inherits from `IUnknown`.
/// * `None` - because `InParam`s are always optional
///
/// It's important to note that owned values **cannot** be passed as `InParam`s. If you get something similar to the following error, it's likely because
/// you are trying to pass an owned value instead of a borrowed value:
///
/// ```
///  the trait bound `SomeType: Into<in_param::InParam<'_, SomeType>>` is not satisfied
/// ```
#[repr(transparent)]
pub struct InParam<'a, T>(Option<Borrowed<'a, T>>);

impl<'a, T> InParam<'a, T> {
    /// Get an optional reference to the underlying value
    ///
    /// Since params are logically optional, this can return `None`.
    pub fn as_ref(&self) -> Option<&T> {
        self.0.as_ref().map(|s| s.deref())
    }
}

impl<'a, T: Abi> InParam<'a, T> {
    /// Get the abi representation for this param
    pub fn abi(&self) -> T::Abi {
        match &self.0 {
            Some(inner) => inner.abi(),
            // SAFETY: the `Abi` trait guarantees that all zeros is a valid representation
            // of its associated `Abi` type.
            None => unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        }
    }
}

impl<'a, T, U> From<&'a T> for InParam<'a, U>
where
    &'a T: Into<Borrowed<'a, U>>,
{
    fn from(item: &'a T) -> Self {
        InParam(Some(item.into()))
    }
}

impl<'a, T> From<Option<&'a T>> for InParam<'a, T>
where
    &'a T: Into<Borrowed<'a, T>>,
{
    fn from(item: Option<&'a T>) -> Self {
        InParam(item.map(|t| t.into()))
    }
}
