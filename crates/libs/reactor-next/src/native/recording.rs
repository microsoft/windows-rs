use std::collections::{BTreeMap, HashMap, HashSet};

use super::*;

#[derive(Debug)]
pub struct RecordedNode {
    kind: Option<MountedKind>,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    properties: BTreeMap<PropertyId, PropertyValue>,
}

#[derive(Default)]
pub struct RecordingRuntime {
    application: Option<NodeId>,
    nodes: HashMap<NodeId, RecordedNode>,
    batches: usize,
    commands: Vec<Vec<Command>>,
    fail_at: HashSet<(usize, usize)>,
    realizations: Vec<RealizationRequest>,
    subscriptions: HashSet<(NodeId, EventId)>,
    windows: HashSet<NodeId>,
}

impl RecordingRuntime {
    pub fn fail_at(&mut self, command_index: usize) {
        self.fail_after(0, command_index);
    }

    pub fn fail_after(&mut self, batches: usize, command_index: usize) {
        self.fail_at
            .insert((self.batches + batches + 1, command_index));
    }

    pub fn node(&self, id: NodeId) -> Option<&RecordedNode> {
        self.nodes.get(&id)
    }

    pub fn batches(&self) -> usize {
        self.batches
    }

    pub fn commands(&self) -> &[Vec<Command>] {
        &self.commands
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn queue_realization(&mut self, request: RealizationRequest) {
        self.realizations.push(request);
    }
}

impl RecordedNode {
    pub fn property(&self, property: PropertyId) -> Option<&PropertyValue> {
        self.properties.get(&property)
    }

    pub fn children(&self) -> &[NodeId] {
        &self.children
    }
}

impl RecordingRuntime {
    fn apply_one(&mut self, command: &Command) -> Result<(), RuntimeError> {
        match command {
            Command::CreateApplication { node } => {
                if self.nodes.contains_key(node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                if self.application.is_some() {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                self.nodes.insert(
                    *node,
                    RecordedNode {
                        kind: None,
                        parent: None,
                        children: Vec::new(),
                        properties: BTreeMap::new(),
                    },
                );
                self.application = Some(*node);
            }
            Command::CreateWindow { node } => {
                if self.nodes.contains_key(node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                self.nodes.insert(
                    *node,
                    RecordedNode {
                        kind: None,
                        parent: None,
                        children: Vec::new(),
                        properties: BTreeMap::new(),
                    },
                );
                self.windows.insert(*node);
            }
            Command::ActivateWindow { node } => {
                self.nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
            }
            Command::ResetWindowContent { window } => {
                if !self.windows.contains(window) {
                    return Err(RuntimeError::MissingNode(*window));
                }
                self.subscriptions.clear();
                let application = self.application;
                let windows = &self.windows;
                self.nodes
                    .retain(|node, _| Some(*node) == application || windows.contains(node));
                for window in &self.windows {
                    self.nodes.get_mut(window).unwrap().children.clear();
                }
            }
            Command::Create { node, kind } => {
                if self.nodes.contains_key(node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                self.nodes.insert(
                    *node,
                    RecordedNode {
                        kind: Some(*kind),
                        parent: None,
                        children: Vec::new(),
                        properties: BTreeMap::new(),
                    },
                );
            }
            Command::CreateVirtualCollection { node, .. } => {
                if self.nodes.contains_key(node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                self.nodes.insert(
                    *node,
                    RecordedNode {
                        kind: None,
                        parent: None,
                        children: Vec::new(),
                        properties: BTreeMap::new(),
                    },
                );
            }
            Command::ResetVirtualCollection { node, .. } => {
                self.nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
            }
            Command::AttachRealized {
                collection, child, ..
            } => {
                if self
                    .nodes
                    .get(child)
                    .ok_or(RuntimeError::MissingNode(*child))?
                    .parent
                    .is_some()
                {
                    return Err(RuntimeError::AlreadyParented(*child));
                }
                self.nodes
                    .get_mut(collection)
                    .ok_or(RuntimeError::MissingNode(*collection))?
                    .children
                    .push(*child);
                self.nodes.get_mut(child).unwrap().parent = Some(*collection);
            }
            Command::DetachRealized {
                collection, child, ..
            } => {
                self.nodes
                    .get(child)
                    .ok_or(RuntimeError::MissingNode(*child))?;
                let collection = self
                    .nodes
                    .get_mut(collection)
                    .ok_or(RuntimeError::MissingNode(*collection))?;
                let position = collection
                    .children
                    .iter()
                    .position(|current| current == child)
                    .ok_or(RuntimeError::ChildNotFound(*child))?;
                collection.children.remove(position);
                self.nodes.get_mut(child).unwrap().parent = None;
            }
            Command::Destroy { node } => {
                let recorded = self
                    .nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                if recorded.parent.is_some() {
                    return Err(RuntimeError::StillParented(*node));
                }
                if !recorded.children.is_empty() {
                    return Err(RuntimeError::HasChildren(*node));
                }
                self.nodes.remove(node);
                self.subscriptions
                    .retain(|(subscription_node, _)| subscription_node != node);
                self.windows.remove(node);
                if self.application == Some(*node) {
                    self.application = None;
                }
            }
            Command::SetProperty {
                node,
                property,
                value,
            } => {
                self.nodes
                    .get_mut(node)
                    .ok_or(RuntimeError::MissingNode(*node))?
                    .properties
                    .insert(*property, value.clone());
            }
            Command::ClearProperty { node, property } => {
                self.nodes
                    .get_mut(node)
                    .ok_or(RuntimeError::MissingNode(*node))?
                    .properties
                    .remove(property);
            }
            Command::SubscribeEvent { node, event, .. } => {
                self.nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                if !self.subscriptions.insert((*node, *event)) {
                    return Err(RuntimeError::DuplicateEvent(*node, *event));
                }
            }
            Command::UnsubscribeEvent { node, event } => {
                self.nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                if !self.subscriptions.remove(&(*node, *event)) {
                    return Err(RuntimeError::MissingSubscription(*node, *event));
                }
            }
            Command::InsertChild {
                parent,
                child,
                index,
            } => {
                if parent == child {
                    return Err(RuntimeError::SelfParent(*child));
                }
                let parent_node = self
                    .nodes
                    .get(parent)
                    .ok_or(RuntimeError::MissingNode(*parent))?;
                if *index > parent_node.children.len() {
                    return Err(RuntimeError::IndexOutOfBounds);
                }
                let child_node = self
                    .nodes
                    .get(child)
                    .ok_or(RuntimeError::MissingNode(*child))?;
                if child_node.parent.is_some() {
                    return Err(RuntimeError::AlreadyParented(*child));
                }

                self.nodes
                    .get_mut(parent)
                    .unwrap()
                    .children
                    .insert(*index, *child);
                self.nodes.get_mut(child).unwrap().parent = Some(*parent);
            }
            Command::RemoveChild { parent, child } => {
                let child_node = self
                    .nodes
                    .get(child)
                    .ok_or(RuntimeError::MissingNode(*child))?;
                if child_node.parent != Some(*parent) {
                    return Err(RuntimeError::ChildNotFound(*child));
                }
                let parent_node = self
                    .nodes
                    .get_mut(parent)
                    .ok_or(RuntimeError::MissingNode(*parent))?;
                let position = parent_node
                    .children
                    .iter()
                    .position(|current| current == child)
                    .ok_or(RuntimeError::ChildNotFound(*child))?;
                parent_node.children.remove(position);
                self.nodes.get_mut(child).unwrap().parent = None;
            }
            Command::MoveChild {
                parent,
                child,
                index,
            } => {
                let parent_node = self
                    .nodes
                    .get_mut(parent)
                    .ok_or(RuntimeError::MissingNode(*parent))?;
                let position = parent_node
                    .children
                    .iter()
                    .position(|current| current == child)
                    .ok_or(RuntimeError::ChildNotFound(*child))?;
                if *index >= parent_node.children.len() {
                    return Err(RuntimeError::IndexOutOfBounds);
                }
                let child = parent_node.children.remove(position);
                parent_node.children.insert(*index, child);
            }
        }
        Ok(())
    }
}

impl NativeRuntime for RecordingRuntime {
    fn apply(&mut self, commands: &[Command]) -> CommitReceipt {
        self.batches += 1;
        self.commands.push(commands.to_vec());
        let mut structural_failure = false;
        let outcomes = commands
            .iter()
            .enumerate()
            .map(|(index, command)| {
                if structural_failure {
                    return CommandOutcome::Skipped;
                }

                let result = if self.fail_at.remove(&(self.batches, index)) {
                    Err(RuntimeError::Injected)
                } else {
                    self.apply_one(command)
                };
                match result {
                    Ok(()) => CommandOutcome::Applied,
                    Err(error) => {
                        structural_failure = command.structural();
                        CommandOutcome::Failed(error)
                    }
                }
            })
            .collect();
        CommitReceipt { outcomes }
    }

    fn reset(&mut self) {
        self.application = None;
        self.nodes.clear();
        self.realizations.clear();
        self.subscriptions.clear();
        self.windows.clear();
    }

    fn drain_realizations(&mut self) -> Vec<RealizationRequest> {
        std::mem::take(&mut self.realizations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROOT: NodeId = NodeId::from_parts(0, 0);
    const CHILD: NodeId = NodeId::from_parts(1, 0);

    #[test]
    fn records_tree_and_property_mutations() {
        let mut runtime = RecordingRuntime::default();
        let receipt = runtime.apply(&[
            Command::Create {
                node: ROOT,
                kind: MountedKind::StackPanel,
            },
            Command::Create {
                node: CHILD,
                kind: MountedKind::TextBlock,
            },
            Command::SetProperty {
                node: CHILD,
                property: PropertyId::TextBlockText,
                value: PropertyValue::Str("hello".into()),
            },
            Command::InsertChild {
                parent: ROOT,
                child: CHILD,
                index: 0,
            },
        ]);

        assert!(
            receipt
                .outcomes
                .iter()
                .all(|outcome| { *outcome == CommandOutcome::Applied })
        );
        assert_eq!(runtime.batches, 1);
        assert_eq!(
            runtime.node(ROOT).unwrap().kind,
            Some(MountedKind::StackPanel)
        );
        assert_eq!(runtime.node(ROOT).unwrap().children, [CHILD]);
        assert_eq!(runtime.node(CHILD).unwrap().parent, Some(ROOT));
        assert_eq!(
            runtime.node(CHILD).unwrap().properties[&PropertyId::TextBlockText],
            PropertyValue::Str("hello".into())
        );
    }

    #[test]
    fn records_clear_move_remove_and_child_first_destroy() {
        let mut runtime = RecordingRuntime::default();
        let second = NodeId::from_parts(2, 0);
        runtime.apply(&[
            Command::Create {
                node: ROOT,
                kind: MountedKind::StackPanel,
            },
            Command::Create {
                node: CHILD,
                kind: MountedKind::TextBlock,
            },
            Command::Create {
                node: second,
                kind: MountedKind::TextBlock,
            },
            Command::InsertChild {
                parent: ROOT,
                child: CHILD,
                index: 0,
            },
            Command::InsertChild {
                parent: ROOT,
                child: second,
                index: 1,
            },
            Command::SetProperty {
                node: CHILD,
                property: PropertyId::TextBlockText,
                value: PropertyValue::Str("temporary".into()),
            },
        ]);

        let receipt = runtime.apply(&[
            Command::ClearProperty {
                node: CHILD,
                property: PropertyId::TextBlockText,
            },
            Command::MoveChild {
                parent: ROOT,
                child: second,
                index: 0,
            },
            Command::RemoveChild {
                parent: ROOT,
                child: CHILD,
            },
            Command::Destroy { node: CHILD },
        ]);

        assert!(receipt.applied(0));
        assert!(receipt.applied(3));
        assert_eq!(runtime.node(ROOT).unwrap().children, [second]);
        assert!(runtime.node(CHILD).is_none());
    }

    #[test]
    fn property_failure_does_not_skip_later_commands() {
        let mut runtime = RecordingRuntime::default();
        let receipt = runtime.apply(&[
            Command::SetProperty {
                node: CHILD,
                property: PropertyId::TextBlockText,
                value: PropertyValue::Str("missing".into()),
            },
            Command::Create {
                node: ROOT,
                kind: MountedKind::StackPanel,
            },
        ]);

        assert_eq!(
            receipt.outcomes,
            [
                CommandOutcome::Failed(RuntimeError::MissingNode(CHILD)),
                CommandOutcome::Applied,
            ]
        );
        assert!(runtime.node(ROOT).is_some());
    }

    #[test]
    fn structural_failure_skips_dependent_commands() {
        let mut runtime = RecordingRuntime::default();
        runtime.fail_at(0);

        let receipt = runtime.apply(&[
            Command::Create {
                node: ROOT,
                kind: MountedKind::StackPanel,
            },
            Command::Create {
                node: CHILD,
                kind: MountedKind::TextBlock,
            },
        ]);

        assert_eq!(
            receipt.outcomes,
            [
                CommandOutcome::Failed(RuntimeError::Injected),
                CommandOutcome::Skipped,
            ]
        );
        assert!(runtime.nodes.is_empty());
    }

    #[test]
    fn reset_window_content_preserves_host_and_drops_control_state() {
        let application = NodeId::from_parts(10, 0);
        let window = NodeId::from_parts(11, 0);
        let button = NodeId::from_parts(12, 0);
        let replacement = NodeId::from_parts(12, 1);
        let mut runtime = RecordingRuntime::default();
        let mounted = runtime.apply(&[
            Command::CreateApplication { node: application },
            Command::CreateWindow { node: window },
            Command::Create {
                node: button,
                kind: MountedKind::Button,
            },
            Command::SubscribeEvent {
                node: button,
                event: EventId::ButtonClick,
                revision: 1,
            },
            Command::InsertChild {
                parent: window,
                child: button,
                index: 0,
            },
        ]);
        assert!(
            mounted
                .outcomes
                .iter()
                .all(|outcome| { *outcome == CommandOutcome::Applied })
        );

        let reset = runtime.apply(&[
            Command::ResetWindowContent { window },
            Command::Create {
                node: replacement,
                kind: MountedKind::TextBlock,
            },
            Command::InsertChild {
                parent: window,
                child: replacement,
                index: 0,
            },
        ]);

        assert!(
            reset
                .outcomes
                .iter()
                .all(|outcome| { *outcome == CommandOutcome::Applied })
        );
        assert!(runtime.node(application).is_some());
        assert!(runtime.node(window).is_some());
        assert!(runtime.node(button).is_none());
        assert_eq!(runtime.node(window).unwrap().children(), &[replacement]);
        assert!(runtime.subscriptions.is_empty());
    }

    #[test]
    fn reset_unknown_window_skips_following_commands() {
        let mut runtime = RecordingRuntime::default();

        let receipt = runtime.apply(&[
            Command::ResetWindowContent { window: ROOT },
            Command::Create {
                node: CHILD,
                kind: MountedKind::TextBlock,
            },
        ]);

        assert_eq!(
            receipt.outcomes,
            [
                CommandOutcome::Failed(RuntimeError::MissingNode(ROOT)),
                CommandOutcome::Skipped,
            ]
        );
    }
}
