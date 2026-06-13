use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub use crate::bindings::DispatcherQueuePriority;

/// Schedules deferred work onto a render-aware queue.
pub trait Dispatcher {
    fn enqueue(&self, priority: DispatcherQueuePriority, f: Box<dyn FnOnce()>) -> bool;
}

/// Thread-safe variant of [`Dispatcher`] accepting `Send` closures.
/// Implementations marshal the closure onto the UI thread.
pub trait SendDispatcher: Send + Sync + 'static {
    fn enqueue_send(
        &self,
        priority: DispatcherQueuePriority,
        f: Box<dyn FnOnce() + Send + 'static>,
    ) -> bool;
}

thread_local! {
    // UI thread's rerender hook, installed by `RenderHost::set_marshaller`.
    // Single-host-per-thread; replace with a per-host registry if
    // multi-host-per-thread is added.
    static UI_RERENDER: RefCell<Option<Rc<dyn Fn()>>> = const { RefCell::new(None) };
}

/// Install (or clear) the UI thread's rerender hook.
pub fn set_ui_rerender(rerender: Option<Rc<dyn Fn()>>) {
    UI_RERENDER.with(|r| {
        *r.borrow_mut() = rerender;
    });
}

/// Request a rerender on the UI thread the marshaller targets.
pub fn request_ui_rerender_on_ui_thread() {
    UI_RERENDER.with(|r| {
        if let Some(rr) = r.borrow().as_ref() {
            rr();
        }
    });
}

/// RAII guard around `set_ui_rerender`; clears the thread-local on drop.
#[must_use = "the guard restores UI_RERENDER on drop; binding it to `_` drops it immediately"]
pub struct UiRerenderGuard {
    _not_send: std::marker::PhantomData<*const ()>,
}

impl UiRerenderGuard {
    pub fn install(rerender: Rc<dyn Fn()>) -> Self {
        set_ui_rerender(Some(rerender));
        Self {
            _not_send: std::marker::PhantomData,
        }
    }
}

impl Drop for UiRerenderGuard {
    fn drop(&mut self) {
        set_ui_rerender(None);
    }
}

/// Thread-safe, clonable handle to the UI thread's render-aware
/// dispatcher. Used by `AsyncSetState` to marshal state writes back
/// onto the UI thread.
#[derive(Clone)]
pub struct UiMarshaller {
    inner: Arc<dyn SendDispatcher>,
}

impl UiMarshaller {
    pub fn new(inner: Arc<dyn SendDispatcher>) -> Self {
        Self { inner }
    }

    /// Schedule `f` to run on the UI thread at normal priority.
    pub fn dispatch<F>(&self, f: F) -> bool
    where
        F: FnOnce() + Send + 'static,
    {
        self.inner
            .enqueue_send(DispatcherQueuePriority::Normal, Box::new(f))
    }

    /// Schedule `f` to run on the UI thread at low priority.
    pub fn dispatch_low<F>(&self, f: F) -> bool
    where
        F: FnOnce() + Send + 'static,
    {
        self.inner
            .enqueue_send(DispatcherQueuePriority::Low, Box::new(f))
    }
}

impl std::fmt::Debug for UiMarshaller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UiMarshaller").finish_non_exhaustive()
    }
}

/// In-memory [`SendDispatcher`] used by tests. Closures are queued in a
/// mutex-guarded `VecDeque`; call [`Self::drain`] to run them.
#[derive(Default)]
pub struct ChannelDispatcher {
    inner: Arc<ChannelDispatcherInner>,
}

#[derive(Default)]
struct ChannelDispatcherInner {
    normal: Mutex<VecDeque<Box<dyn FnOnce() + Send>>>,
    low: Mutex<VecDeque<Box<dyn FnOnce() + Send>>>,
}

impl ChannelDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pending(&self) -> usize {
        self.inner.normal.lock().unwrap().len() + self.inner.low.lock().unwrap().len()
    }

    pub fn drain(&self) -> usize {
        let mut fired = 0;
        loop {
            let item = {
                let mut n = self.inner.normal.lock().unwrap();
                if let Some(f) = n.pop_front() {
                    Some(f)
                } else {
                    drop(n);
                    self.inner.low.lock().unwrap().pop_front()
                }
            };
            match item {
                Some(f) => {
                    f();
                    fired += 1;
                }
                None => break,
            }
        }
        fired
    }

    pub fn handle(&self) -> Arc<dyn SendDispatcher> {
        Arc::clone(&self.inner) as Arc<dyn SendDispatcher>
    }

    pub fn marshaller(&self) -> UiMarshaller {
        UiMarshaller::new(self.handle())
    }
}

impl SendDispatcher for ChannelDispatcherInner {
    fn enqueue_send(
        &self,
        priority: DispatcherQueuePriority,
        f: Box<dyn FnOnce() + Send + 'static>,
    ) -> bool {
        match priority {
            DispatcherQueuePriority::Low => self.low.lock().unwrap().push_back(f),
            _ => self.normal.lock().unwrap().push_back(f),
        }
        true
    }
}

/// In-process [`Dispatcher`] that buffers work until [`drain`](Self::drain)
/// is called. Used by tests and by `Application::run_once`.
#[derive(Default)]
pub struct RunOnDemandDispatcher {
    inner: Rc<DispatcherQueue>,
}

#[derive(Default)]
struct DispatcherQueue {
    normal: RefCell<VecDeque<Box<dyn FnOnce()>>>,
    low: RefCell<VecDeque<Box<dyn FnOnce()>>>,
}

impl RunOnDemandDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pending(&self) -> usize {
        self.inner.normal.borrow().len() + self.inner.low.borrow().len()
    }

    pub fn drain(&self) -> usize {
        let mut fired = 0;
        loop {
            let item = {
                let mut n = self.inner.normal.borrow_mut();
                if let Some(f) = n.pop_front() {
                    Some(f)
                } else {
                    drop(n);
                    self.inner.low.borrow_mut().pop_front()
                }
            };
            match item {
                Some(f) => {
                    f();
                    fired += 1;
                }
                None => break,
            }
        }
        fired
    }
}

impl Dispatcher for RunOnDemandDispatcher {
    fn enqueue(&self, priority: DispatcherQueuePriority, f: Box<dyn FnOnce()>) -> bool {
        match priority {
            DispatcherQueuePriority::Low => self.inner.low.borrow_mut().push_back(f),
            _ => self.inner.normal.borrow_mut().push_back(f),
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn job(log: Rc<RefCell<Vec<&'static str>>>, tag: &'static str) -> Box<dyn FnOnce()> {
        Box::new(move || log.borrow_mut().push(tag))
    }

    // FIFO/priority tests for `RunOnDemandDispatcher` live in
    // `crates/tests/reactor/tests/dispatcher.rs`; this one stays here
    // because it pokes the private `inner` field directly.

    #[test]
    fn run_on_demand_dispatcher_promotes_normal_followup_over_remaining_low() {
        let dispatcher = RunOnDemandDispatcher::new();
        let log: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));

        let log_for_lo1 = Rc::clone(&log);
        let dispatcher_clone = dispatcher.inner.clone();
        let lo1: Box<dyn FnOnce()> = Box::new(move || {
            log_for_lo1.borrow_mut().push("lo1");

            let log2 = Rc::clone(&log_for_lo1);
            dispatcher_clone
                .normal
                .borrow_mut()
                .push_back(Box::new(move || log2.borrow_mut().push("n_followup")));
        });

        dispatcher.enqueue(DispatcherQueuePriority::Low, lo1);
        dispatcher.enqueue(DispatcherQueuePriority::Low, job(Rc::clone(&log), "lo2"));

        dispatcher.drain();
        assert_eq!(*log.borrow(), vec!["lo1", "n_followup", "lo2"]);
    }
}
