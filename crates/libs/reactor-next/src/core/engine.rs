use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeKind {
    Application,
    Window,
    Component,
    Slot,
    Native(MountedKind),
    VirtualCollection,
}

struct Node {
    kind: NodeKind,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TreeError {
    Arena(ArenaError),
    RootAlreadyExists,
}

impl From<ArenaError> for TreeError {
    fn from(value: ArenaError) -> Self {
        Self::Arena(value)
    }
}

pub struct Tree {
    arena: Arena<Node>,
    root: Option<NodeId>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            root: None,
        }
    }

    pub fn insert(&mut self, parent: Option<NodeId>, kind: NodeKind) -> Result<NodeId, TreeError> {
        if let Some(parent) = parent {
            self.arena.get(parent)?;
        } else if self.root.is_some() {
            return Err(TreeError::RootAlreadyExists);
        }

        let id = self.arena.insert(Node {
            kind,
            parent,
            children: Vec::new(),
        })?;

        if let Some(parent) = parent {
            self.arena.get_mut(parent)?.children.push(id);
        } else {
            self.root = Some(id);
        }
        Ok(id)
    }

    pub fn parent(&self, id: NodeId) -> Result<Option<NodeId>, TreeError> {
        Ok(self.arena.get(id)?.parent)
    }

    pub fn children(&self, id: NodeId) -> Result<&[NodeId], TreeError> {
        Ok(&self.arena.get(id)?.children)
    }

    pub fn len(&self) -> usize {
        self.arena.len()
    }

    pub fn retire_subtree(&mut self, id: NodeId) -> Result<Vec<(NodeId, NodeKind)>, TreeError> {
        let mut order = Vec::new();
        self.collect_postorder(id, &mut order)?;

        let parent = self.arena.get(id)?.parent;
        if let Some(parent) = parent {
            self.arena
                .get_mut(parent)?
                .children
                .retain(|child| *child != id);
        } else {
            self.root = None;
        }

        let mut retired = Vec::with_capacity(order.len());
        for id in order {
            let node = self.arena.remove(id)?;
            retired.push((id, node.kind));
        }
        Ok(retired)
    }

    fn collect_postorder(&self, id: NodeId, order: &mut Vec<NodeId>) -> Result<(), TreeError> {
        for child in self.arena.get(id)?.children.iter().copied() {
            self.collect_postorder(child, order)?;
        }
        order.push(id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> usize {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            (self.0 >> 32) as usize
        }
    }

    #[test]
    fn retires_children_before_parent() {
        let mut tree = Tree::new();
        let root = tree.insert(None, NodeKind::Application).unwrap();
        let window = tree.insert(Some(root), NodeKind::Window).unwrap();
        let component = tree.insert(Some(window), NodeKind::Component).unwrap();
        let slot = tree.insert(Some(component), NodeKind::Slot).unwrap();
        let native = tree
            .insert(Some(slot), NodeKind::Native(MountedKind::TextBlock))
            .unwrap();
        let collection = tree
            .insert(Some(window), NodeKind::VirtualCollection)
            .unwrap();

        assert_eq!(tree.parent(native), Ok(Some(slot)));
        assert_eq!(tree.children(root), Ok(&[window][..]));

        let retired = tree.retire_subtree(window).unwrap();

        assert_eq!(
            retired,
            vec![
                (native, NodeKind::Native(MountedKind::TextBlock)),
                (slot, NodeKind::Slot),
                (component, NodeKind::Component),
                (collection, NodeKind::VirtualCollection),
                (window, NodeKind::Window),
            ]
        );
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.children(root), Ok(&[][..]));
        assert_eq!(
            tree.parent(window),
            Err(TreeError::Arena(ArenaError::Stale(window)))
        );
    }

    #[test]
    fn rejects_second_root() {
        let mut tree = Tree::new();
        tree.insert(None, NodeKind::Application).unwrap();

        assert_eq!(
            tree.insert(None, NodeKind::Application),
            Err(TreeError::RootAlreadyExists)
        );
    }

    #[test]
    fn randomized_insert_and_retire_matches_tree_model() {
        let mut rng = Rng(0x5eed);
        let mut tree = Tree::new();
        let root = tree.insert(None, NodeKind::Application).unwrap();
        let mut live = vec![root];
        let mut parents = HashMap::from([(root, None)]);

        for _ in 0..5_000 {
            if live.len() == 1 || !rng.next().is_multiple_of(3) {
                let parent = live[rng.next() % live.len()];
                let id = tree.insert(Some(parent), NodeKind::Slot).unwrap();
                live.push(id);
                assert_eq!(parents.insert(id, Some(parent)), None);
            } else {
                let victim = live[1 + rng.next() % (live.len() - 1)];
                let retired = tree.retire_subtree(victim).unwrap();
                let retired_ids: HashSet<_> = retired.iter().map(|(id, _)| *id).collect();
                assert_eq!(retired.len(), retired_ids.len());

                let positions: HashMap<_, _> = retired
                    .iter()
                    .enumerate()
                    .map(|(position, (id, _))| (*id, position))
                    .collect();
                for id in retired_ids.iter().copied() {
                    if let Some(Some(parent)) = parents.get(&id)
                        && let Some(parent_position) = positions.get(parent)
                    {
                        assert!(positions[&id] < *parent_position);
                    }
                    assert_eq!(
                        tree.parent(id),
                        Err(TreeError::Arena(ArenaError::Stale(id)))
                    );
                }

                live.retain(|id| !retired_ids.contains(id));
                parents.retain(|id, _| !retired_ids.contains(id));
            }

            assert_eq!(tree.len(), live.len());
            assert_eq!(tree.parent(root), Ok(None));
        }
    }

    #[test]
    fn element_split_keeps_props_shallow_and_moves_children_once() {
        let parts = Element::from(StackPanel::new().child("text", TextBlock::new().text("hello")))
            .into_parts();

        assert_eq!(parts.kind, MountedKind::StackPanel);
        assert!(matches!(parts.props, MountedProps::StackPanel { .. }));
        let ElementStructure::Children(children) = parts.structure else {
            panic!("expected keyed children");
        };
        assert_eq!(children.len(), 1);
        assert!(matches!(children[0].element(), Element::TextBlock(_)));
    }
}
