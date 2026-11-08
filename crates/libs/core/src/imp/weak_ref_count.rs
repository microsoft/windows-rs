use super::*;
use crate::{GUID, HRESULT, IUnknown, IUnknown_Vtbl, Interface};
use core::ffi::c_void;
use core::ptr::{null_mut, without_provenance_mut};
use core::sync::atomic::{AtomicPtr, Ordering, fence};

/// A COM reference count that moves into a weak reference tear-off on demand.
///
/// The value is either a strong count, stored as `count << 1 | 1` without provenance, or a pointer
/// to the tear-off that owns the strong count once `IWeakReferenceSource` has been requested.
/// The tear-off is pointer-aligned, so its low bit is always clear.
#[repr(transparent)]
pub struct WeakRefCount(AtomicPtr<c_void>);

const fn encode(count: usize) -> *mut c_void {
    without_provenance_mut((count << 1) | 1)
}

fn count(value: *mut c_void) -> Option<usize> {
    (value.addr() & 1 == 1).then_some(value.addr() >> 1)
}

impl Default for WeakRefCount {
    fn default() -> Self {
        Self::new()
    }
}

impl WeakRefCount {
    pub const fn new() -> Self {
        Self(AtomicPtr::new(encode(1)))
    }

    pub fn add_ref(&self) -> u32 {
        match self
            .0
            .try_update(Ordering::Relaxed, Ordering::Acquire, |value| {
                count(value).map(|count| {
                    if count >= RefCount::MAX as usize {
                        RefCount::ref_count_overflow();
                    }
                    encode(count + 1)
                })
            }) {
            Ok(previous) => (previous.addr() >> 1) as u32 + 1,
            Err(tear_off) => unsafe { (*tear_off.cast::<TearOff>()).strong_count.add_ref() },
        }
    }

    #[inline(always)]
    pub fn is_one(&self) -> bool {
        self.0.load(Ordering::Acquire) == encode(1)
    }

    pub fn release(&self) -> u32 {
        match self
            .0
            .try_update(Ordering::Release, Ordering::Acquire, |value| {
                count(value).map(|count| {
                    assert!(count != 0, "Object has been over-released.");
                    encode(count - 1)
                })
            }) {
            Ok(previous) => {
                let remaining = (previous.addr() >> 1) as u32 - 1;

                if remaining == 0 {
                    fence(Ordering::Acquire);
                }

                remaining
            }
            Err(tear_off) => unsafe { TearOff::release_strong(tear_off.cast()) },
        }
    }

    /// # Safety
    ///
    /// `object` must be the COM identity pointer associated with this reference count and remain
    /// valid while the strong count is nonzero.
    pub unsafe fn query(&self, iid: &GUID, object: *mut c_void) -> *mut c_void {
        if iid != &IWeakReferenceSource::IID {
            return null_mut();
        }

        let mut current = self.0.load(Ordering::Acquire);
        let mut tear_off: *mut TearOff = null_mut();

        unsafe {
            loop {
                let Some(count) = count(current) else {
                    // Another caller installed a tear-off first.
                    if !tear_off.is_null() {
                        drop(Box::from_raw(tear_off));
                    }

                    return TearOff::add_strong(current.cast());
                };

                if tear_off.is_null() {
                    tear_off = TearOff::new(object, count as u32);
                } else {
                    (*tear_off).strong_count = RefCount::new(count as u32);
                }

                match self.0.compare_exchange_weak(
                    current,
                    tear_off.cast(),
                    Ordering::AcqRel,
                    Ordering::Acquire,
                ) {
                    Ok(_) => return TearOff::add_strong(tear_off),
                    Err(actual) => current = actual,
                }
            }
        }
    }
}

/// The weak reference tear-off.
///
/// `IWeakReferenceSource` points at `strong_vtable` and `IWeakReference` at `weak_vtable`. Both
/// interface pointers are derived from the whole allocation so either can recover the tear-off.
/// The tear-off is shared across threads and only accessed through raw pointers and atomics.
#[repr(C)]
struct TearOff {
    strong_vtable: *const IWeakReferenceSource_Vtbl,
    weak_vtable: *const IWeakReference_Vtbl,
    object: *mut c_void,
    strong_count: RefCount,
    weak_count: RefCount,
}

impl TearOff {
    fn new(object: *mut c_void, strong_count: u32) -> *mut Self {
        #[cfg(windows_cast_diagnostics)]
        live_objects::WEAK_REFERENCES.add();
        Box::into_raw(Box::new(Self {
            strong_vtable: &Self::STRONG_VTABLE,
            weak_vtable: &Self::WEAK_VTABLE,
            object,
            strong_count: RefCount::new(strong_count),
            weak_count: RefCount::new(1),
        }))
    }

    const STRONG_VTABLE: IWeakReferenceSource_Vtbl = IWeakReferenceSource_Vtbl {
        base__: IUnknown_Vtbl {
            QueryInterface: Self::StrongQueryInterface,
            AddRef: Self::StrongAddRef,
            Release: Self::StrongRelease,
        },
        GetWeakReference: Self::StrongDowngrade,
    };

    const WEAK_VTABLE: IWeakReference_Vtbl = IWeakReference_Vtbl {
        base__: IUnknown_Vtbl {
            QueryInterface: Self::WeakQueryInterface,
            AddRef: Self::WeakAddRef,
            Release: Self::WeakRelease,
        },
        Resolve: Self::WeakUpgrade,
    };

    fn strong(this: *mut Self) -> *mut c_void {
        this.cast()
    }

    unsafe fn weak(this: *mut Self) -> *mut c_void {
        unsafe { (&raw mut (*this).weak_vtable).cast() }
    }

    fn from_strong(ptr: *mut c_void) -> *mut Self {
        ptr.cast()
    }

    unsafe fn from_weak(ptr: *mut c_void) -> *mut Self {
        unsafe { ptr.cast::<*const IWeakReference_Vtbl>().sub(1).cast() }
    }

    /// Adds a strong reference and returns the `IWeakReferenceSource` pointer.
    unsafe fn add_strong(this: *mut Self) -> *mut c_void {
        unsafe {
            (*this).strong_count.add_ref();
        }
        Self::strong(this)
    }

    unsafe fn release_strong(this: *mut Self) -> u32 {
        unsafe {
            let remaining = (*this).strong_count.release();

            // The last strong reference also releases its implied weak reference.
            if remaining == 0 {
                Self::release_weak(this);
            }

            remaining
        }
    }

    unsafe fn release_weak(this: *mut Self) -> u32 {
        unsafe {
            let remaining = (*this).weak_count.release();

            // The object has already been destroyed, so the tear-off is no longer reachable.
            if remaining == 0 {
                drop(Box::from_raw(this));
            }

            remaining
        }
    }

    unsafe fn object(this: *mut Self) -> (*mut c_void, &'static IUnknown_Vtbl) {
        unsafe {
            let object = (*this).object;
            // The object may be packed even though its vtable is aligned.
            let vtable = object.cast::<*const IUnknown_Vtbl>().read_unaligned();
            (object, &*vtable)
        }
    }

    unsafe extern "system" fn StrongQueryInterface(
        ptr: *mut c_void,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        unsafe {
            if iid.is_null() || interface.is_null() {
                return E_POINTER;
            }

            if *iid == IWeakReferenceSource::IID {
                *interface = Self::add_strong(Self::from_strong(ptr));
                return HRESULT(0);
            }

            // The tear-off shares the object's identity, so other queries go to the object.
            let (object, vtable) = Self::object(Self::from_strong(ptr));
            (vtable.QueryInterface)(object, iid, interface)
        }
    }

    unsafe extern "system" fn WeakQueryInterface(
        ptr: *mut c_void,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        unsafe {
            let this = Self::from_weak(ptr);

            if iid.is_null() || interface.is_null() {
                return E_POINTER;
            }

            // The weak reference has its own COM identity and does not delegate to the object.
            *interface = if *iid == IWeakReference::IID
                || *iid == IUnknown::IID
                || *iid == IAgileObject::IID
            {
                ptr
            } else {
                #[cfg(windows)]
                if *iid == IMarshal::IID {
                    (*this).weak_count.add_ref();
                    return marshaler(IUnknown::from_raw(ptr), interface);
                }

                null_mut()
            };

            if (*interface).is_null() {
                E_NOINTERFACE
            } else {
                (*this).weak_count.add_ref();
                HRESULT(0)
            }
        }
    }

    unsafe extern "system" fn StrongAddRef(ptr: *mut c_void) -> u32 {
        unsafe { (*Self::from_strong(ptr)).strong_count.add_ref() }
    }

    unsafe extern "system" fn WeakAddRef(ptr: *mut c_void) -> u32 {
        unsafe { (*Self::from_weak(ptr)).weak_count.add_ref() }
    }

    unsafe extern "system" fn StrongRelease(ptr: *mut c_void) -> u32 {
        unsafe {
            // The object owns destruction. Its release decrements the tear-off's strong count
            // and releases the implied weak reference when it reaches zero.
            let (object, vtable) = Self::object(Self::from_strong(ptr));
            (vtable.Release)(object)
        }
    }

    unsafe extern "system" fn WeakRelease(ptr: *mut c_void) -> u32 {
        unsafe { Self::release_weak(Self::from_weak(ptr)) }
    }

    unsafe extern "system" fn StrongDowngrade(
        ptr: *mut c_void,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        unsafe {
            // A strong reference implies a weak reference, so this cannot fail.
            let this = Self::from_strong(ptr);
            (*this).weak_count.add_ref();
            *interface = Self::weak(this);
            HRESULT(0)
        }
    }

    unsafe extern "system" fn WeakUpgrade(
        ptr: *mut c_void,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        unsafe {
            let this = Self::from_weak(ptr);

            // Stabilize the object for the duration of the `QueryInterface` call.
            if !(*this).strong_count.try_add_ref() {
                *interface = null_mut();
                return HRESULT(0);
            }

            let (object, vtable) = Self::object(this);
            let result = (vtable.QueryInterface)(object, iid, interface);

            // Release through the object so that it is destroyed if this was the last reference.
            (vtable.Release)(object);
            result
        }
    }
}

#[cfg(windows_cast_diagnostics)]
impl Drop for TearOff {
    fn drop(&mut self) {
        live_objects::WEAK_REFERENCES.remove();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_limit_does_not_wrap() {
        let count = WeakRefCount(AtomicPtr::new(encode(RefCount::MAX as usize - 1)));
        assert_eq!(count.add_ref(), RefCount::MAX);
        assert!(!count.is_one());
        assert_eq!(count.release(), RefCount::MAX - 1);

        let count = RefCount::new(RefCount::MAX - 1);
        assert_eq!(count.add_ref(), RefCount::MAX);
        assert_eq!(count.release(), RefCount::MAX - 1);
        assert!(count.try_add_ref());
        assert_eq!(count.release(), RefCount::MAX - 1);
        assert!(!RefCount::new(0).try_add_ref());
    }

    #[test]
    #[cfg(not(miri))]
    fn count_overflow_aborts() {
        extern crate std;
        use alloc::string::String;

        const CHILD: &str = "WINDOWS_CORE_TEST_REFCOUNT_OVERFLOW";
        if let Ok(case) = std::env::var(CHILD) {
            // Catching an ordinary panic must not allow the corrupted count to be reused.
            let _ = std::panic::catch_unwind(|| match case.as_str() {
                "inline" => {
                    WeakRefCount(AtomicPtr::new(encode(RefCount::MAX as usize))).add_ref();
                }
                "tear_off_or_delegate" => {
                    RefCount::new(RefCount::MAX).add_ref();
                }
                "upgrade" => {
                    RefCount::new(RefCount::MAX).try_add_ref();
                }
                _ => panic!("unknown overflow test"),
            });
            return;
        }

        for case in ["inline", "tear_off_or_delegate", "upgrade"] {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "imp::weak_ref_count::tests::count_overflow_aborts",
                    "--nocapture",
                ])
                .env(CHILD, case)
                .output()
                .unwrap();
            assert!(!output.status.success(), "{case}");
            assert!(
                String::from_utf8_lossy(&output.stderr).contains("COM reference count overflow."),
                "{case}: {output:?}"
            );
        }
    }
}
