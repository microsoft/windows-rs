use super::delegate::Delegate;
use std::sync::{
    Arc, Barrier,
    atomic::{AtomicUsize, Ordering},
};
use windows_core::{Event, GUID, HRESULT, IUnknown, Interface, imp::IAgileObject};

#[test]
fn implemented_non_event_methods_keep_delegate_constructors() {
    let _handler = super::narrow_event_implemented::Handler::new(|_| {});
}

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

#[test]
fn event_concurrent_calls() {
    let event = Event::<Delegate>::new();
    let calls = Arc::new(AtomicUsize::new(0));
    let drops = Arc::new(AtomicUsize::new(0));
    let capture = Capture {
        calls: calls.clone(),
        drops: drops.clone(),
    };
    let barrier = Barrier::new(2);
    event
        .add(&Delegate::new(move |sender, args| {
            barrier.wait();
            capture.invoke(sender, args);
            Ok(())
        }))
        .unwrap();

    std::thread::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|| event.call(|delegate| delegate.Invoke(12, 34)));
        }
    });
    assert_eq!(calls.load(Ordering::Relaxed), 2);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    event.clear();
    assert_eq!(drops.load(Ordering::Relaxed), 1);
}

#[test]
fn event_reentrant_invocation() {
    let event = Arc::new(Event::<Delegate>::new());
    let calls = Arc::new(AtomicUsize::new(0));
    let drops = Arc::new(AtomicUsize::new(0));
    let capture = Capture {
        calls: calls.clone(),
        drops: drops.clone(),
    };
    let weak = Arc::downgrade(&event);
    let reentered = std::sync::atomic::AtomicBool::new(false);
    event
        .add(&Delegate::new(move |sender, args| {
            capture.invoke(sender, args);
            if !reentered.swap(true, Ordering::Relaxed) {
                weak.upgrade()
                    .unwrap()
                    .call(|delegate| delegate.Invoke(sender, args));
            }
            Ok(())
        }))
        .unwrap();

    event.call(|delegate| delegate.Invoke(12, 34));
    assert_eq!(calls.load(Ordering::Relaxed), 2);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    event.clear();
    assert_eq!(drops.load(Ordering::Relaxed), 1);
}

#[test]
fn event_snapshot_owns_capture() {
    for clear in [false, true] {
        let event = Event::<Delegate>::new();
        let calls = Arc::new(AtomicUsize::new(0));
        let drops = Arc::new(AtomicUsize::new(0));
        let capture = Capture {
            calls: calls.clone(),
            drops: drops.clone(),
        };
        let barrier = Arc::new(Barrier::new(2));
        let callback_barrier = barrier.clone();
        let token = event
            .add(&Delegate::new(move |sender, args| {
                callback_barrier.wait();
                callback_barrier.wait();
                capture.invoke(sender, args);
                Ok(())
            }))
            .unwrap();

        std::thread::scope(|scope| {
            scope.spawn(|| event.call(|delegate| delegate.Invoke(12, 34)));
            barrier.wait();
            if clear {
                event.clear();
            } else {
                event.remove(token);
            }
            assert_eq!(drops.load(Ordering::Relaxed), 0);
            event.call(|_| panic!("removed delegate was called"));
            barrier.wait();
        });
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        drop(event);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
    }
}

#[test]
fn event_reentrant_registration() {
    let event = Arc::new(Event::<Delegate>::new());
    let calls: [_; 3] = std::array::from_fn(|_| Arc::new(AtomicUsize::new(0)));
    let drops: [_; 3] = std::array::from_fn(|_| Arc::new(AtomicUsize::new(0)));
    for index in 0..2 {
        let capture = Capture {
            calls: calls[index].clone(),
            drops: drops[index].clone(),
        };
        let weak = Arc::downgrade(&event);
        let replacement_calls = calls[2].clone();
        let replacement_drops = drops[2].clone();
        event
            .add(&Delegate::new(move |sender, args| {
                capture.invoke(sender, args);
                if index == 0 {
                    let event = weak.upgrade().unwrap();
                    event.clear();
                    let replacement = Capture {
                        calls: replacement_calls.clone(),
                        drops: replacement_drops.clone(),
                    };
                    event.add(&Delegate::new(move |sender, args| {
                        replacement.invoke(sender, args);
                        Ok(())
                    }))?;
                }
                Ok(())
            }))
            .unwrap();
    }

    event.call(|delegate| delegate.Invoke(12, 34));
    for index in 0..2 {
        assert_eq!(calls[index].load(Ordering::Relaxed), 1);
        assert_eq!(drops[index].load(Ordering::Relaxed), 1);
    }
    assert_eq!(calls[2].load(Ordering::Relaxed), 0);
    assert_eq!(drops[2].load(Ordering::Relaxed), 0);
    event.call(|delegate| delegate.Invoke(12, 34));
    assert_eq!(calls[2].load(Ordering::Relaxed), 1);
    event.clear();
    assert_eq!(drops[2].load(Ordering::Relaxed), 1);
}
