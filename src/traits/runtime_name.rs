// Distinct from RuntimeType because only some RuntimeType's have a RuntimeName

/// A WinRT type that can be identified by a name in order to support activation.
pub trait RuntimeName {
    const NAME: &'static str;
}
