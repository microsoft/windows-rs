use super::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ComponentHostState {
    pub live_scopes: usize,
    pub scope_slots: usize,
    pub free_scopes: usize,
    pub effects: usize,
    pub tasks: usize,
    pub contexts: usize,
    pub context_consumers: usize,
    pub virtual_rows: usize,
    pub queued_messages: usize,
    pub queue_closed: bool,
    pub graph_objects: usize,
    pub retirements: usize,
}

impl<A: Adapter> ComponentHost<A> {
    pub fn test_adapter_mut(&mut self) -> &mut A {
        self.runtime.adapter_mut()
    }

    pub fn test_state(&self) -> ComponentHostState {
        let queue = self.queue.lock().unwrap();
        ComponentHostState {
            live_scopes: self
                .scopes
                .iter()
                .filter(|slot| slot.scope.is_some())
                .count(),
            scope_slots: self.scopes.len(),
            free_scopes: self.free_scopes.len(),
            effects: self
                .scopes
                .iter()
                .filter_map(|slot| slot.scope.as_ref())
                .map(|scope| scope.effects.len())
                .sum(),
            tasks: self
                .scopes
                .iter()
                .filter_map(|slot| slot.scope.as_ref())
                .map(|scope| scope.component.tracked_tasks())
                .sum(),
            contexts: self.contexts.len(),
            context_consumers: self.context_consumers.values().map(HashSet::len).sum(),
            virtual_rows: self.virtual_rows.len(),
            queued_messages: queue.messages.len() + self.local_queue.borrow().len(),
            queue_closed: queue.closed,
            graph_objects: self.runtime.graph().object_count(),
            retirements: self.runtime.graph().retired_count(),
        }
    }
}
