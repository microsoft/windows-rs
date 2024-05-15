use crate::{AsImpl, IUnknown, IUnknownImpl, Interface, InterfaceRef};
use core::ffi::c_void;
use core::mem::ManuallyDrop;
use core::ptr::NonNull;
use std::borrow::Borrow;

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
    fn get_interface(&self) -> InterfaceRef<'_, I>;
}

/// A counted pointer to a type that implements COM interfaces, where the object has been
/// placed in the heap (boxed).
///
/// This type exists so that you can place an object into the heap and query for COM interfaces,
/// without losing the safe reference to the implementation object.
pub struct ComObject<T: ComImpl> {
    ptr: NonNull<T::Impl>,
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

    // Note that we _do not_ provide a way to get a mutable reference to the outer box.
    // It's ok to return &mut T, but not &mut T::Impl. That would allow someone to replace the
    // contents of the entire object (box and reference count), which could lead to UB.
    // This could maybe be solved by returning Pin<&mut T::Impl>, but that requires some
    // additional thinking.

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
    /// reference to the heap allocation), then this method will extract the inner `T`
    /// (and return it in an `Ok`) and then free the heap allocation.
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

    /// Gets a borrowed reference to an interface that is implemented by this ComObject.
    ///
    /// The returned reference does not have an additional reference count.
    /// You can AddRef it by calling to_owned().
    pub fn as_interface<I: Interface>(&self) -> InterfaceRef<'_, I>
    where
        T::Impl: ComObjectInterface<I>,
    {
        self.get_box().get_interface()
    }

    /// Gets an owned (counted) reference to an interface that is implemented by this ComObject.
    pub fn to_interface<I: Interface>(&self) -> I
    where
        T::Impl: ComObjectInterface<I>,
    {
        self.as_interface::<I>().to_owned()
    }

    /// Converts this `ComObject` into an interface that it implements.
    pub fn into_interface<I: Interface>(self) -> I
    where
        T::Impl: ComObjectInterface<I>,
    {
        unsafe {
            let raw: *mut c_void = self.get_box().get_interface().as_raw();
            core::mem::forget(self);
            I::from_raw(raw)
        }
    }
}

impl<T: ComImpl + Default> Default for ComObject<T> {
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

impl<T: ComImpl> core::ops::Deref for ComObject<T> {
    type Target = T::Impl;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.get_box()
    }
}

// There is no DerefMut implementation because we cannot statically guarantee
// that the reference count is 1, which is a requirement for getting exclusive
// access to the contents of the object. Use get_mut() for dynamically-checked
// exclusive access.

impl<T: ComImpl> From<T> for ComObject<T> {
    fn from(value: T) -> ComObject<T> {
        ComObject::new(value)
    }
}

// Delegate hashing, if implemented.
impl<T: ComImpl + core::hash::Hash> core::hash::Hash for ComObject<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}

// If T is Send (or Sync) then the ComObject<T> is also Send (or Sync).
// Since the actual object storage is in the heap, the object is never moved.
unsafe impl<T: ComImpl + Sync> Send for ComObject<T> {}
unsafe impl<T: ComImpl + Sync> Sync for ComObject<T> {}

impl<T: ComImpl + PartialEq> PartialEq for ComObject<T> {
    fn eq(&self, other: &ComObject<T>) -> bool {
        let inner_self: &T = self.get();
        let other_self: &T = other.get();
        inner_self == other_self
    }
}

impl<T: ComImpl + Eq> Eq for ComObject<T> {}

impl<T: ComImpl + PartialOrd> PartialOrd for ComObject<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let inner_self: &T = self.get();
        let other_self: &T = other.get();
        <T as PartialOrd>::partial_cmp(inner_self, other_self)
    }
}

impl<T: ComImpl + Ord> Ord for ComObject<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let inner_self: &T = self.get();
        let other_self: &T = other.get();
        <T as Ord>::cmp(inner_self, other_self)
    }
}

impl<T: ComImpl + core::fmt::Debug> core::fmt::Debug for ComObject<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <T as core::fmt::Debug>::fmt(self.get(), f)
    }
}

impl<T: ComImpl + core::fmt::Display> core::fmt::Display for ComObject<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <T as core::fmt::Display>::fmt(self.get(), f)
    }
}

impl<T: ComImpl> Borrow<T> for ComObject<T> {
    fn borrow(&self) -> &T {
        self.get()
    }
}
