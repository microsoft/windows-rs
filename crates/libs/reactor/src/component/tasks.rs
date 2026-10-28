use super::*;

pub trait ComponentTimerRegistration: Send + Sync {
    fn cancel(&self);
}

pub trait ComponentServices: Send + Sync {
    fn spawn_background(&self, work: Box<dyn FnOnce() + Send>);

    fn set_timeout(
        &self,
        delay: Duration,
        callback: Box<dyn FnOnce() + Send>,
    ) -> Arc<dyn ComponentTimerRegistration>;
}

pub(crate) trait ComponentUiServices {
    fn open_window(&self, root: ComponentNode, policy: WindowPolicy) -> bool;

    fn publish_window(&self, _publication: WindowPublication) {}

    fn activate_window(&self) -> bool {
        false
    }

    fn close_window(&self) -> bool {
        false
    }

    fn run_window(&self, _operation: Box<dyn FnOnce(*mut core::ffi::c_void)>) -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentTaskStatus {
    Running,
    Queued,
    Delivered,
    Cancelled,
    Rejected,
}

pub(super) struct TaskControl {
    status: AtomicU8,
    timer: Mutex<Option<Arc<dyn ComponentTimerRegistration>>>,
}

impl Default for TaskControl {
    fn default() -> Self {
        Self {
            status: AtomicU8::new(0),
            timer: Mutex::new(None),
        }
    }
}

impl TaskControl {
    pub(super) fn set_timer(&self, timer: Arc<dyn ComponentTimerRegistration>) {
        let mut current = self.timer.lock().unwrap();
        if self.status() == ComponentTaskStatus::Running {
            *current = Some(timer);
        } else {
            drop(current);
            timer.cancel();
        }
    }

    pub(super) fn timer_fired(&self) {
        self.timer.lock().unwrap().take();
    }

    pub(super) fn queue(&self) -> bool {
        self.status
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    pub(super) fn deliver(&self) -> bool {
        self.status
            .compare_exchange(1, 2, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    pub(super) fn cancel(&self) {
        let mut current = self.status.load(Ordering::Acquire);
        while current <= 1 {
            match self
                .status
                .compare_exchange(current, 3, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => {
                    if let Some(timer) = self.timer.lock().unwrap().take() {
                        timer.cancel();
                    }
                    return;
                }
                Err(actual) => current = actual,
            }
        }
    }

    pub(super) fn reject(&self) {
        let _ = self
            .status
            .compare_exchange(1, 4, Ordering::AcqRel, Ordering::Acquire);
    }

    pub(super) fn status(&self) -> ComponentTaskStatus {
        match self.status.load(Ordering::Acquire) {
            0 => ComponentTaskStatus::Running,
            1 => ComponentTaskStatus::Queued,
            2 => ComponentTaskStatus::Delivered,
            3 => ComponentTaskStatus::Cancelled,
            4 => ComponentTaskStatus::Rejected,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct CancellationToken {
    pub(super) control: Arc<TaskControl>,
}

impl CancellationToken {
    pub fn is_cancelled(&self) -> bool {
        self.control.status() == ComponentTaskStatus::Cancelled
    }
}

#[derive(Clone)]
pub struct ComponentTask {
    pub(super) control: Arc<TaskControl>,
}

impl ComponentTask {
    pub fn cancel(&self) {
        self.control.cancel();
    }

    /// Returns whether the result message was rejected because the component queue was full
    /// or closed. Rejection is terminal; the result is not retried.
    ///
    /// This is a snapshot, not a notification. `false` does not imply successful delivery:
    /// the task may still be running, queued, delivered, or cancelled.
    pub fn is_rejected(&self) -> bool {
        self.control.status() == ComponentTaskStatus::Rejected
    }
}

#[must_use = "dropping the timer cancels message delivery"]
pub struct ComponentTimer {
    pub(super) task: ComponentTask,
}

impl ComponentTimer {
    pub fn cancel(&self) {
        self.task.cancel();
    }

    /// Returns whether the timer message was rejected because the component queue was full
    /// or closed. Rejection is terminal; the message is not retried.
    ///
    /// This is a snapshot, not a notification. `false` does not imply successful delivery:
    /// the timer may still be waiting, queued, delivered, or cancelled.
    pub fn is_rejected(&self) -> bool {
        self.task.is_rejected()
    }
}

impl Drop for ComponentTimer {
    fn drop(&mut self) {
        self.cancel();
    }
}
