#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorkPriority {
    Low,
    Normal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScheduleTicket {
    pub generation: u64,
    pub priority: WorkPriority,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleAction {
    None,
    Enqueue(ScheduleTicket),
    Closed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SchedulerPhase {
    Idle,
    Scheduled(ScheduleTicket),
    Dispatching,
    Closing,
}

pub struct SchedulerState {
    generation: u64,
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
            generation: 0,
            pending: None,
            phase: SchedulerPhase::Idle,
        }
    }

    pub fn request(&mut self, priority: WorkPriority) -> ScheduleAction {
        if matches!(self.phase, SchedulerPhase::Closing) {
            return ScheduleAction::Closed;
        }
        self.pending = Some(
            self.pending
                .map_or(priority, |current| current.max(priority)),
        );
        let pending = self.pending.unwrap();
        match self.phase {
            SchedulerPhase::Idle => self.schedule(pending),
            SchedulerPhase::Scheduled(scheduled) if pending > scheduled.priority => {
                self.schedule(pending)
            }
            SchedulerPhase::Scheduled(_) | SchedulerPhase::Dispatching => ScheduleAction::None,
            SchedulerPhase::Closing => ScheduleAction::Closed,
        }
    }

    pub fn begin_dispatch(&mut self, ticket: ScheduleTicket) -> bool {
        if self.phase != SchedulerPhase::Scheduled(ticket) {
            return false;
        }
        self.phase = SchedulerPhase::Dispatching;
        self.pending = None;
        true
    }

    pub fn finish_dispatch(&mut self) -> ScheduleAction {
        if matches!(self.phase, SchedulerPhase::Closing) {
            return ScheduleAction::Closed;
        }
        assert_eq!(self.phase, SchedulerPhase::Dispatching);
        self.phase = SchedulerPhase::Idle;
        if let Some(priority) = self.pending {
            self.schedule(priority)
        } else {
            ScheduleAction::None
        }
    }

    pub fn enqueue_failed(&mut self, ticket: ScheduleTicket) {
        if self.phase == SchedulerPhase::Scheduled(ticket) {
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

    fn schedule(&mut self, priority: WorkPriority) -> ScheduleAction {
        self.generation = self
            .generation
            .checked_add(1)
            .expect("scheduler generation exhausted");
        let ticket = ScheduleTicket {
            generation: self.generation,
            priority,
        };
        self.phase = SchedulerPhase::Scheduled(ticket);
        ScheduleAction::Enqueue(ticket)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_work_replaces_a_scheduled_low_priority_callback() {
        let mut state = SchedulerState::new();
        let ScheduleAction::Enqueue(low) = state.request(WorkPriority::Low) else {
            panic!("expected enqueue");
        };
        let ScheduleAction::Enqueue(normal) = state.request(WorkPriority::Normal) else {
            panic!("expected priority upgrade");
        };

        assert_eq!(low.priority, WorkPriority::Low);
        assert_eq!(normal.priority, WorkPriority::Normal);
        assert!(!state.begin_dispatch(low));
        assert!(state.begin_dispatch(normal));
        assert_eq!(state.finish_dispatch(), ScheduleAction::None);
    }

    #[test]
    fn same_priority_scheduled_work_is_coalesced() {
        let mut state = SchedulerState::new();
        let ScheduleAction::Enqueue(ticket) = state.request(WorkPriority::Normal) else {
            panic!("expected enqueue");
        };

        assert_eq!(state.request(WorkPriority::Normal), ScheduleAction::None);
        assert!(state.begin_dispatch(ticket));
        assert_eq!(state.finish_dispatch(), ScheduleAction::None);
    }

    #[test]
    fn reentrant_work_is_rearmed_after_dispatch() {
        let mut state = SchedulerState::new();
        let ScheduleAction::Enqueue(ticket) = state.request(WorkPriority::Normal) else {
            panic!("expected enqueue");
        };
        assert!(state.begin_dispatch(ticket));

        assert_eq!(state.request(WorkPriority::Low), ScheduleAction::None);
        let ScheduleAction::Enqueue(rearmed) = state.finish_dispatch() else {
            panic!("expected rearm");
        };
        assert_eq!(rearmed.priority, WorkPriority::Low);
    }

    #[test]
    fn enqueue_failure_retains_pending_work_for_a_later_request() {
        let mut state = SchedulerState::new();
        let ScheduleAction::Enqueue(ticket) = state.request(WorkPriority::Low) else {
            panic!("expected enqueue");
        };

        state.enqueue_failed(ticket);

        let ScheduleAction::Enqueue(retry) = state.request(WorkPriority::Normal) else {
            panic!("expected retry");
        };
        assert_eq!(retry.priority, WorkPriority::Normal);
    }

    #[test]
    fn lower_priority_request_does_not_downgrade_retained_work() {
        let mut state = SchedulerState::new();
        let ScheduleAction::Enqueue(ticket) = state.request(WorkPriority::Normal) else {
            panic!("expected enqueue");
        };

        state.enqueue_failed(ticket);

        let ScheduleAction::Enqueue(retry) = state.request(WorkPriority::Low) else {
            panic!("expected retry");
        };
        assert_eq!(retry.priority, WorkPriority::Normal);
    }

    #[test]
    fn stale_enqueue_failure_does_not_cancel_replacement() {
        let mut state = SchedulerState::new();
        let ScheduleAction::Enqueue(low) = state.request(WorkPriority::Low) else {
            panic!("expected enqueue");
        };
        let ScheduleAction::Enqueue(normal) = state.request(WorkPriority::Normal) else {
            panic!("expected priority upgrade");
        };

        state.enqueue_failed(low);

        assert!(state.begin_dispatch(normal));
    }

    #[test]
    fn close_rejects_and_discards_work_until_opened() {
        let mut state = SchedulerState::new();
        state.close();

        assert_eq!(state.request(WorkPriority::Normal), ScheduleAction::Closed);

        state.open();
        assert!(matches!(
            state.request(WorkPriority::Normal),
            ScheduleAction::Enqueue(_)
        ));
    }
}
