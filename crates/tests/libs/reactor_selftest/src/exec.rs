use std::cell::UnsafeCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake};
use std::thread::ThreadId;

use windows::core::Result;

use crate::bindings::{DispatcherQueue, DispatcherQueueHandler, DispatcherQueuePriority};

pub(crate) struct Task {
    future: UnsafeCell<Option<Pin<Box<dyn Future<Output = ()>>>>>,
    dispatcher: DispatcherQueue,
    origin: ThreadId,
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Task {
    fn assert_thread(&self) {
        assert_eq!(
            std::thread::current().id(),
            self.origin,
            "selftest-host exec::Task accessed off the origin thread"
        );
    }

    fn poll(self: &Arc<Self>) {
        self.assert_thread();
        let waker = Arc::clone(self).into();
        let mut cx = Context::from_waker(&waker);

        let slot = unsafe { &mut *self.future.get() };
        if let Some(fut) = slot.as_mut()
            && fut.as_mut().poll(&mut cx).is_ready()
        {
            *slot = None;
        }
    }
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        Self::wake_by_ref(&self);
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let task = Arc::clone(self);

        let handler = DispatcherQueueHandler::new(move || {
            task.poll();
        });
        let _ = self.dispatcher.TryEnqueue(&handler);
    }
}

pub(crate) fn spawn_root<F>(future: F, dispatcher: DispatcherQueue)
where
    F: Future<Output = ()> + 'static,
{
    let task = Arc::new(Task {
        future: UnsafeCell::new(Some(Box::pin(future))),
        dispatcher,
        origin: std::thread::current().id(),
    });
    task.poll();
}

pub(crate) struct YieldLow {
    dispatcher: DispatcherQueue,
    state: YieldState,
}

enum YieldState {
    Idle,
    Enqueued,
    Fired,
}

impl YieldLow {
    pub(crate) fn new(dispatcher: DispatcherQueue) -> Self {
        Self {
            dispatcher,
            state: YieldState::Idle,
        }
    }
}

impl Future for YieldLow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        match self.state {
            YieldState::Fired => Poll::Ready(()),
            YieldState::Enqueued => Poll::Pending,
            YieldState::Idle => {
                let waker = cx.waker().clone();

                let state_ptr = StateHandle::new(&mut self.state);
                let waker_clone = waker;
                let handler = DispatcherQueueHandler::new(move || {
                    state_ptr.set_fired();
                    waker_clone.wake_by_ref();
                });
                let r: Result<bool> = self
                    .dispatcher
                    .TryEnqueueWithPriority(DispatcherQueuePriority::Low, &handler);
                if let Ok(true) = r {
                    self.state = YieldState::Enqueued;
                    Poll::Pending
                } else {
                    self.state = YieldState::Fired;
                    Poll::Ready(())
                }
            }
        }
    }
}

struct StateHandle {
    ptr: *mut YieldState,
}

unsafe impl Send for StateHandle {}
unsafe impl Sync for StateHandle {}

impl StateHandle {
    fn new(state: &mut YieldState) -> Self {
        Self {
            ptr: state as *mut YieldState,
        }
    }
    fn set_fired(&self) {
        unsafe { *self.ptr = YieldState::Fired }
    }
}
