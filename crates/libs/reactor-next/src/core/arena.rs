use std::rc::Rc;

const CHUNK_CAPACITY: usize = 256;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeId {
    index: u32,
    generation: u32,
}

impl NodeId {
    pub const fn from_parts(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    pub(crate) const fn index(self) -> usize {
        self.index as usize
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
    value: Option<Rc<T>>,
}

#[derive(Clone)]
pub struct Arena<T> {
    chunks: Rc<Vec<Rc<Vec<Slot<T>>>>>,
    free: Rc<Vec<u32>>,
    live: usize,
    slots: usize,
}

impl<T: Clone> Arena<T> {
    pub fn new() -> Self {
        Self {
            chunks: Rc::new(Vec::new()),
            free: Rc::new(Vec::new()),
            live: 0,
            slots: 0,
        }
    }

    pub fn insert(&mut self, value: T) -> Result<NodeId, ArenaError> {
        let id = if let Some(index) = Rc::make_mut(&mut self.free).pop() {
            let slot = self.slot_index_mut(index as usize);
            debug_assert!(slot.value.is_none());
            slot.value = Some(Rc::new(value));
            NodeId {
                index,
                generation: slot.generation,
            }
        } else {
            let index = u32::try_from(self.slots).map_err(|_| ArenaError::CapacityExceeded)?;
            let chunks = Rc::make_mut(&mut self.chunks);
            if chunks
                .last()
                .is_none_or(|chunk| chunk.len() == CHUNK_CAPACITY)
            {
                chunks.push(Rc::new(Vec::with_capacity(CHUNK_CAPACITY)));
            }
            Rc::make_mut(chunks.last_mut().unwrap()).push(Slot {
                generation: 0,
                value: Some(Rc::new(value)),
            });
            self.slots += 1;
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
        slot.value.as_deref().ok_or(ArenaError::Stale(id))
    }

    pub fn get_mut(&mut self, id: NodeId) -> Result<&mut T, ArenaError> {
        let slot = self.slot_mut(id)?;
        slot.value
            .as_mut()
            .map(Rc::make_mut)
            .ok_or(ArenaError::Stale(id))
    }

    pub fn remove(&mut self, id: NodeId) -> Result<T, ArenaError> {
        let slot = self.slot_mut(id)?;
        let value = Rc::unwrap_or_clone(slot.value.take().ok_or(ArenaError::Stale(id))?);
        if let Some(generation) = slot.generation.checked_add(1) {
            slot.generation = generation;
            Rc::make_mut(&mut self.free).push(id.index);
        }
        self.live -= 1;
        Ok(value)
    }

    pub fn len(&self) -> usize {
        self.live
    }

    fn slot(&self, id: NodeId) -> Result<&Slot<T>, ArenaError> {
        let slot = self
            .slot_index(id.index as usize)
            .ok_or(ArenaError::Stale(id))?;
        if slot.generation == id.generation {
            Ok(slot)
        } else {
            Err(ArenaError::Stale(id))
        }
    }

    fn slot_mut(&mut self, id: NodeId) -> Result<&mut Slot<T>, ArenaError> {
        if id.index as usize >= self.slots {
            return Err(ArenaError::Stale(id));
        }
        let slot = self.slot_index_mut(id.index as usize);
        if slot.generation == id.generation {
            Ok(slot)
        } else {
            Err(ArenaError::Stale(id))
        }
    }

    fn slot_index(&self, index: usize) -> Option<&Slot<T>> {
        self.chunks
            .get(index / CHUNK_CAPACITY)?
            .get(index % CHUNK_CAPACITY)
    }

    fn slot_index_mut(&mut self, index: usize) -> &mut Slot<T> {
        let chunks = Rc::make_mut(&mut self.chunks);
        let chunk = Rc::make_mut(&mut chunks[index / CHUNK_CAPACITY]);
        &mut chunk[index % CHUNK_CAPACITY]
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

    #[test]
    fn cloned_chunks_isolate_mutation_and_reuse() {
        let mut original = Arena::new();
        let ids = (0..(CHUNK_CAPACITY + 1))
            .map(|value| original.insert(value).unwrap())
            .collect::<Vec<_>>();
        let mut candidate = original.clone();

        *candidate.get_mut(ids[CHUNK_CAPACITY]).unwrap() = usize::MAX;
        assert_eq!(original.get(ids[CHUNK_CAPACITY]), Ok(&CHUNK_CAPACITY));
        assert_eq!(candidate.get(ids[CHUNK_CAPACITY]), Ok(&usize::MAX));

        assert_eq!(candidate.remove(ids[0]), Ok(0));
        let replacement = candidate.insert(42).unwrap();
        assert_eq!(replacement.index, ids[0].index);
        assert_ne!(replacement.generation, ids[0].generation);
        assert_eq!(original.get(ids[0]), Ok(&0));
    }
}
