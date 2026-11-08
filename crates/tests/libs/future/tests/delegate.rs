// Exercises the shared delegate implementation without calling native APIs so it can run under Miri.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use windows_core::*;
use windows_future::*;

const E_NOINTERFACE: HRESULT = HRESULT(0x80004002_u32 as _);

struct Captured(Arc<AtomicUsize>);

impl Drop for Captured {
    fn drop(&mut self) {
        assert_eq!(self.0.fetch_add(1, Ordering::Relaxed), 0);
    }
}

#[test]
fn lifetime_and_identity() -> Result<()> {
    let invoked = Arc::new(AtomicUsize::new(0));
    let dropped = Arc::new(AtomicUsize::new(0));
    let captured = Captured(dropped.clone());
    let counter = invoked.clone();

    let handler = AsyncActionCompletedHandler::new(move |sender, status| {
        let _ = &captured;
        assert!(sender.is_none());
        assert_eq!(status, AsyncStatus::Completed);
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    });

    handler.Invoke(None, AsyncStatus::Completed)?;

    let unknown: IUnknown = handler.cast()?;
    let clone: AsyncActionCompletedHandler = unknown.cast()?;
    clone.Invoke(None, AsyncStatus::Completed)?;
    assert_eq!(handler, clone);
    for iid in [IInspectable::IID, IAsyncAction::IID] {
        let mut ptr = std::ptr::null_mut();
        assert_eq!(unsafe { unknown.query(&iid, &mut ptr) }, E_NOINTERFACE);
        assert!(ptr.is_null());
    }

    drop(handler);
    drop(clone);
    assert_eq!(dropped.load(Ordering::Relaxed), 0);
    drop(unknown);

    assert_eq!(invoked.load(Ordering::Relaxed), 2);
    assert_eq!(dropped.load(Ordering::Relaxed), 1);
    Ok(())
}
