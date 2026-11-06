use super::*;
use std::future::{Future, IntoFuture};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

/// Adapts a WinRT asynchronous operation to [`Future`].
pub struct AsyncFuture<A: Async> {
    inner: A,

    // Cached to avoid querying IAsyncInfo on every poll.
    status: IAsyncInfo,

    // The completion handler is fixed, so later polls replace this shared waker.
    waker: Option<Arc<AsyncWaker>>,
}

// Waker callbacks, including clone and drop, must run outside the lock.
struct AsyncWaker(Mutex<Option<Waker>>);

impl AsyncWaker {
    fn new(waker: &Waker) -> Self {
        Self(Mutex::new(Some(waker.clone())))
    }

    fn register(&self, waker: &Waker) {
        if self
            .0
            .lock()
            .unwrap()
            .as_ref()
            .is_some_and(|current| current.will_wake(waker))
        {
            return;
        }
        let waker = waker.clone();
        let previous = self.0.lock().unwrap().replace(waker);
        drop(previous);
    }

    fn wake(&self) {
        let waker = self.0.lock().unwrap().take();
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

impl<A: Async> AsyncFuture<A> {
    fn new(inner: A) -> Self {
        Self {
            // All four async interfaces implement `IAsyncInfo` so this `cast` will always succeed.
            status: inner.cast().unwrap(),
            inner,
            waker: None,
        }
    }
}

unsafe impl<A: Async> Send for AsyncFuture<A> {}
unsafe impl<A: Async> Sync for AsyncFuture<A> {}
impl<A: Async> Unpin for AsyncFuture<A> {}

impl<A: Async> Future for AsyncFuture<A> {
    type Output = Result<A::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // WinRT operations start eagerly, so any state other than Started has a result.
        if self.status.Status()? != AsyncStatus::Started {
            return Poll::Ready(self.inner.get_results());
        }

        if let Some(shared_waker) = &self.waker {
            shared_waker.register(cx.waker());

            // Completion may have signaled the previous waker before it was replaced.
            if self.status.Status()? != AsyncStatus::Started {
                return Poll::Ready(self.inner.get_results());
            }
        } else {
            let shared_waker = Arc::new(AsyncWaker::new(cx.waker()));
            self.waker = Some(shared_waker.clone());

            // The handler runs even if the operation completed before registration.
            self.inner.set_completed(move |_| {
                shared_waker.wake();
            })?;
        };

        Poll::Pending
    }
}

impl IntoFuture for IAsyncAction {
    type Output = Result<()>;
    type IntoFuture = AsyncFuture<Self>;

    fn into_future(self) -> Self::IntoFuture {
        AsyncFuture::new(self)
    }
}

impl<T: RuntimeType> IntoFuture for IAsyncOperation<T> {
    type Output = Result<T>;
    type IntoFuture = AsyncFuture<Self>;

    fn into_future(self) -> Self::IntoFuture {
        AsyncFuture::new(self)
    }
}

impl<P: RuntimeType> IntoFuture for IAsyncActionWithProgress<P> {
    type Output = Result<()>;
    type IntoFuture = AsyncFuture<Self>;

    fn into_future(self) -> Self::IntoFuture {
        AsyncFuture::new(self)
    }
}

impl<T: RuntimeType, P: RuntimeType> IntoFuture for IAsyncOperationWithProgress<T, P> {
    type Output = Result<T>;
    type IntoFuture = AsyncFuture<Self>;

    fn into_future(self) -> Self::IntoFuture {
        AsyncFuture::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::{
            Barrier, OnceLock, Weak,
            atomic::{AtomicUsize, Ordering},
        },
        task::Wake,
    };

    struct ReentrantWake {
        registration: OnceLock<Weak<AsyncWaker>>,
        calls: AtomicUsize,
    }

    impl Wake for ReentrantWake {
        fn wake(self: Arc<Self>) {
            self.wake_by_ref();
        }

        fn wake_by_ref(self: &Arc<Self>) {
            let registration = self.registration.get().unwrap().upgrade().unwrap();
            assert!(
                registration.0.try_lock().is_ok(),
                "wake held the waker lock"
            );
            registration.register(&Waker::from(Arc::clone(self)));
            self.calls.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn waker_can_register_during_wake() {
        let wake = Arc::new(ReentrantWake {
            registration: OnceLock::new(),
            calls: AtomicUsize::new(0),
        });
        let registration = Arc::new(AsyncWaker::new(&Waker::from(wake.clone())));
        wake.registration
            .set(Arc::downgrade(&registration))
            .unwrap();
        registration.wake();
        registration.wake();
        assert_eq!(wake.calls.load(Ordering::Relaxed), 2);
    }

    struct ReentrantDrop {
        registration: Arc<OnceLock<Weak<AsyncWaker>>>,
        drops: Arc<AtomicUsize>,
    }

    impl Wake for ReentrantDrop {
        fn wake(self: Arc<Self>) {
            panic!("replaced waker must not be woken");
        }
    }

    impl Drop for ReentrantDrop {
        fn drop(&mut self) {
            let registration = self.registration.get().unwrap().upgrade().unwrap();
            assert!(
                registration.0.try_lock().is_ok(),
                "drop held the waker lock"
            );
            registration.register(Waker::noop());
            self.drops.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn waker_can_register_during_replacement_drop() {
        let slot = Arc::new(OnceLock::new());
        let drops = Arc::new(AtomicUsize::new(0));
        let waker = Waker::from(Arc::new(ReentrantDrop {
            registration: slot.clone(),
            drops: drops.clone(),
        }));
        let registration = Arc::new(AsyncWaker::new(&waker));
        slot.set(Arc::downgrade(&registration)).unwrap();
        drop(waker);
        registration.register(Waker::noop());
        assert_eq!(drops.load(Ordering::Relaxed), 1);
    }

    #[derive(Default)]
    struct CountWake(AtomicUsize);

    impl Wake for CountWake {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn waker_replacement_and_completion() {
        let first = Arc::new(CountWake::default());
        let second = Arc::new(CountWake::default());
        let registration = AsyncWaker::new(&Waker::from(first.clone()));
        registration.register(&Waker::from(second.clone()));
        assert_eq!(Arc::strong_count(&first), 1);
        registration.wake();
        registration.wake();
        assert_eq!(first.0.load(Ordering::Relaxed), 0);
        assert_eq!(second.0.load(Ordering::Relaxed), 1);
        assert_eq!(Arc::strong_count(&second), 1);
    }

    #[test]
    fn waker_completion_races_registration() {
        let first = Arc::new(CountWake::default());
        let second = Arc::new(CountWake::default());
        let registration = AsyncWaker::new(&Waker::from(first.clone()));
        let barrier = Barrier::new(2);
        std::thread::scope(|scope| {
            scope.spawn(|| {
                barrier.wait();
                registration.wake();
            });
            barrier.wait();
            registration.register(&Waker::from(second.clone()));
        });
        registration.wake();
        assert!(first.0.load(Ordering::Relaxed) <= 1);
        assert_eq!(second.0.load(Ordering::Relaxed), 1);
        assert_eq!(Arc::strong_count(&first), 1);
        assert_eq!(Arc::strong_count(&second), 1);
    }
}
