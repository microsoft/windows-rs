use super::delegate::Delegate;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use windows_core::{GUID, HRESULT, IUnknown, Interface, imp::IAgileObject};

struct Capture {
    calls: Arc<AtomicUsize>,
    drops: Arc<AtomicUsize>,
}

impl Capture {
    fn invoke(&self, sender: i32, args: i32) {
        assert_eq!((sender, args), (12, 34));
        self.calls.fetch_add(1, Ordering::Relaxed);
    }
}

impl Drop for Capture {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn query_owns_capture() {
    for iid in [Delegate::IID, IUnknown::IID, IAgileObject::IID] {
        let calls = Arc::new(AtomicUsize::new(0));
        let drops = Arc::new(AtomicUsize::new(0));
        let capture = Capture {
            calls: calls.clone(),
            drops: drops.clone(),
        };
        let delegate = Delegate::new(move |sender, args| {
            capture.invoke(sender, args);
            Ok(())
        });
        let clone = delegate.clone();
        clone.Invoke(12, 34).unwrap();

        let mut raw = core::ptr::null_mut();
        assert_eq!(unsafe { clone.query(&iid, &mut raw) }, HRESULT(0));
        assert_eq!(raw, delegate.as_raw());
        // All supported interfaces begin with IUnknown and own the returned reference.
        let unknown = unsafe { IUnknown::from_raw(raw) };
        drop(delegate);
        drop(clone);
        assert_eq!(drops.load(Ordering::Relaxed), 0);

        let queried = unknown.cast::<Delegate>().unwrap();
        let survivor = queried.clone();
        drop(unknown);
        drop(queried);
        survivor.Invoke(12, 34).unwrap();
        assert_eq!(calls.load(Ordering::Relaxed), 2);
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        drop(survivor);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
    }
}

#[test]
fn query_errors_preserve_ownership() {
    let calls = Arc::new(AtomicUsize::new(0));
    let drops = Arc::new(AtomicUsize::new(0));
    let capture = Capture {
        calls: calls.clone(),
        drops: drops.clone(),
    };
    let delegate = Delegate::new(move |sender, args| {
        capture.invoke(sender, args);
        Ok(())
    });

    unsafe {
        let mut raw = delegate.as_raw();
        assert_eq!(
            delegate.query(&GUID::zeroed(), &mut raw),
            HRESULT(0x80004002u32 as i32)
        );
        assert!(raw.is_null());
        // Test invalid arguments at the ABI, without violating Interface::query's contract.
        let query = delegate.assume_vtable::<IUnknown>().QueryInterface;
        assert_eq!(
            query(delegate.as_raw(), core::ptr::null(), &mut raw),
            HRESULT(0x80004003u32 as i32)
        );
        assert_eq!(
            query(delegate.as_raw(), &Delegate::IID, core::ptr::null_mut()),
            HRESULT(0x80004003u32 as i32)
        );
    }

    delegate.Invoke(12, 34).unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(delegate);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
}

#[cfg(all(windows, not(miri)))]
#[test]
fn marshaler_owns_capture() {
    let calls = Arc::new(AtomicUsize::new(0));
    let drops = Arc::new(AtomicUsize::new(0));
    let capture = Capture {
        calls: calls.clone(),
        drops: drops.clone(),
    };
    let delegate = Delegate::new(move |sender, args| {
        capture.invoke(sender, args);
        Ok(())
    });

    // This path creates the native free-threaded marshaler, which Miri cannot execute.
    let identity = delegate.as_raw();
    let marshaler = delegate.cast::<windows_core::imp::IMarshal>().unwrap();
    drop(delegate);
    assert_eq!(drops.load(Ordering::Relaxed), 0);

    let clone = marshaler.clone();
    let unknown = marshaler.cast::<IUnknown>().unwrap();
    assert_eq!(unknown.as_raw(), identity);
    let delegate = unknown.cast::<Delegate>().unwrap();
    drop(marshaler);
    drop(unknown);
    delegate.Invoke(12, 34).unwrap();
    drop(delegate);
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(clone);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
}
