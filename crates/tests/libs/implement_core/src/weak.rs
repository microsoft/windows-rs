use std::ffi::c_void;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::sync::{Arc, Barrier, mpsc};
use std::thread;
use std::time::Duration;
use windows_core::*;

#[interface("7a80e456-e4c0-4235-b3b8-55023680721d")]
unsafe trait IValue: IUnknown {
    fn get(&self) -> u32;
}

static_assertions::assert_not_impl_any!(Weak<IValue>: Send, Sync);

#[implement(IValue)]
struct Value(u32, Arc<AtomicBool>);

impl IValue_Impl for Value_Impl {
    unsafe fn get(&self) -> u32 {
        self.0
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        assert!(!self.1.swap(true, SeqCst));
    }
}

// The implementation is free-threaded.
struct Agile(IValue);
unsafe impl Send for Agile {}

#[test]
fn upgrade_and_expire() {
    let dropped = Arc::new(AtomicBool::new(false));
    let value: IValue = Value(7, dropped.clone()).into();
    let first = value.downgrade().unwrap();
    let second = value.downgrade().unwrap();

    let strong = first.upgrade().unwrap();
    assert_eq!(unsafe { strong.get() }, 7);
    assert_eq!(strong, value);
    drop(strong);

    drop(value);
    assert!(dropped.load(SeqCst));
    assert!(first.upgrade().is_none());
    assert!(second.upgrade().is_none());
}

#[test]
fn concurrent_downgrade_upgrade_release() {
    for _ in 0..if cfg!(miri) { 4 } else { 200 } {
        let dropped = Arc::new(AtomicBool::new(false));
        let value: IValue = Value(7, dropped.clone()).into();

        let (progress, receive_progress) = mpsc::channel();
        let threads: Vec<_> = (0..4)
            .map(|_| {
                let agile = Agile(value.clone());
                let progress = progress.clone();
                let (proceed, receive_proceed) = mpsc::channel();
                let thread = thread::spawn(move || {
                    let agile = agile;
                    // Race to install the tear-off, keeping each weak handle on its thread.
                    let weak = agile.0.downgrade().unwrap();
                    drop(agile);
                    progress.send(()).unwrap();
                    receive_proceed
                        .recv_timeout(Duration::from_secs(10))
                        .unwrap();

                    // Race upgrades against the final release.
                    if let Some(value) = weak.upgrade() {
                        assert_eq!(unsafe { value.get() }, 7);
                    }
                    progress.send(()).unwrap();
                    receive_proceed
                        .recv_timeout(Duration::from_secs(10))
                        .unwrap();
                    assert!(weak.upgrade().is_none());
                });
                (proceed, thread)
            })
            .collect();
        drop(progress);

        for _ in &threads {
            receive_progress
                .recv_timeout(Duration::from_secs(10))
                .unwrap();
        }
        for (proceed, _) in &threads {
            proceed.send(()).unwrap();
        }
        drop(value);
        for _ in &threads {
            receive_progress
                .recv_timeout(Duration::from_secs(10))
                .unwrap();
        }
        for (proceed, thread) in threads {
            proceed.send(()).unwrap();
            thread.join().unwrap();
        }

        assert!(dropped.load(SeqCst));
    }
}

#[test]
fn failed_resolution_releases_last_strong_reference() {
    #[repr(C)]
    struct Object {
        vtable: &'static IUnknown_Vtbl,
        count: imp::WeakRefCount,
        entered: Arc<Barrier>,
        resume: Arc<Barrier>,
        dropped: Arc<AtomicBool>,
    }

    impl Drop for Object {
        fn drop(&mut self) {
            assert!(!self.dropped.swap(true, SeqCst));
        }
    }

    unsafe extern "system" fn query(
        this: *mut c_void,
        iid: *const GUID,
        result: *mut *mut c_void,
    ) -> HRESULT {
        unsafe {
            let object = &*this.cast::<Object>();
            if *iid == IUnknown::IID {
                object.count.add_ref();
                *result = this;
                return HRESULT(0);
            }
            *result = object.count.query(&*iid, this, true);
            if !(*result).is_null() {
                return HRESULT(0);
            }

            // Resolve has stabilized the object. Release its last external strong reference
            // before allowing this failed query to return.
            object.entered.wait();
            object.resume.wait();
            imp::E_NOINTERFACE
        }
    }

    unsafe extern "system" fn add_ref(this: *mut c_void) -> u32 {
        unsafe { (*this.cast::<Object>()).count.add_ref() }
    }

    unsafe extern "system" fn release(this: *mut c_void) -> u32 {
        unsafe {
            let object = this.cast::<Object>();
            let remaining = (*object).count.release();
            if remaining == 0 {
                drop(Box::from_raw(object));
            }
            remaining
        }
    }

    let entered = Arc::new(Barrier::new(2));
    let resume = Arc::new(Barrier::new(2));
    let dropped = Arc::new(AtomicBool::new(false));
    let object = unsafe {
        IUnknown::from_raw(
            Box::into_raw(Box::new(Object {
                vtable: &IUnknown_Vtbl {
                    QueryInterface: query,
                    AddRef: add_ref,
                    Release: release,
                },
                count: imp::WeakRefCount::new(),
                entered: entered.clone(),
                resume: resume.clone(),
                dropped: dropped.clone(),
            }))
            .cast(),
        )
    };
    let source: imp::IWeakReferenceSource = object.cast().unwrap();
    let weak = unsafe {
        let mut result = null_mut();
        assert_eq!(
            (source.vtable().GetWeakReference)(source.as_raw(), &mut result),
            HRESULT(0)
        );
        imp::IWeakReference::from_raw(result)
    };
    drop(source);

    struct SendWeak(imp::IWeakReference);
    // The weak reference implementation is free-threaded.
    unsafe impl Send for SendWeak {}

    let weak = SendWeak(weak);
    let worker = thread::spawn(move || {
        let weak = weak;
        let mut result = null_mut();
        let status =
            unsafe { (weak.0.vtable().Resolve)(weak.0.as_raw(), &GUID::from_u128(1), &mut result) };
        assert_eq!(status, imp::E_NOINTERFACE);
        assert!(result.is_null());
    });

    entered.wait();
    drop(object);
    assert!(!dropped.load(SeqCst));
    resume.wait();
    worker.join().unwrap();
    assert!(dropped.load(SeqCst));
}
