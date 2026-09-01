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

    pub fn insert(&mut self, value: T) -> NodeId {
        let id = if let Some(index) = Rc::make_mut(&mut self.free).pop() {
            let slot = self.slot_index_mut(index as usize);
            debug_assert!(slot.value.is_none());
            slot.value = Some(Rc::new(value));
            NodeId {
                index,
                generation: slot.generation,
            }
        } else {
            let index = u32::try_from(self.slots).unwrap();
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
        id
    }

    pub(crate) fn next_id(&self) -> NodeId {
        if let Some(index) = self.free.last().copied() {
            let slot = self.slot_index(index as usize).unwrap();
            NodeId {
                index,
                generation: slot.generation,
            }
        } else {
            NodeId {
                index: u32::try_from(self.slots).unwrap(),
                generation: 0,
            }
        }
    }

    pub fn get(&self, id: NodeId) -> Option<&T> {
        self.slot(id)?.value.as_deref()
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut T> {
        self.slot_mut(id)?.value.as_mut().map(Rc::make_mut)
    }

    pub fn remove(&mut self, id: NodeId) -> Option<T> {
        let slot = self.slot_mut(id)?;
        let value = Rc::unwrap_or_clone(slot.value.take()?);
        if let Some(generation) = slot.generation.checked_add(1) {
            slot.generation = generation;
            Rc::make_mut(&mut self.free).push(id.index);
        }
        self.live -= 1;
        Some(value)
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.live
    }

    fn slot(&self, id: NodeId) -> Option<&Slot<T>> {
        let slot = self.slot_index(id.index as usize)?;
        if slot.generation == id.generation {
            Some(slot)
        } else {
            None
        }
    }

    fn slot_mut(&mut self, id: NodeId) -> Option<&mut Slot<T>> {
        if id.index as usize >= self.slots {
            return None;
        }
        let slot = self.slot_index_mut(id.index as usize);
        if slot.generation == id.generation {
            Some(slot)
        } else {
            None
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
        let first = arena.insert("first");
        assert_eq!(arena.remove(first), Some("first"));

        let second = arena.insert("second");

        assert_eq!(first.index, second.index);
        assert_ne!(first.generation, second.generation);
        assert_eq!(arena.get(first), None);
        assert_eq!(arena.get(second), Some(&"second"));
        assert_eq!(arena.len(), 1);
    }

    #[test]
    fn next_id_predicts_new_and_reused_slots() {
        let mut arena = Arena::new();

        let predicted = arena.next_id();
        let first = arena.insert("first");
        assert_eq!(predicted, first);

        assert_eq!(arena.remove(first), Some("first"));
        let predicted = arena.next_id();
        let second = arena.insert("second");
        assert_eq!(predicted, second);
    }

    #[test]
    fn cloned_chunks_isolate_mutation_and_reuse() {
        let mut original = Arena::new();
        let ids = (0..(CHUNK_CAPACITY + 1))
            .map(|value| original.insert(value))
            .collect::<Vec<_>>();
        let mut candidate = original.clone();

        *candidate.get_mut(ids[CHUNK_CAPACITY]).unwrap() = usize::MAX;
        assert_eq!(original.get(ids[CHUNK_CAPACITY]), Some(&CHUNK_CAPACITY));
        assert_eq!(candidate.get(ids[CHUNK_CAPACITY]), Some(&usize::MAX));

        assert_eq!(candidate.remove(ids[0]), Some(0));
        let replacement = candidate.insert(42);
        assert_eq!(replacement.index, ids[0].index);
        assert_ne!(replacement.generation, ids[0].generation);
        assert_eq!(original.get(ids[0]), Some(&0));
    }
}
