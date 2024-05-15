use super::*;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::NonNull;

/// Provides low-level access to an interface vtable.
///
/// This trait is automatically implemented by the generated bindings and should not be
/// implemented manually.
///
/// # Safety
pub unsafe trait Interface: Sized + Clone {
    #[doc(hidden)]
    type Vtable;

    /// The `GUID` associated with the interface.
    const IID: GUID;

    #[doc(hidden)]
    const UNKNOWN: bool = true;

    /// A reference to the interface's vtable
    #[doc(hidden)]
    #[inline(always)]
    fn vtable(&self) -> &Self::Vtable {
        // SAFETY: the implementor of the trait guarantees that `Self` is castable to its vtable
        unsafe { self.assume_vtable::<Self>() }
    }

    /// Cast this interface as a reference to the supplied interfaces `Vtable`
    ///
    /// # Safety
    ///
    /// This is safe if `T` is an equivalent interface to `Self` or a super interface.
    /// In other words, `T::Vtable` must be equivalent to the beginning of `Self::Vtable`.
    #[doc(hidden)]
    #[inline(always)]
    unsafe fn assume_vtable<T: Interface>(&self) -> &T::Vtable {
        &**(self.as_raw() as *mut *mut T::Vtable)
    }

    /// Returns the raw COM interface pointer. The resulting pointer continues to be owned by the `Interface` implementation.
    #[inline(always)]
    fn as_raw(&self) -> *mut c_void {
        // SAFETY: implementors of this trait must guarantee that the implementing type has a pointer in-memory representation
        unsafe { std::mem::transmute_copy(self) }
    }

    /// Returns the raw COM interface pointer and releases ownership. It the caller's responsibility to release the COM interface pointer.
    #[inline(always)]
    fn into_raw(self) -> *mut c_void {
        // SAFETY: implementors of this trait must guarantee that the implementing type has a pointer in-memory representation
        let raw = self.as_raw();
        std::mem::forget(self);
        raw
    }

    /// Creates an `Interface` by taking ownership of the `raw` COM interface pointer.
    ///
    /// # Safety
    ///
    /// The `raw` pointer must be owned by the caller and represent a valid COM interface pointer. In other words,
    /// it must point to a vtable beginning with the `IUnknown` function pointers and match the vtable of `Interface`.
    unsafe fn from_raw(raw: *mut c_void) -> Self {
        std::mem::transmute_copy(&raw)
    }

    /// Creates an `Interface` that is valid so long as the `raw` COM interface pointer is valid.
    ///
    /// # Safety
    ///
    /// The `raw` pointer must be a valid COM interface pointer. In other words, it must point to a vtable
    /// beginning with the `IUnknown` function pointers and match the vtable of `Interface`.
    #[inline(always)]
    unsafe fn from_raw_borrowed(raw: &*mut c_void) -> Option<&Self> {
        if raw.is_null() {
            None
        } else {
            Some(std::mem::transmute_copy(&raw))
        }
    }

    /// Attempts to cast the current interface to another interface using `QueryInterface`.
    ///
    /// The name `cast` is preferred to `query` because there is a WinRT method named query but not one
    /// named cast.
    #[inline(always)]
    fn cast<T: Interface>(&self) -> Result<T> {
        // SAFETY: `result` is valid for writing an interface pointer and it is safe
        // to cast the `result` pointer as `T` on success because we are using the `IID` tied
        // to `T` which the implementor of `Interface` has guaranteed is correct
        unsafe {
            // If query() returns a failure code then we propagate that failure code to the caller.
            // In that case, we ignore the contents of 'result' (which will _not_ be dropped,
            // because MaybeUninit intentionally does not drop its contents).
            //
            // This guards against implementations of COM interfaces which may store non-null values
            // in 'result' but still return E_NOINTERFACE.
            let mut result = core::mem::MaybeUninit::<Option<T>>::zeroed();
            self.query(&T::IID, result.as_mut_ptr() as _).ok()?;

            // If we get here, then query() has succeeded, but we still need to double-check
            // that the output pointer is non-null.
            if let Some(obj) = result.assume_init() {
                Ok(obj)
            } else {
                Err(imp::E_POINTER.into())
            }
        }
    }

    /// Attempts to create a [`Weak`] reference to this object.
    fn downgrade(&self) -> Result<Weak<Self>> {
        self.cast::<imp::IWeakReferenceSource>().and_then(|source| Weak::downgrade(&source))
    }

    /// Call `QueryInterface` on this interface
    ///
    /// # Safety
    ///
    /// `interface` must be a non-null, valid pointer for writing an interface pointer.
    #[inline(always)]
    unsafe fn query(&self, iid: *const GUID, interface: *mut *mut c_void) -> HRESULT {
        if Self::UNKNOWN {
            (self.assume_vtable::<IUnknown>().QueryInterface)(self.as_raw(), iid, interface)
        } else {
            panic!("Non-COM interfaces cannot be queried.")
        }
    }

    /// Creates an `InterfaceRef` for this reference. The `InterfaceRef` tracks lifetimes statically,
    /// and eliminates the need for dynamic reference count adjustments (AddRef/Release).
    fn to_ref(&self) -> InterfaceRef<'_, Self> {
        InterfaceRef::from_interface(self)
    }
}

/// # Safety
#[doc(hidden)]
pub unsafe fn from_raw_borrowed<T: Interface>(raw: &*mut c_void) -> Option<&T> {
    T::from_raw_borrowed(raw)
}

/// This has the same memory representation as `IFoo`, but represents a borrowed interface pointer.
///
/// This type has no `Drop` impl; it does not AddRef/Release the given interface. However, because
/// it has a lifetime parameter, it always represents a non-null pointer to an interface.
#[repr(transparent)]
pub struct InterfaceRef<'a, I>(NonNull<c_void>, PhantomData<&'a I>);

impl<'a, I> Copy for InterfaceRef<'a, I> {}

impl<'a, I> Clone for InterfaceRef<'a, I> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, I: core::fmt::Debug + Interface> core::fmt::Debug for InterfaceRef<'a, I> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <I as core::fmt::Debug>::fmt(&**self, f)
    }
}

impl<'a, I: Interface> InterfaceRef<'a, I> {
    /// Creates an `InterfaceRef` from a raw pointer. _This is extremely dangerous, since there
    /// is no lifetime tracking at all!_
    ///
    /// # Safety
    /// The caller must guarantee that the `'a` lifetime parameter is bound by context to a correct
    /// lifetime.
    #[inline(always)]
    pub unsafe fn from_raw(ptr: NonNull<c_void>) -> Self {
        Self(ptr, PhantomData)
    }

    /// Creates an `InterfaceRef` from an interface reference. This safely associates the lifetime
    /// of the interface reference with the `'a` parameter of `InterfaceRef`. This allows for
    /// lifetime checking _without_ calling AddRef/Release on the underlying lifetime, which can
    /// improve efficiency.
    #[inline(always)]
    pub fn from_interface(interface: &I) -> Self {
        unsafe {
            // SAFETY: new_unchecked() should be valid because Interface::as_raw should always
            // return a non-null pointer.
            Self(NonNull::new_unchecked(interface.as_raw()), PhantomData)
        }
    }

    /// Calls AddRef on the underlying COM interface and returns an "owned" (counted) reference.
    #[inline(always)]
    pub fn to_owned(self) -> I {
        (*self).clone()
    }
}

impl<'a, 'i: 'a, I: Interface> From<&'i I> for InterfaceRef<'a, I> {
    #[inline(always)]
    fn from(interface: &'a I) -> InterfaceRef<'a, I> {
        InterfaceRef::from_interface(interface)
    }
}

impl<'a, I: Interface> core::ops::Deref for InterfaceRef<'a, I> {
    type Target = I;

    #[inline(always)]
    fn deref(&self) -> &I {
        unsafe { core::mem::transmute(self) }
    }
}
