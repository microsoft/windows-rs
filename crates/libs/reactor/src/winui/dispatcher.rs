use std::sync::Arc;

use super::*;
use crate::bindings::*;

/// [`Dispatcher`] backed by the WinUI thread's `DispatcherQueue`.
pub struct WinUIDispatcher {
    queue: DispatcherQueue,
    /// Agile handle used to satisfy [`SendDispatcher`] from any thread.
    send_handle: Arc<AgileDispatcherQueue>,
}

impl WinUIDispatcher {
    pub fn for_current_thread() -> windows_core::Result<Self> {
        let queue = DispatcherQueue::GetForCurrentThread()?;
        let send_handle = Arc::new(AgileDispatcherQueue {
            queue: queue.clone(),
        });
        Ok(Self { queue, send_handle })
    }

    pub fn queue(&self) -> &DispatcherQueue {
        &self.queue
    }

    /// Thread-safe handle to this dispatcher.
    pub fn send_handle(&self) -> Arc<dyn SendDispatcher> {
        Arc::clone(&self.send_handle) as Arc<dyn SendDispatcher>
    }

    /// Build a [`UiMarshaller`] backed by this dispatcher.
    pub fn marshaller(&self) -> UiMarshaller {
        UiMarshaller::new(self.send_handle())
    }
}

impl Dispatcher for WinUIDispatcher {
    fn enqueue(&self, priority: DispatchPriority, f: Box<dyn FnOnce()>) -> bool {
        let slot = std::cell::RefCell::new(Some(f));
        let handler = DispatcherQueueHandler::new(move || {
            if let Some(f) = slot.borrow_mut().take() {
                f();
            }
        });
        match priority {
            DispatchPriority::Normal => self.queue.TryEnqueue(&handler).unwrap_or(false),
            DispatchPriority::Low => self
                .queue
                .TryEnqueueWithPriority(DispatcherQueuePriority::Low, &handler)
                .unwrap_or(false),
        }
    }
}

/// Wrapper around an agile `DispatcherQueue` so closures can be posted
/// across threads. `DispatcherQueue` implements `IAgileObject`; its
/// `TryEnqueue` is callable from any thread.
struct AgileDispatcherQueue {
    queue: DispatcherQueue,
}

// SAFETY: DispatcherQueue is an agile COM object and TryEnqueue is
// documented as callable from any thread.
unsafe impl Send for AgileDispatcherQueue {}
unsafe impl Sync for AgileDispatcherQueue {}

impl SendDispatcher for AgileDispatcherQueue {
    fn enqueue_send(
        &self,
        priority: DispatchPriority,
        f: Box<dyn FnOnce() + Send + 'static>,
    ) -> bool {
        let slot = std::sync::Mutex::new(Some(f));
        let handler = DispatcherQueueHandler::new(move || {
            if let Some(f) = slot.lock().unwrap().take() {
                f();
            }
        });
        match priority {
            DispatchPriority::Normal => self.queue.TryEnqueue(&handler).unwrap_or(false),
            DispatchPriority::Low => self
                .queue
                .TryEnqueueWithPriority(DispatcherQueuePriority::Low, &handler)
                .unwrap_or(false),
        }
    }
}
