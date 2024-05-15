use crate::{AsImpl, IUnknown, IUnknownImpl, Interface, Ref};
use core::mem::ManuallyDrop;
use core::ptr::NonNull;

// This is implemented on user types that are marked with #[implement].
#[allow(missing_docs)]
pub unsafe trait ComImpl {
    // The generated <foo>_Impl type (aka the "boxed" type)
    type Impl: IUnknownImpl<Impl = Self>;
}

/// Describes the COM interfaces that a specific ComObject implements.
/// This trait is implemented by ComObject implementation obejcts (e.g. `MyApp_Impl`).
pub trait ComObjectInterface<I: Interface> {
    /// Gets a borrowed interface on the ComObject.
    fn get_interface(&self) -> Ref<'_, I>;
}

/// A counted pointed to a type that implements COM interfaces, where the object has been
/// placed in the heap (boxed).
///
/// This type exists so that you can place an object into the heap and query for COM interfaces,
/// without losing the safe reference to the implementation object.
pub struct ComObject<T: ComImpl> {
    ptr: NonNull<T::Impl>,
}

impl<T: Default + ComImpl> Default for ComObject<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: ComImpl> Drop for ComObject<T> {
    fn drop(&mut self) {
        unsafe {
            T::Impl::Release(self.ptr.as_ptr());
        }
    }
}

impl<T: ComImpl> ComObject<T> {
    /// Allocates a heap cell (box) and moves `obj` into it. Returns a counted pointer to `obj`.
    pub fn new(value: T) -> Self {
        unsafe {
            let box_ = T::Impl::new_box(value);

            Self { ptr: NonNull::new_unchecked(Box::into_raw(box_)) }
        }
    }

    /// Gets a reference to the shared object stored in the box.
    ///
    /// `ComObject` also implements `Deref`, so you can often deref directly into the object.
    /// For those situations where using the `Deref` impl is inconvenient, you can use
    /// this method to explicitly get a reference to the contents.
    #[inline(always)]
    pub fn get(&self) -> &T {
        self.get_box().get_impl()
    }

    /// Gets a reference to the shared object's heap box.
    #[inline(always)]
    pub fn get_box(&self) -> &T::Impl {
        unsafe { self.ptr.as_ref() }
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

    /// If this object has only a single object reference (i.e. this `ComObject` is the only
    /// reference to the heap allocation), then this method will destroy the `ComObject` and will
    /// return the inner implementation object, wrapped in `Ok`.
    ///
    /// If there is more than one reference to this object, then this returns `Err(self)`.
    #[inline(always)]
    pub fn try_take(self) -> Result<T, Self> {
        unsafe {
            let count = self.ptr.as_ref().count();
            if count.is_one() {
                let ptr = self.ptr;
                core::mem::forget(self);
                Ok(T::Impl::extract_inner(ptr))
            } else {
                Err(self)
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

    /// Gets a reference to an interface that is implemented by this ComObject.
    ///
    /// The returned reference does not have an additional reference count.
    /// You can AddRef it by calling clone().
    pub fn borrow_interface<I: Interface>(&self) -> Ref<'_, I>
    where
        T::Impl: ComObjectInterface<I>,
    {
        self.get_box().get_interface()
    }

    /// Gets a counted reference to an interface that is implemented by this ComObject.
    pub fn get_interface<I: Interface>(&self) -> I
    where
        I: Clone,
        T::Impl: ComObjectInterface<I>,
    {
        let interface_ref: Ref<'_, I> = self.get_box().get_interface();
        let interface_inner: &I = interface_ref.ok().unwrap();
        interface_inner.clone()
    }

}

impl<T: ComImpl> Clone for ComObject<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        unsafe {
            self.ptr.as_ref().AddRef();
            Self { ptr: self.ptr }
        }
    }
}

impl<T: ComImpl> AsRef<T> for ComObject<T>
where
    IUnknown: From<T> + AsImpl<T>,
{
    #[inline(always)]
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T: ComImpl<Impl = T>> core::ops::Deref for ComObject<T> {
    type Target = T::Impl;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.get_box()
    }
}

impl<T: ComImpl> From<T> for ComObject<T> {
    fn from(value: T) -> ComObject<T> {
        ComObject::new(value)
    }
}
