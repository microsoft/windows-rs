#![cfg(feature = "std")]

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
};

/// Wraps an `IAsyncOperation`, `IAsyncOperationWithProgress`, `IAsyncAction`, or `IAsyncActionWithProgress`.
/// Impls for this trait are generated automatically by windows-bindgen.
pub trait AsyncOperation {
    /// The type produced when the operation finishes.
    type Output;
    /// Returns whether the operation is finished, in which case `self.get_results()` can be used to get the returned data.
    /// Wraps `self.Status() != AsyncStatus::Started`.
    fn is_complete(&self) -> crate::Result<bool>;
    /// Register a callback that will be called once the operation is finished.
    /// This can only be called once.
    /// Wraps `self.SetCompleted(f)`.
    fn set_completed(&self, f: impl Fn() + Send + 'static) -> crate::Result<()>;
    /// Get the result value from a completed operation.
    /// Wraps `self.GetResults()`.
    fn get_results(&self) -> crate::Result<Self::Output>;
}

/// A wrapper around an `AsyncOperation` that implements `std::future::Future`.
/// This is used by generated `IntoFuture` impls. It shouldn't be necessary to use this type manually.
pub struct FutureWrapper<T> {
    inner: T,
    waker: Option<Arc<Mutex<Waker>>>,
}

impl<T> FutureWrapper<T> {
    /// Creates a `FutureWrapper`, which implements `std::future::Future`.
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            waker: None,
        }
    }
}

impl<T> Unpin for FutureWrapper<T> {}

impl<T: AsyncOperation> Future for FutureWrapper<T> {
    type Output = crate::Result<T::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if self.inner.is_complete()? {
            Poll::Ready(self.inner.get_results())
        } else {
            if let Some(saved_waker) = &self.waker {
                // Update the saved waker, in case the future has been transferred to a different executor.
                // (e.g. if using `select`.)
                let mut saved_waker = saved_waker.lock().unwrap();
                saved_waker.clone_from(cx.waker());
            } else {
                let saved_waker = Arc::new(Mutex::new(cx.waker().clone()));
                self.waker = Some(saved_waker.clone());
                self.inner.set_completed(move || {
                    saved_waker.lock().unwrap().wake_by_ref();
                })?;
            }
            Poll::Pending
        }
    }
}
