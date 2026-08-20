#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorkPriority {
    Low,
    Normal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleAction {
    None,
    Enqueue(WorkPriority),
    Closed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SchedulerPhase {
    Idle,
    Scheduled,
    Dispatching,
    Closing,
}

pub struct SchedulerState {
    pending: Option<WorkPriority>,
    phase: SchedulerPhase,
}

impl Default for SchedulerState {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedulerState {
    pub fn new() -> Self {
        Self {
            pending: None,
            phase: SchedulerPhase::Idle,
        }
    }

    pub fn request(&mut self, priority: WorkPriority) -> ScheduleAction {
        if self.phase == SchedulerPhase::Closing {
            return ScheduleAction::Closed;
        }
        self.pending = Some(
            self.pending
                .map_or(priority, |current| current.max(priority)),
        );
        if self.phase == SchedulerPhase::Idle {
            self.phase = SchedulerPhase::Scheduled;
            ScheduleAction::Enqueue(self.pending.unwrap())
        } else {
            ScheduleAction::None
        }
    }

    pub fn begin_dispatch(&mut self) -> bool {
        if self.phase != SchedulerPhase::Scheduled {
            return false;
        }
        self.phase = SchedulerPhase::Dispatching;
        self.pending = None;
        true
    }

    pub fn finish_dispatch(&mut self) -> ScheduleAction {
        if self.phase == SchedulerPhase::Closing {
            return ScheduleAction::Closed;
        }
        assert_eq!(self.phase, SchedulerPhase::Dispatching);
        self.phase = SchedulerPhase::Idle;
        if let Some(priority) = self.pending {
            self.phase = SchedulerPhase::Scheduled;
            ScheduleAction::Enqueue(priority)
        } else {
            ScheduleAction::None
        }
    }

    pub fn enqueue_failed(&mut self) {
        if self.phase == SchedulerPhase::Scheduled {
            self.phase = SchedulerPhase::Idle;
        }
    }

    pub fn close(&mut self) {
        self.pending = None;
        self.phase = SchedulerPhase::Closing;
    }

    pub fn open(&mut self) {
        self.pending = None;
        self.phase = SchedulerPhase::Idle;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coalesces_scheduled_work_at_the_highest_priority() {
        let mut state = SchedulerState::new();

        assert_eq!(
            state.request(WorkPriority::Low),
            ScheduleAction::Enqueue(WorkPriority::Low)
        );
        assert_eq!(state.request(WorkPriority::Normal), ScheduleAction::None);
        assert!(state.begin_dispatch());
        assert_eq!(state.finish_dispatch(), ScheduleAction::None);
    }

    #[test]
    fn reentrant_work_is_rearmed_after_dispatch() {
        let mut state = SchedulerState::new();
        assert!(matches!(
            state.request(WorkPriority::Normal),
            ScheduleAction::Enqueue(_)
        ));
        assert!(state.begin_dispatch());

        assert_eq!(state.request(WorkPriority::Low), ScheduleAction::None);
        assert_eq!(
            state.finish_dispatch(),
            ScheduleAction::Enqueue(WorkPriority::Low)
        );
    }

    #[test]
    fn enqueue_failure_retains_pending_work_for_a_later_request() {
        let mut state = SchedulerState::new();
        assert!(matches!(
            state.request(WorkPriority::Low),
            ScheduleAction::Enqueue(_)
        ));

        state.enqueue_failed();

        assert_eq!(
            state.request(WorkPriority::Normal),
            ScheduleAction::Enqueue(WorkPriority::Normal)
        );
    }

    #[test]
    fn close_rejects_and_discards_work_until_opened() {
        let mut state = SchedulerState::new();
        state.close();

        assert_eq!(state.request(WorkPriority::Normal), ScheduleAction::Closed);

        state.open();
        assert_eq!(
            state.request(WorkPriority::Normal),
            ScheduleAction::Enqueue(WorkPriority::Normal)
        );
    }
}
