/// A WinRT type that can be identified by a name in order to support activation and marshaling.
///
/// This trait is automatically used by the generated bindings and should not be
/// used directly.
pub trait RuntimeName {
    const NAME: &'static str;
}
