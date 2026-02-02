/// Custom code to free a handle.
///
/// This is similar to the [`Drop`] trait, and may be used to implement [`Drop`], but allows handles
/// to be dropped depending on context.
pub trait Free : Clone {
    /// Calls the handle's free function.
    ///
    /// # Safety
    /// The handle must be owned by the caller and safe to free.
    unsafe fn free(&mut self);
}

/// A wrapper to provide ownership for handles to automatically drop via the handle's [`Free`] trait.
#[repr(transparent)]
#[derive(PartialEq, Eq, Default, Debug)]
pub struct Owned<T: Free>(T);

impl<T: Free> Owned<T> {
    /// Takes ownership of the handle.
    ///
    /// # Safety
    ///
    /// The handle must be owned by the caller and safe to free.
    pub unsafe fn new(x: T) -> Self {
        Self(x)
    }

    /// Consumes the `Owned<T>` and relinquishes ownership of the handle.
    /// The caller is now responsible for freeing the returned handle.
    #[must_use = "losing the handle will leak it"]
    pub fn into_raw(o: Self) -> T {
        let o = core::mem::ManuallyDrop::new(o);
        o.0.clone()
    }
}

impl<T: Free> Drop for Owned<T> {
    fn drop(&mut self) {
        unsafe { self.0.free() };
    }
}

impl<T: Free> core::ops::Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Free> core::ops::DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
