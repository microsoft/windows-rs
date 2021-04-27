#![allow(
    non_upper_case_globals,
    non_snake_case,
    non_camel_case_types,
    dead_code,
    unused_variables
)]

use crate::*;
use bindings::Windows::Win32::WinRT::{
    IWeakReference, IWeakReferenceSource, IWeakReferenceSource_abi, IWeakReference_abi,
};
use std::sync::atomic::{AtomicIsize, Ordering};

/// A thread-safe reference count for use with COM weak reference implementations.
#[repr(transparent)]
#[derive(Default)]
pub struct WeakRefCount(AtomicIsize);

impl WeakRefCount {
    pub fn new() -> Self {
        Self(AtomicIsize::new(1))
    }

    pub fn add_ref(&self) -> u32 {
        let count_or_pointer = self.0.load(Ordering::Relaxed);

        loop {
            if is_weak_ref(count_or_pointer) {
                unsafe {
                    return TearOff::decode(count_or_pointer).strong_count.add_ref();
                }
            }

            if self
                .0
                .compare_exchange_weak(
                    count_or_pointer,
                    count_or_pointer + 1,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                return count_or_pointer as u32 + 1;
            }
        }
    }

    pub fn release(&self) -> u32 {
        let count_or_pointer = self.0.load(Ordering::Relaxed);

        loop {
            if is_weak_ref(count_or_pointer) {
                unsafe {
                    return TearOff::decode(count_or_pointer).strong_count.release();
                }
            }

            if self
                .0
                .compare_exchange_weak(
                    count_or_pointer,
                    count_or_pointer - 1,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                return count_or_pointer as u32 - 1;
            }
        }
    }

    pub unsafe fn query(&self, iid: &::windows::Guid, object: RawPtr) -> RawPtr {
        if iid != &IWeakReferenceSource::IID {
            return std::ptr::null_mut();
        }

        let count_or_pointer = self.0.load(Ordering::Relaxed);

        if is_weak_ref(count_or_pointer) {
            return TearOff::from_encoding(count_or_pointer);
        }

        let tear_off = TearOff::new(object, count_or_pointer as _);
        let encoding: usize =
            ((tear_off.abi() as usize) >> 1) | (1 << (std::mem::size_of::<usize>() * 8 - 1));

        loop {
            if self
                .0
                .compare_exchange_weak(
                    count_or_pointer,
                    encoding as _,
                    Ordering::AcqRel,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                return std::mem::transmute(tear_off);
            }

            if is_weak_ref(count_or_pointer) {
                return TearOff::from_encoding(count_or_pointer);
            }

            TearOff::from_strong_ptr(tear_off.abi())
                .strong_count
                .0
                .store(count_or_pointer as _, Ordering::SeqCst);
        }
    }
}

fn is_weak_ref(value: isize) -> bool {
    value < 0
}

#[repr(C)]
struct TearOff {
    strong_vtable: *const IWeakReferenceSource_abi,
    weak_vtable: *const IWeakReference_abi,
    object: RawPtr,
    strong_count: RefCount,
    weak_count: RefCount,
}

impl TearOff {
    unsafe fn new(object: RawPtr, strong_count: u32) -> IWeakReferenceSource {
        std::mem::transmute(::std::boxed::Box::new(TearOff {
            strong_vtable: &Self::STRONG_VTABLE,
            weak_vtable: &Self::WEAK_VTABLE,
            object,
            strong_count: RefCount::new(strong_count),
            weak_count: RefCount::new(1),
        }))
    }

    unsafe fn from_encoding(encoding: isize) -> RawPtr {
        let tear_off = TearOff::decode(encoding);
        tear_off.strong_count.add_ref();
        return std::mem::transmute(tear_off.strong_vtable);
    }

    const STRONG_VTABLE: IWeakReferenceSource_abi = IWeakReferenceSource_abi(
        Self::StrongQueryInterface,
        Self::StrongAddRef,
        Self::StrongRelease,
        Self::StrongDowngrade,
    );

    const WEAK_VTABLE: IWeakReference_abi = IWeakReference_abi(
        Self::WeakQueryInterface,
        Self::WeakAddRef,
        Self::WeakRelease,
        Self::WeakUpgrade,
    );

    unsafe fn from_strong_ptr<'a>(this: RawPtr) -> &'a mut Self {
        &mut *(this as *mut RawPtr as *mut Self)
    }

    unsafe fn from_weak_ptr<'a>(this: RawPtr) -> &'a mut Self {
        &mut *((this as *mut RawPtr).sub(1) as *mut Self)
    }

    unsafe fn decode<'a>(value: isize) -> &'a mut Self {
        std::mem::transmute(value << 1)
    }

    unsafe fn query_interface(&self, iid: *const Guid, interface: *mut RawPtr) -> HRESULT {
        ((*(*(self.object as *mut *mut _) as *mut IUnknown_abi)).0)(self.object, iid, interface)
    }

    unsafe extern "system" fn StrongQueryInterface(
        ptr: RawPtr,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> HRESULT {
        let this = Self::from_strong_ptr(ptr);

        // Only directly respond to queries for the the tear-off's strong interface. This is
        // effectively a self-query.
        if iid == &IWeakReferenceSource::IID {
            *interface = ptr;
            this.strong_count.add_ref();
            return HRESULT(0);
        }

        // As the tear-off is sharing the identity of the object, simply delegate any remaining
        // queries to the object.
        this.query_interface(iid, interface)
    }

    unsafe extern "system" fn WeakQueryInterface(
        ptr: RawPtr,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> HRESULT {
        let this = Self::from_weak_ptr(ptr);

        // While the weak vtable is packed into the same allocation as the strong vtable and
        // tear-off, it represents a distinct COM identity and thus does not share or delegate to
        // the object.

        *interface =
            if iid == &IWeakReference::IID || iid == &IUnknown::IID || iid == &IAgileObject::IID {
                ptr
            } else {
                std::ptr::null_mut()
            };

        // TODO: implement IMarshal

        if (*interface).is_null() {
            HRESULT(0x8000_4002) // E_NOINTERFACE
        } else {
            this.weak_count.add_ref();
            HRESULT(0)
        }
    }

    unsafe extern "system" fn StrongAddRef(ptr: ::windows::RawPtr) -> u32 {
        let this = Self::from_strong_ptr(ptr);

        // Implement `AddRef` directly as we own the strong reference.
        this.strong_count.add_ref()
    }

    unsafe extern "system" fn WeakAddRef(ptr: ::windows::RawPtr) -> u32 {
        let this = Self::from_weak_ptr(ptr);

        // Implement `AddRef` directly as we own the weak reference.
        this.weak_count.add_ref()
    }

    unsafe extern "system" fn StrongRelease(ptr: ::windows::RawPtr) -> u32 {
        let this = Self::from_strong_ptr(ptr);

        // Forward strong `Release` to the object so that it can destroy itself. It will then
        // decrement its weak reference and allow the tear-off to be released as needd.
        ((*(*(this.object as *mut *mut _) as *mut IUnknown_abi)).2)((*this).object)
    }

    unsafe extern "system" fn WeakRelease(ptr: ::windows::RawPtr) -> u32 {
        let this = Self::from_weak_ptr(ptr);

        // Implement `Release` directly as we own the weak reference.
        let remaining = (*this).weak_count.release();

        // If there are no remaining references, it means that the object has already been
        // destroyed. Go ahead and destroy the tear-off.
        if remaining == 0 {
            Box::from_raw(this);
        }

        remaining
    }

    unsafe extern "system" fn StrongDowngrade(ptr: RawPtr, interface: *mut RawPtr) -> HRESULT {
        let this = Self::from_strong_ptr(ptr);

        // The strong vtable hands out a reference to the weak vtable. This is always safe and
        // straightforward since a strong refernece guarantees there is at lerast one weak
        // reference.
        *interface = &mut this.weak_vtable as *mut _ as _;
        this.weak_count.add_ref();
        HRESULT(0)
    }

    unsafe extern "system" fn WeakUpgrade(
        ptr: RawPtr,
        iid: *const Guid,
        interface: *mut RawPtr,
    ) -> HRESULT {
        let this = Self::from_weak_ptr(ptr);

        let count = this.strong_count.0.load(Ordering::Relaxed);

        loop {
            if count == 0 {
                *interface = std::ptr::null_mut();
                return HRESULT(0);
            }

            // Attempt to acquire a strong reference count to stabilize the object for the duration
            // of the `QueryInterface` call.
            if this
                .strong_count
                .0
                .compare_exchange_weak(count, count + 1, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                // Let the object respond to the upgrade query.
                let result = this.query_interface(iid, interface);
                // Decrement the temporary reference account used to stablize the object.
                this.strong_count.0.fetch_sub(1, Ordering::Relaxed);
                // Return the result of the query.
                return result;
            }
        }
    }
}
