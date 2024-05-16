use super::*;
use core::marker::PhantomData;

/// A borrowed type with the same memory layout as the type itself that can be used to construct ABI-compatible function signatures.
#[repr(transparent)]
pub struct Ref<'a, T: Type<T>>(T::Abi, PhantomData<&'a T>);

impl<'a, T: Type<T, Default = Option<T>, Abi = *mut std::ffi::c_void>> Ref<'a, T> {
    /// Converts the argument to a [Result<&T>] reference.
    pub fn ok(&self) -> Result<&T> {
        if self.0.is_null() {
            Err(Error::from_hresult(imp::E_POINTER))
        } else {
            unsafe { Ok(std::mem::transmute::<&*mut std::ffi::c_void, &T>(&self.0)) }
        }
    }
}

impl<'a, T: Type<T>> std::ops::Deref for Ref<'a, T> {
    type Target = T::Default;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.0) }
    }
}
