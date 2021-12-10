use super::*;

/// A trait for retrieving the implementation behind a COM or WinRT interface.
///
/// This trait is automatically implemented when using the `implement` macro but
/// is considered unsafe since different implementations of the `from` interface
/// may exist.
pub trait ToImpl<T: Interface> {
    /// # Safety
    #[allow(clippy::mut_from_ref)]
    unsafe fn to_impl(from: &T) -> &mut Self;
}
