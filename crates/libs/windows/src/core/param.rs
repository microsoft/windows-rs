use super::{Abi, Borrowed};

/// An "IN" param to a Windows API.
///
/// In the Windows API, "IN" params are optional params that are borrowed for the lifetime of the function call.
///
/// Therefore, `InParam<'a, T>` is essentially, an optional `&'a T` (where `'a` is the lifetime of the function call)
/// that has the right in-memory layout to passed to a Windows API.
///
/// # Usage
///
/// Many functions in the `windows` crate have signatures similar to the following:
///
/// ```ignore
/// fn SomeFunction<'a, P: Into<InParam<'a, IUnknown>>>(iunknown: P);
/// ```
///
/// This signature allows the parameter `iunknown` to passed any value that implements `Into<InParam<'a, IUnknown>>`.
/// In other words, anything that can be turned into an `InParam<'a, IUnknown>` can be used as an argument to this function.
///
/// So, what can be turned into an `InParam`?
///
/// Generally, if this is safe to do, the `windows` crate provides an implementation allowing you to pass the item. This means,
/// you should be able to pass anything that is logically equivalent to an `IUnknown` into `SomeFunction`.
///
/// Here are the typical things that can be converted into an `InParam<'a, T>`:
///
/// * References to a value of type `T` (i.e., `&'a T`).
/// * Anything that can be turned into a `&'a T`.
///   * For example, many COM interfaces have such conversions to parent interfaces. For example, if a function requires an `InParam<'a, IUnknown>`,
///     you can pass a `&'a IInspectable` since `IInspectable` inherits from `IUnknown`.
/// * `None` - because `InParams`s are always optional
/// * Children classes in a WinRT hierarchy.
///   * For example, `CompositionProjectedShadowCaster` has a method called `SetBrush`. This method takes any type that implements `Into<InParam<'a, CompositionBrush>>`.
///     This allows you to pass a `&CompositionBrush` but also children classes like `CompositionColorBrush`.
///
/// # What is the reason for `InParam` existing?
///
/// `InParams`s can be thought of much kind of like an `Cow<'a, Option<T>>` - i.e., an optional value that might be owned or might be borrowed.
/// The reason `InParam` must be used instead of `Cow<'a, Option<T>>` is because for FFI calls to work, we need to be very careful about the in-memory
/// layout of the parameter. `InParam` takes care of turning the value into the proper in-memory layout that the FFI call expects.
///
/// `InParam`s are composed of either an owned type or a `Borrowed<'a, T>`. If you want to know more about how the types line up at the abi layer,
/// read the docs for `Borrowed` and for `Abi`.
pub struct InParam<'a, T: Abi> {
    inner: InParamRepr<'a, T>,
}

impl<'a, T: Abi> InParam<'a, T> {
    /// Pass a null pointer as the param
    pub fn null() -> Self {
        Self::borrowed(Borrowed::none())
    }

    /// Create an owned `InParam`
    ///
    /// Normally, it is not necessary to use this function. Generally, there is a `From` implementation
    /// that allows you to call `.into` to safely create an `InParam` value.
    pub fn owned(item: T) -> Self {
        Self { inner: InParamRepr::Owned(item) }
    }

    /// Create a borrowed `InParam`
    ///
    /// Normally, it is not necessary to use this function. Generally, there is a `From` implementation
    /// that allows you to call `.into` to safely create an `InParam` value.
    pub fn borrowed(item: Borrowed<'a, T>) -> Self {
        Self { inner: InParamRepr::Borrowed(item) }
    }

    /// Convert this `InParam` into its abi representation
    pub fn abi(&self) -> T::Abi {
        self.inner.abi()
    }
}

impl<'a, T, U: Abi> From<&'a T> for InParam<'a, U>
where
    &'a T: Into<Borrowed<'a, U>>,
{
    fn from(borrowed: &'a T) -> Self {
        InParam::borrowed(borrowed.into())
    }
}

impl<'a, T: Abi> From<Option<&'a T>> for InParam<'a, T>
where
    &'a T: Into<Borrowed<'a, T>>,
{
    fn from(item: Option<&'a T>) -> Self {
        InParam::borrowed(item.into())
    }
}

/// This allows us to represent `InParam` as an enum but keep that as an implementation detail
enum InParamRepr<'a, T: Abi> {
    Owned(T),
    Borrowed(Borrowed<'a, T>),
}

impl<'a, T: Abi> InParamRepr<'a, T> {
    fn abi(&self) -> T::Abi {
        let borrowed = match self {
            Self::Owned(item) => item.into(),
            Self::Borrowed(borrowed) => borrowed.clone(),
        };
        borrowed.abi()
    }
}

macro_rules! primitive_types {
    ($($t:ty),+) => {
        $(
            impl <'a> From<$t> for InParam<'a, $t> {
                fn from(item: $t) -> Self {
                    Self::borrowed(item.into())
                }
            }
        )*
    };
}

primitive_types!(bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, usize, isize);
