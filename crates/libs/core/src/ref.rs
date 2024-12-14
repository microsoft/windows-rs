use super::*;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::transmute;

/// A borrowed type with the same memory layout as the type itself that can be used to construct ABI-compatible function signatures.
#[repr(transparent)]
pub struct Ref<'a, T: Type<T>>(T::Abi, PhantomData<&'a T>);

impl<T: Type<T, Default = Option<T>, Abi = *mut c_void>> Ref<'_, T> {
    /// Returns `true` if the argument is null.
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Converts the argument to a [Result<&T>] reference.
    pub fn ok(&self) -> Result<&T> {
        if self.0.is_null() {
            Err(Error::from_hresult(imp::E_POINTER))
        } else {
            unsafe { Ok(self.assume_init()) }
        }
    }

    /// Converts the argument to a `&T` reference.
    ///
    /// # Panics
    ///
    /// Panics if the argument is null.
    pub fn unwrap(&self) -> &T {
        if self.0.is_null() {
            panic!("called `Ref::unwrap` on a null value")
        } else {
            unsafe { self.assume_init() }
        }
    }

    /// Converts the argument to an [Option<T>] by cloning the reference.
    pub fn cloned(&self) -> Option<T> {
        if self.0.is_null() {
            None
        } else {
            unsafe { Some(self.assume_init().clone()) }
        }
    }

    unsafe fn assume_init(&self) -> &T {
        unsafe { transmute::<&*mut c_void, &T>(&self.0) }
    }
}

impl<T: Type<T>> core::ops::Deref for Ref<'_, T> {
    type Target = T::Default;
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(&self.0) }
    }
}
