use super::*;
use bindings::*;
use core::sync::atomic::{AtomicIsize, Ordering};

#[doc(hidden)]
#[repr(transparent)]
#[derive(Default)]
pub struct WeakRefCount(AtomicIsize);

impl WeakRefCount {
    pub fn new() -> Self {
        Self(AtomicIsize::new(1))
    }

    pub fn add_ref(&self) -> u32 {
        self.0.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |count_or_pointer| (!is_weak_ref(count_or_pointer)).then(|| count_or_pointer + 1)).map(|u| u as u32 + 1).unwrap_or_else(|pointer| unsafe { TearOff::decode(pointer).strong_count.add_ref() })
    }

    pub fn release(&self) -> u32 {
        self.0.fetch_update(Ordering::Release, Ordering::Relaxed, |count_or_pointer| (!is_weak_ref(count_or_pointer)).then(|| count_or_pointer - 1)).map(|u| u as u32 - 1).unwrap_or_else(|pointer| unsafe {
            let tear_off = TearOff::decode(pointer);
            let remaining = tear_off.strong_count.release();

            // If this is the last strong reference, we can release the weak reference implied by the strong reference.
            // There may still be weak references, so the WeakRelease is called to handle such possibilities.
            if remaining == 0 {
                TearOff::WeakRelease(&mut tear_off.weak_vtable as *mut _ as _);
            }

            remaining
        })
    }

    /// # Safety
    pub unsafe fn query(&self, iid: &::windows::core::GUID, object: RawPtr) -> RawPtr {
        if iid != &IWeakReferenceSource::IID {
            return core::ptr::null_mut();
        }

        let mut count_or_pointer = self.0.load(Ordering::Relaxed);

        if is_weak_ref(count_or_pointer) {
            return TearOff::from_encoding(count_or_pointer);
        }

        let tear_off = TearOff::new(object, count_or_pointer as _);
        let tear_off_ptr: RawPtr = core::mem::transmute_copy(&tear_off);
        let encoding: usize = ((tear_off_ptr as usize) >> 1) | (1 << (core::mem::size_of::<usize>() * 8 - 1));

        loop {
            match self.0.compare_exchange_weak(count_or_pointer, encoding as _, Ordering::AcqRel, Ordering::Relaxed) {
                Ok(_) => {
                    let result: RawPtr = core::mem::transmute(tear_off);
                    TearOff::from_strong_ptr(result).strong_count.add_ref();
                    return result;
                }
                Err(pointer) => count_or_pointer = pointer,
            }

            if is_weak_ref(count_or_pointer) {
                return TearOff::from_encoding(count_or_pointer);
            }

            TearOff::from_strong_ptr(tear_off_ptr).strong_count.0.store(count_or_pointer as _, Ordering::SeqCst);
        }
    }
}

fn is_weak_ref(value: isize) -> bool {
    value < 0
}

#[repr(C)]
struct TearOff {
    strong_vtable: *const IWeakReferenceSource_Vtbl,
    weak_vtable: *const IWeakReference_Vtbl,
    object: RawPtr,
    strong_count: RefCount,
    weak_count: RefCount,
}

impl TearOff {
    #[allow(clippy::new_ret_no_self)]
    unsafe fn new(object: RawPtr, strong_count: u32) -> IWeakReferenceSource {
        core::mem::transmute(windows::core::alloc::boxed::Box::new(TearOff {
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
        core::mem::transmute(tear_off)
    }

    const STRONG_VTABLE: IWeakReferenceSource_Vtbl = IWeakReferenceSource_Vtbl {
        base: IUnknownVtbl { QueryInterface: Self::StrongQueryInterface, AddRef: Self::StrongAddRef, Release: Self::StrongRelease },
        GetWeakReference: Self::StrongDowngrade,
    };

    const WEAK_VTABLE: IWeakReference_Vtbl = IWeakReference_Vtbl {
        base: IUnknownVtbl { QueryInterface: Self::WeakQueryInterface, AddRef: Self::WeakAddRef, Release: Self::WeakRelease },
        Resolve: Self::WeakUpgrade,
    };

    unsafe fn from_strong_ptr<'a>(this: RawPtr) -> &'a mut Self {
        &mut *(this as *mut RawPtr as *mut Self)
    }

    unsafe fn from_weak_ptr<'a>(this: RawPtr) -> &'a mut Self {
        &mut *((this as *mut RawPtr).sub(1) as *mut Self)
    }

    unsafe fn decode<'a>(value: isize) -> &'a mut Self {
        core::mem::transmute(value << 1)
    }

    unsafe fn query_interface(&self, iid: &GUID, interface: *mut RawPtr) -> HRESULT {
        ((*(*(self.object as *mut *mut _) as *mut IUnknownVtbl)).QueryInterface)(self.object, iid, interface)
    }

    unsafe extern "system" fn StrongQueryInterface(ptr: RawPtr, iid: &GUID, interface: *mut RawPtr) -> HRESULT {
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

    unsafe extern "system" fn WeakQueryInterface(ptr: RawPtr, iid: &GUID, interface: *mut RawPtr) -> HRESULT {
        let this = Self::from_weak_ptr(ptr);

        // While the weak vtable is packed into the same allocation as the strong vtable and
        // tear-off, it represents a distinct COM identity and thus does not share or delegate to
        // the object.

        *interface = if iid == &IWeakReference::IID || iid == &IUnknown::IID || iid == &IAgileObject::IID { ptr } else { core::ptr::null_mut() };

        // TODO: implement IMarshal

        if (*interface).is_null() {
            E_NOINTERFACE
        } else {
            this.weak_count.add_ref();
            HRESULT(0)
        }
    }

    unsafe extern "system" fn StrongAddRef(ptr: ::windows::core::RawPtr) -> u32 {
        let this = Self::from_strong_ptr(ptr);

        // Implement `AddRef` directly as we own the strong reference.
        this.strong_count.add_ref()
    }

    unsafe extern "system" fn WeakAddRef(ptr: ::windows::core::RawPtr) -> u32 {
        let this = Self::from_weak_ptr(ptr);

        // Implement `AddRef` directly as we own the weak reference.
        this.weak_count.add_ref()
    }

    unsafe extern "system" fn StrongRelease(ptr: ::windows::core::RawPtr) -> u32 {
        let this = Self::from_strong_ptr(ptr);

        // Forward strong `Release` to the object so that it can destroy itself. It will then
        // decrement its weak reference and allow the tear-off to be released as needed.
        ((*(*(this.object as *mut *mut _) as *mut IUnknownVtbl)).Release)((*this).object)
    }

    unsafe extern "system" fn WeakRelease(ptr: ::windows::core::RawPtr) -> u32 {
        let this = Self::from_weak_ptr(ptr);

        // Implement `Release` directly as we own the weak reference.
        let remaining = (*this).weak_count.release();

        // If there are no remaining references, it means that the object has already been
        // destroyed. Go ahead and destroy the tear-off.
        if remaining == 0 {
            windows::core::alloc::boxed::Box::from_raw(this);
        }

        remaining
    }

    unsafe extern "system" fn StrongDowngrade(ptr: RawPtr, interface: *mut RawPtr) -> HRESULT {
        let this = Self::from_strong_ptr(ptr);

        // The strong vtable hands out a reference to the weak vtable. This is always safe and
        // straightforward since a strong reference guarantees there is at least one weak
        // reference.
        *interface = &mut this.weak_vtable as *mut _ as _;
        this.weak_count.add_ref();
        HRESULT(0)
    }

    unsafe extern "system" fn WeakUpgrade(ptr: RawPtr, iid: *const GUID, interface: *mut RawPtr) -> HRESULT {
        let this = Self::from_weak_ptr(ptr);

        this.strong_count
            .0
            .fetch_update(Ordering::Acquire, Ordering::Relaxed, |count| {
                // Attempt to acquire a strong reference count to stabilize the object for the duration
                // of the `QueryInterface` call.
                (count != 0).then(|| count + 1)
            })
            .map(|_| {
                // Let the object respond to the upgrade query.
                let result = this.query_interface(&*iid, interface);
                // Decrement the temporary reference account used to stabilize the object.
                this.strong_count.0.fetch_sub(1, Ordering::Relaxed);
                // Return the result of the query.
                result
            })
            .unwrap_or_else(|_| {
                *interface = core::ptr::null_mut();
                HRESULT(0)
            })
    }
}
