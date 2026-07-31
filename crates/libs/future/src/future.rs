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
    waker: Option<Arc<Mutex<Waker>>>,
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
            let mut guard = shared_waker.lock().unwrap();
            guard.clone_from(cx.waker());

            // Completion may have signaled the previous waker before it was replaced.
            if self.status.Status()? != AsyncStatus::Started {
                return Poll::Ready(self.inner.get_results());
            }
        } else {
            let shared_waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(shared_waker.clone());

            // The handler runs even if the operation completed before registration.
            self.inner.set_completed(move |_| {
                shared_waker.lock().unwrap().wake_by_ref();
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
