#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeId {
    index: u32,
    generation: u32,
}

impl NodeId {
    pub const fn from_parts(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArenaError {
    Stale(NodeId),
    CapacityExceeded,
}

#[derive(Clone)]
struct Slot<T> {
    generation: u32,
    value: Option<T>,
}

#[derive(Clone)]
pub struct Arena<T> {
    slots: Vec<Slot<T>>,
    free: Vec<u32>,
    live: usize,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            free: Vec::new(),
            live: 0,
        }
    }

    pub fn insert(&mut self, value: T) -> Result<NodeId, ArenaError> {
        let id = if let Some(index) = self.free.pop() {
            let slot = &mut self.slots[index as usize];
            debug_assert!(slot.value.is_none());
            slot.value = Some(value);
            NodeId {
                index,
                generation: slot.generation,
            }
        } else {
            let index =
                u32::try_from(self.slots.len()).map_err(|_| ArenaError::CapacityExceeded)?;
            self.slots.push(Slot {
                generation: 0,
                value: Some(value),
            });
            NodeId {
                index,
                generation: 0,
            }
        };
        self.live += 1;
        Ok(id)
    }

    pub fn get(&self, id: NodeId) -> Result<&T, ArenaError> {
        let slot = self.slot(id)?;
        slot.value.as_ref().ok_or(ArenaError::Stale(id))
    }

    pub fn get_mut(&mut self, id: NodeId) -> Result<&mut T, ArenaError> {
        let slot = self.slot_mut(id)?;
        slot.value.as_mut().ok_or(ArenaError::Stale(id))
    }

    pub fn remove(&mut self, id: NodeId) -> Result<T, ArenaError> {
        let slot = self.slot_mut(id)?;
        let value = slot.value.take().ok_or(ArenaError::Stale(id))?;
        if let Some(generation) = slot.generation.checked_add(1) {
            slot.generation = generation;
            self.free.push(id.index);
        }
        self.live -= 1;
        Ok(value)
    }

    pub fn len(&self) -> usize {
        self.live
    }

    fn slot(&self, id: NodeId) -> Result<&Slot<T>, ArenaError> {
        let slot = self
            .slots
            .get(id.index as usize)
            .ok_or(ArenaError::Stale(id))?;
        if slot.generation == id.generation {
            Ok(slot)
        } else {
            Err(ArenaError::Stale(id))
        }
    }

    fn slot_mut(&mut self, id: NodeId) -> Result<&mut Slot<T>, ArenaError> {
        let slot = self
            .slots
            .get_mut(id.index as usize)
            .ok_or(ArenaError::Stale(id))?;
        if slot.generation == id.generation {
            Ok(slot)
        } else {
            Err(ArenaError::Stale(id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reused_slot_rejects_old_generation() {
        let mut arena = Arena::new();
        let first = arena.insert("first").unwrap();
        assert_eq!(arena.remove(first), Ok("first"));

        let second = arena.insert("second").unwrap();

        assert_eq!(first.index, second.index);
        assert_ne!(first.generation, second.generation);
        assert_eq!(arena.get(first), Err(ArenaError::Stale(first)));
        assert_eq!(arena.get(second), Ok(&"second"));
        assert_eq!(arena.len(), 1);
    }
}
