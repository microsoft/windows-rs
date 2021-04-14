use crate::*;

// A WinRT method parameter used to accept either a reference or value. `Param` is used by the
// generated bindings and should not generally be used directly.
#[doc(hidden)]
pub enum Param<'a, T: Abi> {
    Borrowed(&'a T),
    Owned(T),
    Boxed(T),
    None,
}

impl<'a, T: Abi> Param<'a, T> {
    pub fn abi(&mut self) -> T::Abi {
        match self {
            Param::Borrowed(value) => value.abi(),
            Param::Owned(value) => value.abi(),
            Param::Boxed(value) => value.abi(),
            // It is always safe to form an `Abi` type's binary representation from an all-zero
            // byte-pattern as this represents the null or default state for every type.
            Param::None => unsafe { std::mem::zeroed() },
        }
    }
}

impl<'a, T: Abi> Drop for Param<'a, T> {
    fn drop(&mut self) {
        T::drop_param(self);
    }
}
