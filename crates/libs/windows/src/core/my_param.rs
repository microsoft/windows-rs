use std::ops::Deref;
use super::{Abi, Borrowed};

/// An in parameter for a function. 
///
/// Params can be thought of much like a `Option<&T>` where `&T` lives for as long as the function call.
/// The reason `Param` must be used instead of `Option<&T>` (and the reason why `Param` exists) is because
/// `Param` has the same in-memory layout as `T`. This is necessary for FFI calls that expect a logically
/// borrowed type that, in-memory, looks like an owned type. See the `Borrowed` type for more info.
#[repr(transparent)]
pub struct MyParam<'a, T>(Option<Borrowed<'a, T>>);

impl <'a, T> MyParam<'a, T> {
    /// Get an optional reference to the underlying value
    ///
    /// Since params are logically optional, this can return `None`.
    pub fn as_ref(&self) -> Option<&T> {
        self.0.as_ref().map(|s| s.deref())
    }
}

impl <'a, T: Abi> MyParam<'a, T> {
    /// Get the abi representation for this param
    pub fn abi(&self) -> T::Abi {
        match &self.0 {
            Some(inner) => inner.abi(),
            // SAFETY: TODO
            None => unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
        }
    }
}

impl <'a, T> From<&'a T> for MyParam<'a, T> where &'a T: Into<Borrowed<'a, T>> {
    fn from(item: &'a T) -> Self {
        MyParam(Some(item.into()))
    }
}

impl <'a, T> From<Option<&'a T>> for MyParam<'a, T> where &'a T: Into<Borrowed<'a, T>> {
    fn from(item: Option<&'a T>) -> Self {
        MyParam(item.map(|t| t.into()))
    }
}