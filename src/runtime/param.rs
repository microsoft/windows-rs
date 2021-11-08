use super::*;

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
    /// # Safety
    pub unsafe fn abi(&self) -> T::Abi {
        match self {
            Param::Borrowed(value) => core::mem::transmute_copy(*value),
            Param::Owned(value) => core::mem::transmute_copy(value),
            Param::Boxed(value) => core::mem::transmute_copy(value),
            Param::None => core::mem::zeroed(),
        }
    }
}

impl<'a, T: Abi> Drop for Param<'a, T> {
    fn drop(&mut self) {
        unsafe { T::drop_param(self) }
    }
}
