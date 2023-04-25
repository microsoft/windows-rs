/// A trait for retrieving the implementation behind a COM or WinRT interface.
///
/// This trait is automatically implemented when using the `implement` macro.
pub trait AsImpl<T> {
    fn as_impl(&self) -> &T;
}
