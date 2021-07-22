use super::*;

/// A trait for retrieving the implementation behind a COM or WinRT interface.
///
/// This trait is automatically implemented when using the [`implement`] macro but
/// is considered unsafe since different implemenetations of the `from` interface
// may exist.
pub unsafe trait ToImpl<T: Interface> {
    fn to_impl(from: &T) -> &mut Self;
}
