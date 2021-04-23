use crate::*;
use std::sync::atomic::{fence, AtomicIsize, AtomicU32, Ordering};

/// A thread-safe reference count for use with COM weak reference implementations.
#[repr(transparent)]
#[derive(Default)]
pub struct WeakRefCount(AtomicIsize);

impl WeakRefCount {
    pub fn new() -> Self {
        Self(AtomicIsize::new(1))
    }

    pub fn add_ref(&self) -> u32 {
        (self.0.fetch_add(1, Ordering::Relaxed) + 1) as u32
    }

    pub fn release(&self) -> u32 {
        let remaining = self.0.fetch_sub(1, Ordering::Release) - 1;

        if remaining == 0 {
            fence(Ordering::Acquire);
        } else if remaining < 0 {
            panic!("Object has been over-released.");
        }

        remaining as u32
    }
}

#[repr(C)]
struct TearOff {
    strong_vtable: *const IWeakReferenceSource_vtable,
    weak_vtable: *const IWeakReference_vtable,
    object: RawPtr,
    strong_count: AtomicU32,
    weak_count: AtomicU32,
}

impl TearOff {
    fn new(object: RawPtr, strong_count: u32) -> IWeakReferenceSource {
        unsafe {
            std::mem::transmute(::std::boxed::Box::new(TearOff {
                strong_vtable: &Self::STRONG_VTABLE,
                weak_vtable: &Self::WEAK_VTABLE,
                object,
                strong_count: AtomicU32::new(strong_count),
                weak_count: AtomicU32::new(1),
            }))
        }
    }

    const STRONG_VTABLE: IWeakReferenceSource_vtable = IWeakReferenceSource_vtable(
        Self::StrongQueryInterface,
        Self::StrongAddRef,
        Self::StrongRelease,
        Self::StrongDowngrade,
    );

    const WEAK_VTABLE: IWeakReference_vtable = IWeakReference_vtable(
        Self::WeakQueryInterface,
        Self::WeakAddRef,
        Self::WeakRelease,
        Self::WeakUpgrade,
    );

    unsafe extern "system" fn StrongQueryInterface(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> HRESULT {
        let this = this as *mut RawPtr as *mut Self;
        panic!();
    }

    unsafe extern "system" fn StrongAddRef(this: ::windows::RawPtr) -> u32 {
        let this = this as *mut RawPtr as *mut Self;
        // Implement strong `AddRef` directly as we own the reference.
        ((*this).strong_count.fetch_add(1, Ordering::Relaxed) + 1) as u32
    }

    unsafe extern "system" fn StrongRelease(this: ::windows::RawPtr) -> u32 {
        let this = this as *mut RawPtr as *mut Self;
        // Forward strong `Release` to the object so that it can destroy itself.
        // It will then cause the tear-off to be destroyed as needed.
        ((*(*((*this).object as *mut *mut _) as *mut IUnknown_vtable)).2)((*this).object)
    }

    unsafe extern "system" fn StrongDowngrade(this: RawPtr, interface: *mut RawPtr) -> HRESULT {
        let this = this as *mut RawPtr as *mut Self;
        panic!();
    }

    unsafe extern "system" fn WeakQueryInterface(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> HRESULT {
        let this = (this as *mut RawPtr).sub(1) as *mut Self;
        panic!();
    }

    unsafe extern "system" fn WeakAddRef(this: ::windows::RawPtr) -> u32 {
        let this = (this as *mut RawPtr).sub(1) as *mut Self;
        panic!();
    }

    unsafe extern "system" fn WeakRelease(this: ::windows::RawPtr) -> u32 {
        let this = (this as *mut RawPtr).sub(1) as *mut Self;
        ((*this).weak_count.fetch_add(1, Ordering::Relaxed) + 1) as u32
    }

    unsafe extern "system" fn WeakUpgrade(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> HRESULT {
        let this = (this as *mut RawPtr).sub(1) as *mut Self;
        panic!();
    }
}
