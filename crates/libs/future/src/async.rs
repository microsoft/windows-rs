use super::*;
use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

struct Once<T> {
    taken: AtomicBool,
    value: UnsafeCell<Option<T>>,
}

// Only the winning take accesses the value through a shared reference.
unsafe impl<T: Send> Sync for Once<T> {}

impl<T> Once<T> {
    fn new(value: T) -> Self {
        Self {
            taken: AtomicBool::new(false),
            value: UnsafeCell::new(Some(value)),
        }
    }

    fn take(&self) -> Option<T> {
        if self.taken.swap(true, Ordering::Relaxed) {
            None
        } else {
            unsafe { (*self.value.get()).take() }
        }
    }
}

/// Represents a WinRT async execution object or type. There are precisely four
/// such types:
/// - [`IAsyncAction`]
/// - [`IAsyncActionWithProgress`]
/// - [`IAsyncOperation`]
/// - [`IAsyncOperationWithProgress`]
///
/// All four implementations are provided by this crate, so there is no need to
/// implement this trait yourself. It abstracts over the relevant differences so
/// that the crate's async capabilities can be reused across all four.
pub trait Async: Interface {
    /// The type of value produced on completion.
    type Output: Clone;

    /// The type of the delegate used for completion notification.
    type CompletedHandler: Interface;

    /// Sets the handler to invoke when execution completes. This handler can
    /// only be set once.
    fn set_completed<F: Fn(&Self) + Send + Sync + 'static>(&self, handler: F) -> Result<()>;

    /// Calls the given handler with the current object and status.
    #[cfg(feature = "std")]
    fn invoke_completed(&self, handler: &Self::CompletedHandler, status: AsyncStatus);

    /// Returns the value produced on completion. This should only be called once
    /// execution completes.
    fn get_results(&self) -> Result<Self::Output>;

    /// Gets the current status of async execution. This calls `QueryInterface`
    /// so should be used sparingly.
    fn status(&self) -> Result<AsyncStatus>;

    /// Waits for the async execution to finish and then returns the results.
    fn join(&self) -> Result<Self::Output> {
        if self.status()? == AsyncStatus::Started {
            let (waiter, signaler) = Waiter::new()?;
            let signaler = Once::new(signaler);
            self.set_completed(move |_| {
                if let Some(signaler) = signaler.take() {
                    // The waiter is kept alive until this signal and cannot be signaled again.
                    unsafe {
                        signaler.signal();
                    }
                }
            })?;
            waiter.wait();
        }
        self.get_results()
    }

    /// Calls `op(result)` when async execution completes.
    fn when<F>(&self, op: F) -> Result<()>
    where
        F: FnOnce(Result<Self::Output>) + Send + 'static,
    {
        if self.status()? == AsyncStatus::Started {
            let op = Once::new(op);
            self.set_completed(move |sender| {
                if let Some(op) = op.take() {
                    op(sender.get_results());
                }
            })?;
        } else {
            op(self.get_results());
        }
        Ok(())
    }
}

impl Async for IAsyncAction {
    type Output = ();
    type CompletedHandler = AsyncActionCompletedHandler;

    fn set_completed<F: Fn(&Self) + Send + Sync + 'static>(&self, handler: F) -> Result<()> {
        self.SetCompleted(&AsyncActionCompletedHandler::new(move |sender, _| {
            handler(sender.ok()?);
            Ok(())
        }))
    }

    #[cfg(feature = "std")]
    fn invoke_completed(&self, handler: &Self::CompletedHandler, status: AsyncStatus) {
        _ = handler.Invoke(self, status);
    }

    fn get_results(&self) -> Result<Self::Output> {
        self.GetResults()
    }

    fn status(&self) -> Result<AsyncStatus> {
        self.Status()
    }
}

impl<T: RuntimeType> Async for IAsyncOperation<T> {
    type Output = T;
    type CompletedHandler = AsyncOperationCompletedHandler<T>;

    fn set_completed<F: Fn(&Self) + Send + Sync + 'static>(&self, handler: F) -> Result<()> {
        self.SetCompleted(&AsyncOperationCompletedHandler::new(move |sender, _| {
            handler(sender.ok()?);
            Ok(())
        }))
    }

    #[cfg(feature = "std")]
    fn invoke_completed(&self, handler: &Self::CompletedHandler, status: AsyncStatus) {
        _ = handler.Invoke(self, status);
    }

    fn get_results(&self) -> Result<Self::Output> {
        self.GetResults()
    }

    fn status(&self) -> Result<AsyncStatus> {
        self.Status()
    }
}

impl<P: RuntimeType> Async for IAsyncActionWithProgress<P> {
    type Output = ();
    type CompletedHandler = AsyncActionWithProgressCompletedHandler<P>;

    fn set_completed<F: Fn(&Self) + Send + Sync + 'static>(&self, handler: F) -> Result<()> {
        self.SetCompleted(&AsyncActionWithProgressCompletedHandler::new(
            move |sender, _| {
                handler(sender.ok()?);
                Ok(())
            },
        ))
    }

    #[cfg(feature = "std")]
    fn invoke_completed(&self, handler: &Self::CompletedHandler, status: AsyncStatus) {
        _ = handler.Invoke(self, status);
    }

    fn get_results(&self) -> Result<Self::Output> {
        self.GetResults()
    }

    fn status(&self) -> Result<AsyncStatus> {
        self.Status()
    }
}

impl<T: RuntimeType, P: RuntimeType> Async for IAsyncOperationWithProgress<T, P> {
    type Output = T;
    type CompletedHandler = AsyncOperationWithProgressCompletedHandler<T, P>;

    fn set_completed<F: Fn(&Self) + Send + Sync + 'static>(&self, handler: F) -> Result<()> {
        self.SetCompleted(&AsyncOperationWithProgressCompletedHandler::new(
            move |sender, _| {
                handler(sender.ok()?);
                Ok(())
            },
        ))
    }

    #[cfg(feature = "std")]
    fn invoke_completed(&self, handler: &Self::CompletedHandler, status: AsyncStatus) {
        _ = handler.Invoke(self, status);
    }

    fn get_results(&self) -> Result<Self::Output> {
        self.GetResults()
    }

    fn status(&self) -> Result<AsyncStatus> {
        self.Status()
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use std::{
        cell::Cell,
        sync::{Arc, Barrier, atomic::AtomicUsize},
    };

    struct Capture(Arc<AtomicUsize>, Cell<usize>);

    impl Drop for Capture {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn once_concurrent_take() {
        let drops = Arc::new(AtomicUsize::new(0));
        let calls = AtomicUsize::new(0);
        let once = Once::new(Capture(drops.clone(), Cell::new(0)));
        let barrier = Barrier::new(2);
        std::thread::scope(|scope| {
            for _ in 0..2 {
                scope.spawn(|| {
                    barrier.wait();
                    if let Some(capture) = once.take() {
                        capture.1.set(capture.1.get() + 1);
                        assert_eq!(capture.1.get(), 1);
                        calls.fetch_add(1, Ordering::Relaxed);
                    }
                });
            }
        });
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        drop(once);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn once_capture_lifetime() {
        let drops = Arc::new(AtomicUsize::new(0));
        drop(Once::new(Capture(drops.clone(), Cell::new(0))));
        assert_eq!(drops.load(Ordering::Relaxed), 1);

        let once = Once::new(Capture(drops.clone(), Cell::new(0)));
        let capture = once.take().unwrap();
        drop(once);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        drop(capture);
        assert_eq!(drops.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn once_reentrant_take() {
        let once = Arc::<Once<Box<dyn FnOnce() + Send>>>::new_cyclic(|weak| {
            let weak = weak.clone();
            Once::new(Box::new(move || {
                assert!(weak.upgrade().unwrap().take().is_none());
            }) as Box<dyn FnOnce() + Send>)
        });
        once.take().unwrap()();
        assert!(once.take().is_none());
    }
}
