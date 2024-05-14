use crate::{AsImpl, IUnknown, IUnknownImpl, Interface};
use core::mem::ManuallyDrop;
use core::ptr::NonNull;

// This is implemented on user types that are marked with #[implement].
#[allow(missing_docs)]
pub unsafe trait ComImpl {
    // The generated <foo>_Impl type (aka the "boxed" type)
    type Impl: IUnknownImpl<Impl = Self>;
}

/// A counted pointed to a type that implements COM interfaces, where the object has been
/// placed in the heap (boxed).
///
/// This type exists so that you can place an object into the heap and query for COM interfaces,
/// without losing the safe reference to the implementation object.
pub struct BoxRef<T: ComImpl> {
    ptr: NonNull<T::Impl>,
}

impl<T: Default + ComImpl> Default for BoxRef<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: ComImpl> Drop for BoxRef<T> {
    fn drop(&mut self) {
        unsafe {
            T::Impl::Release(self.ptr.as_ptr());
        }
    }
}

impl<T: ComImpl> BoxRef<T> {
    /// Allocates a heap cell (box) and moves `obj` into it. Returns a counted pointer to `obj`.
    pub fn new(value: T) -> Self {
        unsafe {
            let box_ = T::Impl::new_box(value);

            Self { ptr: NonNull::new_unchecked(Box::into_raw(box_)) }
        }
    }

    /// Gets a reference to the shared object stored in the box.
    ///
    /// `BoxRef` also implements `Deref`, so you can often deref directly into the object.
    /// For those situations where using the `Deref` impl is inconvenient, you can use
    /// this method to explicitly get a reference to the contents.
    #[inline(always)]
    pub fn get(&self) -> &T {
        unsafe { self.ptr.as_ref().get_impl() }
    }

    /// Gets a mutable reference to the object stored in the box, if the reference count
    /// is exactly 1. If there are multiple references to this object then this returns `None`.
    #[inline(always)]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        unsafe {
            let count = self.ptr.as_ref().count();
            if count.is_one() {
                Some(self.ptr.as_mut().get_impl_mut())
            } else {
                None
            }
        }
    }

    /// Casts to a given interface type.
    #[inline(always)]
    pub fn cast<J: Interface>(&self) -> windows_core::Result<J> {
        unsafe {
            let unknown_ptr = self.ptr.as_ref().iunknown_ptr();
            let unknown: ManuallyDrop<IUnknown> = core::mem::transmute(unknown_ptr);
            unknown.cast()
        }
    }
}

impl<T: ComImpl> Clone for BoxRef<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        unsafe {
            self.ptr.as_ref().AddRef();
            Self { ptr: self.ptr }
        }
    }
}

impl<T: ComImpl> AsRef<T> for BoxRef<T>
where
    IUnknown: From<T> + AsImpl<T>,
{
    #[inline(always)]
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T: ComImpl> core::ops::Deref for BoxRef<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}
