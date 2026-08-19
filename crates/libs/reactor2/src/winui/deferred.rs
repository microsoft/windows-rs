use super::*;

pub(super) struct DeferredUpdate {
    pub(super) active: Rc<Cell<bool>>,
    pub(super) revision: Rc<Cell<u64>>,
}

impl DeferredUpdate {
    pub(super) fn new() -> Self {
        Self {
            active: Rc::new(Cell::new(true)),
            revision: Rc::new(Cell::new(0)),
        }
    }
}

impl Drop for DeferredUpdate {
    fn drop(&mut self) {
        self.active.set(false);
        self.revision.set(self.revision.get().wrapping_add(1));
    }
}

impl WinUiRuntime {
    pub(super) fn enqueue_deferred_ready(
        &self,
        target: NodeId,
        revision: u64,
        action: DeferredAction,
        active: Rc<Cell<bool>>,
        current_revision: Rc<Cell<u64>>,
        rejected: &'static str,
    ) -> WindowsResult<()> {
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let handler = bindings::DispatcherQueueHandler::new(move || {
            if !active.get() || current_revision.get() != revision {
                return;
            }
            events.borrow_mut().push_back(NativeEvent::DeferredReady {
                target,
                revision,
                action,
            });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        });
        if self
            .dispatcher
            .TryEnqueueWithPriority(bindings::DispatcherQueuePriority::Low, &handler)?
        {
            Ok(())
        } else {
            panic!("{rejected}")
        }
    }
}
