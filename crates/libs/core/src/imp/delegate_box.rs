use super::*;
use crate::{IUnknown, Interface, GUID, HRESULT};
use core::ffi::c_void;

/// A reference-counted, COM-compatible heap allocation used by generated WinRT delegate types.
///
/// The bindgen-generated `*<Delegate>::new()` allocates an instance of this type and reinterprets
/// the resulting pointer as the delegate. The generated per-delegate code only needs to provide
/// the delegate-specific `Invoke` thunk; the generic `QueryInterface`, `AddRef`, and `Release`
/// implementations are shared across all delegate types via this helper, removing roughly
/// ~80 lines of identical boilerplate per delegate.
#[repr(C)]
#[doc(hidden)]
pub struct DelegateBox<I: Interface, F> {
    pub vtable: *const I::Vtable,
    pub invoke: F,
    pub count: RefCount,
}

impl<I: Interface, F> DelegateBox<I, F> {
    /// Creates a new `DelegateBox` with a reference count of 1.
    pub const fn new(vtable: *const I::Vtable, invoke: F) -> Self {
        Self {
            vtable,
            invoke,
            count: RefCount::new(1),
        }
    }

    /// Generic `IUnknown::QueryInterface` implementation for delegate boxes.
    ///
    /// Responds to the delegate's own IID, `IUnknown`, `IAgileObject`, and (on Windows) `IMarshal`.
    pub unsafe extern "system" fn QueryInterface(
        this: *mut c_void,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        unsafe {
            let this = this as *mut *mut c_void as *mut Self;

            if iid.is_null() || interface.is_null() {
                return HRESULT(-2147467261); // E_POINTER
            }

            *interface = if *iid == <I as Interface>::IID
                || *iid == <IUnknown as Interface>::IID
                || *iid == <IAgileObject as Interface>::IID
            {
                &mut (*this).vtable as *mut _ as _
            } else {
                #[cfg(windows)]
                if *iid == <IMarshal as Interface>::IID {
                    (*this).count.add_ref();
                    return marshaler(
                        core::mem::transmute(&mut (*this).vtable as *mut _ as *mut c_void),
                        interface,
                    );
                }
                core::ptr::null_mut()
            };

            if (*interface).is_null() {
                HRESULT(-2147467262) // E_NOINTERFACE
            } else {
                (*this).count.add_ref();
                HRESULT(0)
            }
        }
    }

    /// Generic `IUnknown::AddRef` implementation for delegate boxes.
    pub unsafe extern "system" fn AddRef(this: *mut c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut c_void as *mut Self;
            (*this).count.add_ref()
        }
    }

    /// Generic `IUnknown::Release` implementation for delegate boxes.
    pub unsafe extern "system" fn Release(this: *mut c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut c_void as *mut Self;
            let remaining = (*this).count.release();

            if remaining == 0 {
                let _ = Box::from_raw(this);
            }

            remaining
        }
    }
}
