#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScopeId {
    index: u32,
    generation: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScopeState {
    Reserved,
    Published,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScopeError {
    InvalidTransition(ScopeState, ScopeState),
    Stale(ScopeId),
}

struct ScopeSlot<T> {
    generation: u32,
    entry: Option<ScopeEntry<T>>,
}

struct ScopeEntry<T> {
    state: ScopeState,
    value: T,
}

pub struct ScopeArena<T> {
    slots: Vec<ScopeSlot<T>>,
    free: Vec<u32>,
    live: usize,
}

impl<T> Default for ScopeArena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ScopeArena<T> {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            free: Vec::new(),
            live: 0,
        }
    }

    #[cfg(test)]
    pub fn reserve(&mut self, value: T) -> ScopeId {
        self.reserve_with(|_| value)
    }

    pub fn reserve_with(&mut self, create: impl FnOnce(ScopeId) -> T) -> ScopeId {
        let id = if let Some(index) = self.free.pop() {
            let slot = &mut self.slots[index as usize];
            debug_assert!(slot.entry.is_none());
            ScopeId {
                index,
                generation: slot.generation,
            }
        } else {
            let index = u32::try_from(self.slots.len()).unwrap();
            self.slots.push(ScopeSlot {
                generation: 0,
                entry: None,
            });
            ScopeId {
                index,
                generation: 0,
            }
        };
        self.slots[id.index as usize].entry = Some(ScopeEntry {
            state: ScopeState::Reserved,
            value: create(id),
        });
        self.live += 1;
        id
    }

    pub fn publish(&mut self, id: ScopeId) -> Result<(), ScopeError> {
        self.transition(id, ScopeState::Reserved, ScopeState::Published)
    }

    pub fn state(&self, id: ScopeId) -> Result<ScopeState, ScopeError> {
        Ok(self.entry(id)?.state)
    }

    pub fn get(&self, id: ScopeId) -> Result<&T, ScopeError> {
        Ok(&self.entry(id)?.value)
    }

    pub fn get_mut(&mut self, id: ScopeId) -> Result<&mut T, ScopeError> {
        Ok(&mut self.entry_mut(id)?.value)
    }

    pub fn remove(&mut self, id: ScopeId) -> Result<T, ScopeError> {
        let slot = self.slot_mut(id)?;
        let entry = slot.entry.take().ok_or(ScopeError::Stale(id))?;
        if let Some(generation) = slot.generation.checked_add(1) {
            slot.generation = generation;
            self.free.push(id.index);
        }
        self.live -= 1;
        Ok(entry.value)
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.live == 0
    }

    fn transition(
        &mut self,
        id: ScopeId,
        from: ScopeState,
        to: ScopeState,
    ) -> Result<(), ScopeError> {
        let entry = self.entry_mut(id)?;
        if entry.state != from {
            return Err(ScopeError::InvalidTransition(entry.state, to));
        }
        entry.state = to;
        Ok(())
    }

    fn entry(&self, id: ScopeId) -> Result<&ScopeEntry<T>, ScopeError> {
        self.slot(id)?.entry.as_ref().ok_or(ScopeError::Stale(id))
    }

    fn entry_mut(&mut self, id: ScopeId) -> Result<&mut ScopeEntry<T>, ScopeError> {
        self.slot_mut(id)?
            .entry
            .as_mut()
            .ok_or(ScopeError::Stale(id))
    }

    fn slot(&self, id: ScopeId) -> Result<&ScopeSlot<T>, ScopeError> {
        let slot = self
            .slots
            .get(id.index as usize)
            .ok_or(ScopeError::Stale(id))?;
        if slot.generation == id.generation {
            Ok(slot)
        } else {
            Err(ScopeError::Stale(id))
        }
    }

    fn slot_mut(&mut self, id: ScopeId) -> Result<&mut ScopeSlot<T>, ScopeError> {
        let slot = self
            .slots
            .get_mut(id.index as usize)
            .ok_or(ScopeError::Stale(id))?;
        if slot.generation == id.generation {
            Ok(slot)
        } else {
            Err(ScopeError::Stale(id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    struct NonCloneState {
        drops: Rc<Cell<u32>>,
    }

    impl Drop for NonCloneState {
        fn drop(&mut self) {
            self.drops.set(self.drops.get() + 1);
        }
    }

    #[test]
    fn enforces_scope_lifecycle() {
        let mut arena = ScopeArena::new();
        let scope = arena.reserve("scope");

        assert_eq!(arena.state(scope), Ok(ScopeState::Reserved));
        arena.publish(scope).unwrap();
        assert_eq!(arena.state(scope), Ok(ScopeState::Published));
        assert_eq!(arena.remove(scope), Ok("scope"));
        assert!(arena.is_empty());
    }

    #[test]
    fn failed_reservation_can_be_removed_without_publication() {
        let drops = Rc::new(Cell::new(0));
        let mut arena = ScopeArena::new();
        let scope = arena.reserve(NonCloneState {
            drops: Rc::clone(&drops),
        });

        drop(arena.remove(scope).unwrap());

        assert_eq!(drops.get(), 1);
        assert_eq!(arena.state(scope), Err(ScopeError::Stale(scope)));
    }

    #[test]
    fn reused_slot_rejects_old_generation() {
        let mut arena = ScopeArena::new();
        let first = arena.reserve("first");
        arena.remove(first).unwrap();

        let second = arena.reserve("second");

        assert_ne!(first, second);
        assert_eq!(arena.get(first), Err(ScopeError::Stale(first)));
        assert_eq!(arena.get(second), Ok(&"second"));
    }
}
