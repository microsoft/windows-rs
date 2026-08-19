use std::collections::BTreeMap;

use crate::element::tree::StructuralSlot;
use crate::id::NodeId;
use crate::mounted::Mounted;
use crate::runtime::{CommandSection, NativeKind, NavigationSection, OwnerRelation};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealizedRow {
    pub lease: u64,
    pub key: u64,
    pub root: NodeId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Application,
    Window,
    Logical,
    StructuralSlot(StructuralSlot),
    CommandSection(CommandSection),
    NavigationSection(NavigationSection),
    OwnedNative,
    OwnerBound {
        relation: OwnerRelation,
        project_accessory: bool,
    },
    Native,
    VirtualHost {
        realized: BTreeMap<usize, RealizedRow>,
    },
}

impl NodeKind {
    pub const fn is_native(&self) -> bool {
        matches!(
            self,
            Self::Native | Self::OwnedNative | Self::VirtualHost { .. }
        )
    }

    pub const fn projects_native_root(&self) -> bool {
        matches!(self, Self::Native | Self::VirtualHost { .. })
    }
}

pub(crate) struct Node {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub kind: NodeKind,
    pub native_kind: Option<NativeKind>,
    pub mounted: Option<Mounted>,
}

struct Slot {
    generation: u32,
    node: Option<Node>,
}

#[derive(Default)]
pub(crate) struct Arena {
    slots: Vec<Slot>,
    free: Vec<u32>,
    revision: u64,
}

impl Arena {
    pub fn insert(&mut self, node: Node) -> NodeId {
        self.revision = self.revision.wrapping_add(1);
        if let Some(index) = self.free.pop() {
            let slot = &mut self.slots[index as usize];
            debug_assert!(slot.node.is_none());
            slot.node = Some(node);
            NodeId::new(index, slot.generation)
        } else {
            let index = u32::try_from(self.slots.len()).unwrap();
            self.slots.push(Slot {
                generation: 0,
                node: Some(node),
            });
            NodeId::new(index, 0)
        }
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        let slot = self.slots.get(id.index() as usize)?;
        (slot.generation == id.generation())
            .then_some(slot.node.as_ref())
            .flatten()
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        let slot = self.slots.get_mut(id.index() as usize)?;
        (slot.generation == id.generation())
            .then_some(slot.node.as_mut())
            .flatten()
    }

    pub fn remove(&mut self, id: NodeId) -> Option<Node> {
        let slot = self.slots.get_mut(id.index() as usize)?;
        if slot.generation != id.generation() {
            return None;
        }
        let node = slot.node.take()?;
        self.revision = self.revision.wrapping_add(1);
        if let Some(generation) = slot.generation.checked_add(1) {
            slot.generation = generation;
            self.free.push(id.index());
        }
        Some(node)
    }

    pub fn contains(&self, id: NodeId) -> bool {
        self.get(id).is_some()
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.slots.iter().enumerate().filter_map(|(index, slot)| {
            slot.node
                .as_ref()
                .map(|_| NodeId::new(u32::try_from(index).unwrap(), slot.generation))
        })
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.slots.iter().filter_map(|slot| slot.node.as_ref())
    }
}

#[cfg(test)]
#[path = "../testing/private/arena.rs"]
mod tests;
