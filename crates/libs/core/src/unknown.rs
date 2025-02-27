use crate::imp::WeakRefCount;

use super::*;
use core::ffi::c_void;
use core::ptr::NonNull;

/// Base interface for all COM interfaces.
///
/// All COM interfaces (and thus WinRT classes and interfaces) implement
/// [IUnknown](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// under the hood to provide reference-counted lifetime management as well as the ability
/// to query for additional interfaces that the object may implement.
#[repr(transparent)]
pub struct IUnknown(NonNull<c_void>);

#[doc(hidden)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct IUnknown_Vtbl {
    pub QueryInterface: unsafe extern "system" fn(
        this: *mut c_void,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut c_void) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut c_void) -> u32,
}

unsafe impl Interface for IUnknown {
    type Vtable = IUnknown_Vtbl;
    const IID: GUID = GUID::from_u128(0x00000000_0000_0000_c000_000000000046);
}

impl Clone for IUnknown {
    fn clone(&self) -> Self {
        unsafe {
            (self.vtable().AddRef)(core::mem::transmute_copy(self));
        }

        Self(self.0)
    }
}

impl Drop for IUnknown {
    fn drop(&mut self) {
        unsafe {
            (self.vtable().Release)(core::mem::transmute_copy(self));
        }
    }
}

impl PartialEq for IUnknown {
    fn eq(&self, other: &Self) -> bool {
        // First we test for ordinary pointer equality. If two COM interface pointers have the
        // same pointer value, then they are the same object. This can save us a lot of time,
        // since calling QueryInterface is much more expensive than a single pointer comparison.
        //
        // However, interface pointers may have different values and yet point to the same object.
        // Since COM objects may implement multiple interfaces, COM identity can only
        // be determined by querying for `IUnknown` explicitly and then comparing the
        // pointer values. This works since `QueryInterface` is required to return
        // the same pointer value for queries for `IUnknown`.
        self.as_raw() == other.as_raw()
            || self.cast::<IUnknown>().unwrap().0 == other.cast::<IUnknown>().unwrap().0
    }
}

impl Eq for IUnknown {}

impl core::fmt::Debug for IUnknown {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IUnknown").field(&self.as_raw()).finish()
    }
}

/// The `#[implement]` macro generates implementations of this trait for the types
/// that it generates, e.g. `MyApp_Impl`,
///
/// `ComObject` uses this trait to interact with boxed COM objects.
#[doc(hidden)]
pub trait IUnknownImpl {
    /// The contained user type, e.g. `MyApp`. Also known as the "inner" type.
    type Impl;

    /// Get a reference to the backing implementation.
    fn get_impl(&self) -> &Self::Impl;

    /// Get a mutable reference to the contained (inner) object.
    fn get_impl_mut(&mut self) -> &mut Self::Impl;

    /// Consumes the box and returns the contained (inner) object. This is the opposite of `new_box`.
    fn into_inner(self) -> Self::Impl;

    /// Reference to the identity interface of this COM object. For aggregated objects, this
    /// points to the identity defined on the root (non-aggregated) object.
    fn identity_interface(&self) -> &&'static IInspectable_Vtbl;

    /// The classic `QueryInterface` method from COM.
    ///
    /// # Safety
    ///
    /// This function is safe to call as long as the interface pointer is non-null and valid for writes
    /// of an interface pointer.

    /// the real implementation for this specific type
    unsafe fn QueryInterface(&self, iid: *const GUID, interface: *mut *mut c_void) -> HRESULT;

    /// Gets access to the internal reference count field.
    ///
    /// # Safety
    ///
    /// Because `WeakRefCount` allows the caller to increase or decrease the reference count within
    /// safe code, and because that changes the lifetime of memory owned by the implementation of
    /// `IUnknownImpl`, we cannot permit a `&WeakRefCount` to be visible to safe code. This is why
    /// this function has an `unsafe` signature.
    ///
    /// All callers of this function must ensure that changes to the ref count are done safely.
    fn count_field(&self) -> &WeakRefCount {
        &self.header().count
    }

    /// Gets a reference to the header of this COM object.
    fn header(&self) -> &ComObjectHeader;

    /// Increments the reference count of the interface
    #[inline(always)]
    unsafe fn add_ref(&self) -> u32 {
        unsafe { self.count_field().add_ref() }
    }

    /// Decrements the reference count causing the interface's memory to be freed when the count is 0
    ///
    /// # Safety
    ///
    /// This function should only be called when the interface pointer is no longer used as calling `Release`
    /// on a non-aliased interface pointer and then using that interface pointer may result in use after free.
    ///
    /// This function takes `*mut Self` because the object may be freed by the time this method returns.
    /// Taking `&self` would violate Rust's rules on reference lifetime.
    #[inline(always)]
    unsafe fn release_ref(self_: *mut Self) -> u32 {
        unsafe {
            let remaining = (*self_).count_field().release();
            if remaining > 0 {
                return remaining;
            }

            Self::destroy(self_);
            0
        }
    }

    /// Destroys a COM object, after all references to it have been released.
    ///
    /// This uses the equivalent of a virtual destructor in order to run the correct
    /// drop handlers and free the allocation with the correct size/alignment.
    #[inline(never)]
    fn destroy(self_: *mut Self) {
        unsafe {
            let destructor = (*self_).header().destructor;
            destructor(self_ as *mut c_void);
        }
    }

    /// Returns `true` if the reference count of the box is equal to 1.
    #[inline(always)]
    fn is_reference_count_one(&self) -> bool {
        self.count_field().is_one()
    }

    /// Gets the trust level of the current object.
    unsafe fn GetTrustLevel(&self, value: *mut i32) -> HRESULT;

    /// Gets a borrowed reference to an interface that is implemented by this ComObject.
    ///
    /// The returned reference does not have an additional reference count.
    /// You can AddRef it by calling to_owned().
    #[inline(always)]
    fn as_interface<I: Interface>(&self) -> InterfaceRef<'_, I>
    where
        Self: ComObjectInterface<I>,
    {
        <Self as ComObjectInterface<I>>::as_interface_ref(self)
    }

    /// Gets an owned (counted) reference to an interface that is implemented by this ComObject.
    #[inline(always)]
    fn to_interface<I: Interface>(&self) -> I
    where
        Self: ComObjectInterface<I>,
    {
        <Self as ComObjectInterface<I>>::as_interface_ref(self).to_owned()
    }

    /// Creates a new owned reference to this object.
    ///
    /// # Safety
    ///
    /// This function can only be safely called by `<Foo>_Impl` objects that are embedded in a
    /// `ComObject`. Since we only allow safe Rust code to access these objects using a `ComObject`
    /// or a `&<Foo>_Impl` that points within a `ComObject`, this is safe.
    fn to_object(&self) -> ComObject<Self::Impl>
    where
        Self::Impl: ComObjectInner<Outer = Self>;

    // /// The distance from the start of `<Foo>_Impl` to the `this` field within it, measured in
    // /// bytes. The `this` field contains the `MyApp` instance.
    // const INNER_OFFSET_IN_BYTES: usize;
}

impl IUnknown_Vtbl {
    pub const fn new<T: IUnknownImpl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<T: IUnknownImpl, const OFFSET: isize>(
            this: *mut c_void,
            iid: *const GUID,
            interface: *mut *mut c_void,
        ) -> HRESULT {
            unsafe {
                let impl_ptr = (this as *mut *mut c_void).offset(OFFSET) as *mut T;
                let impl_ref = &*impl_ptr;
                let header = impl_ref.header();
                (header.query_interface)(impl_ptr as *const c_void, iid, interface)
            }
        }
        unsafe extern "system" fn AddRef<T: IUnknownImpl, const OFFSET: isize>(
            this: *mut c_void,
        ) -> u32 {
            unsafe {
                let impl_ptr = (this as *mut *mut c_void).offset(OFFSET) as *mut T;
                (*impl_ptr).add_ref()
            }
        }
        unsafe extern "system" fn Release<T: IUnknownImpl, const OFFSET: isize>(
            this: *mut c_void,
        ) -> u32 {
            unsafe {
                let impl_ptr = (this as *mut *mut c_void).offset(OFFSET) as *mut T;
                T::release_ref(impl_ptr)
            }
        }
        Self {
            QueryInterface: QueryInterface::<T, OFFSET>,
            AddRef: AddRef::<T, OFFSET>,
            Release: Release::<T, OFFSET>,
        }
    }
}

/// Header of all COM objects
pub struct ComObjectHeader {
    /// reference count
    pub count: WeakRefCount,
    /// virtual destructor
    pub destructor: unsafe fn(*mut c_void),
    /// query interface implementation
    ///
    /// The `this` pointer points to the base of the ComObject allocation, not a COM interface.
    pub query_interface: unsafe fn(*const c_void, *const GUID, *mut *mut c_void) -> HRESULT,
}
