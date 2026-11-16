use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentTaskStatus {
    Running,
    Queued,
    Delivered,
    Cancelled,
}

pub(super) struct TaskControl {
    status: AtomicU8,
    timer: Mutex<Option<Arc<TimerRegistration>>>,
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
    pub(super) fn set_timer(&self, timer: Arc<TimerRegistration>) {
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

    pub(super) fn status(&self) -> ComponentTaskStatus {
        match self.status.load(Ordering::Acquire) {
            0 => ComponentTaskStatus::Running,
            1 => ComponentTaskStatus::Queued,
            2 => ComponentTaskStatus::Delivered,
            3 => ComponentTaskStatus::Cancelled,
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
}

#[must_use = "dropping the timer cancels message delivery"]
pub struct ComponentTimer {
    pub(super) task: ComponentTask,
}

impl ComponentTimer {
    pub fn cancel(&self) {
        self.task.cancel();
    }
}

impl Drop for ComponentTimer {
    fn drop(&mut self) {
        self.cancel();
    }
}
