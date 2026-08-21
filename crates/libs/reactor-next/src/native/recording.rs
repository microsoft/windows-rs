use std::collections::{BTreeMap, HashMap, HashSet};

use super::*;

#[derive(Debug)]
pub struct RecordedNode {
    kind: Option<MountedKind>,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    slots: BTreeMap<SlotId, NodeId>,
    properties: BTreeMap<PropertyId, PropertyValue>,
}

pub struct RecordingRuntime {
    application: Option<NodeId>,
    attachments: HashMap<(NodeId, RealizedContainer), NodeId>,
    nodes: HashMap<NodeId, RecordedNode>,
    batches: usize,
    commands: Vec<Vec<Command>>,
    close_requests: Vec<NodeId>,
    record_commands: bool,
    fail_at: HashSet<(usize, usize)>,
    identity: Option<WindowToken>,
    realizations: Vec<NativeWork<RealizationRequest>>,
    source_revisions: HashMap<NodeId, u64>,
    subscriptions: HashSet<(NodeId, EventId)>,
    window_titles: HashMap<NodeId, String>,
    windows: HashSet<NodeId>,
}

impl Default for RecordingRuntime {
    fn default() -> Self {
        Self {
            application: None,
            attachments: HashMap::new(),
            nodes: HashMap::new(),
            batches: 0,
            commands: Vec::new(),
            close_requests: Vec::new(),
            record_commands: true,
            fail_at: HashSet::new(),
            identity: None,
            realizations: Vec::new(),
            source_revisions: HashMap::new(),
            subscriptions: HashSet::new(),
            window_titles: HashMap::new(),
            windows: HashSet::new(),
        }
    }
}

impl RecordingRuntime {
    pub fn record_commands(&mut self, record: bool) {
        self.record_commands = record;
        if !record {
            self.commands.clear();
        }
    }

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

    #[cfg(any(test, feature = "test"))]
    pub fn close_requests(&self) -> &[NodeId] {
        &self.close_requests
    }

    #[cfg(any(test, feature = "test"))]
    pub fn window_title(&self, node: NodeId) -> Option<&str> {
        self.window_titles.get(&node).map(String::as_str)
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn queue_realization(&mut self, request: RealizationRequest) {
        self.realizations.push(NativeWork {
            identity: self.identity.unwrap(),
            work: request,
        });
    }

    pub fn queue_realize(
        &mut self,
        collection: NodeId,
        container: RealizedContainer,
        index: usize,
    ) {
        self.queue_realization(RealizationRequest::Realize {
            collection,
            container,
            index,
            source_revision: self.source_revisions[&collection],
        });
    }

    pub fn queue_recycle(&mut self, collection: NodeId, container: RealizedContainer) {
        self.queue_realization(RealizationRequest::Recycle {
            collection,
            container,
            source_revision: self.source_revisions[&collection],
        });
    }

    pub fn source_revision(&self, collection: NodeId) -> Option<u64> {
        self.source_revisions.get(&collection).copied()
    }

    pub fn queue_realization_with_identity(
        &mut self,
        identity: WindowToken,
        request: RealizationRequest,
    ) {
        self.realizations.push(NativeWork {
            identity,
            work: request,
        });
    }
}

impl RecordedNode {
    pub fn kind(&self) -> Option<MountedKind> {
        self.kind
    }

    pub fn property(&self, property: PropertyId) -> Option<&PropertyValue> {
        self.properties.get(&property)
    }

    pub fn children(&self) -> &[NodeId] {
        &self.children
    }

    pub fn slot(&self, slot: SlotId) -> Option<NodeId> {
        self.slots.get(&slot).copied()
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
                        slots: BTreeMap::new(),
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
                        slots: BTreeMap::new(),
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
            Command::CloseWindow { node } => {
                if !self.windows.contains(node) {
                    return Err(RuntimeError::MissingNode(*node));
                }
                self.close_requests.push(*node);
            }
            Command::SetWindowTitle { node, title } => {
                if !self.windows.contains(node) {
                    return Err(RuntimeError::MissingNode(*node));
                }
                self.window_titles.insert(*node, title.clone());
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
                        slots: BTreeMap::new(),
                        properties: BTreeMap::new(),
                    },
                );
            }
            Command::CreateVirtualCollection {
                node,
                source_revision,
                ..
            } => {
                if self.nodes.contains_key(node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                self.nodes.insert(
                    *node,
                    RecordedNode {
                        kind: None,
                        parent: None,
                        children: Vec::new(),
                        slots: BTreeMap::new(),
                        properties: BTreeMap::new(),
                    },
                );
                self.source_revisions.insert(*node, *source_revision);
            }
            Command::ResetVirtualCollection {
                node,
                source_revision,
                ..
            } => {
                self.nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                self.source_revisions.insert(*node, *source_revision);
            }
            Command::AttachRealized {
                collection,
                container,
                child,
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
                    .get(collection)
                    .ok_or(RuntimeError::MissingNode(*collection))?;
                if let Some(previous) = self.attachments.get(&(*collection, *container)).copied() {
                    self.nodes
                        .get(&previous)
                        .ok_or(RuntimeError::MissingNode(previous))?;
                    let collection = self.nodes.get_mut(collection).unwrap();
                    let position = collection
                        .children
                        .iter()
                        .position(|current| *current == previous)
                        .ok_or(RuntimeError::ChildNotFound(previous))?;
                    collection.children.remove(position);
                    self.nodes.get_mut(&previous).unwrap().parent = None;
                }
                self.attachments.insert((*collection, *container), *child);
                self.nodes
                    .get_mut(collection)
                    .ok_or(RuntimeError::MissingNode(*collection))?
                    .children
                    .push(*child);
                self.nodes.get_mut(child).unwrap().parent = Some(*collection);
            }
            Command::DetachRealized {
                collection,
                container,
                child,
            } => {
                let attachment = (*collection, *container);
                self.nodes
                    .get(child)
                    .ok_or(RuntimeError::MissingNode(*child))?;
                if self.attachments.get(&attachment) != Some(child) {
                    return Err(RuntimeError::ChildNotFound(*child));
                }
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
                self.attachments.remove(&attachment);
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
                if !recorded.children.is_empty() || !recorded.slots.is_empty() {
                    return Err(RuntimeError::HasChildren(*node));
                }
                self.nodes.remove(node);
                self.source_revisions.remove(node);
                self.subscriptions
                    .retain(|(subscription_node, _)| subscription_node != node);
                self.window_titles.remove(node);
                self.windows.remove(node);
                if self.application == Some(*node) {
                    self.application = None;
                }
            }
            Command::Focus { node } => {
                self.nodes
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
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
            Command::SetSlot {
                parent,
                slot,
                child,
            } => {
                if child == &Some(*parent) {
                    return Err(RuntimeError::SelfParent(*parent));
                }
                self.nodes
                    .get(parent)
                    .ok_or(RuntimeError::MissingNode(*parent))?;
                if let Some(child) = child {
                    let child_node = self
                        .nodes
                        .get(child)
                        .ok_or(RuntimeError::MissingNode(*child))?;
                    let current = self.nodes[parent].slots.get(slot).copied();
                    if child_node.parent.is_some() && current != Some(*child) {
                        return Err(RuntimeError::AlreadyParented(*child));
                    }
                }
                let previous = if let Some(child) = child {
                    self.nodes
                        .get_mut(parent)
                        .unwrap()
                        .slots
                        .insert(*slot, *child)
                } else {
                    self.nodes.get_mut(parent).unwrap().slots.remove(slot)
                };
                if let Some(previous) = previous {
                    self.nodes.get_mut(&previous).unwrap().parent = None;
                }
                if let Some(child) = child {
                    self.nodes.get_mut(child).unwrap().parent = Some(*parent);
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
            Command::ResetChildren { parent } => {
                let children = {
                    let parent = self
                        .nodes
                        .get_mut(parent)
                        .ok_or(RuntimeError::MissingNode(*parent))?;
                    std::mem::take(&mut parent.children)
                };
                for child in children {
                    self.nodes.get_mut(&child).unwrap().parent = None;
                }
            }
            Command::SynchronizeChildren { parent, children } => {
                if children.contains(parent) {
                    return Err(RuntimeError::SelfParent(*parent));
                }
                if children.iter().collect::<HashSet<_>>().len() != children.len() {
                    return Err(RuntimeError::DuplicateNode(*parent));
                }
                for child in children {
                    let node = self
                        .nodes
                        .get(child)
                        .ok_or(RuntimeError::MissingNode(*child))?;
                    if node.parent.is_some() && node.parent != Some(*parent) {
                        return Err(RuntimeError::AlreadyParented(*child));
                    }
                }
                let previous = {
                    let parent = self
                        .nodes
                        .get_mut(parent)
                        .ok_or(RuntimeError::MissingNode(*parent))?;
                    std::mem::replace(&mut parent.children, children.clone())
                };
                for child in previous {
                    self.nodes.get_mut(&child).unwrap().parent = None;
                }
                for child in children {
                    let node = self.nodes.get_mut(child).unwrap();
                    if node.parent.is_some() {
                        return Err(RuntimeError::AlreadyParented(*child));
                    }
                    node.parent = Some(*parent);
                }
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
    fn apply(&mut self, commands: &[Command]) -> Result<(), NativeApplyError> {
        self.batches += 1;
        if self.record_commands {
            self.commands.push(commands.to_vec());
        }
        for (index, command) in commands.iter().enumerate() {
            let result = if self.fail_at.remove(&(self.batches, index)) {
                Err(RuntimeError::Injected)
            } else {
                self.apply_one(command)
            };
            result.map_err(|error| NativeApplyError {
                command: index,
                error,
            })?;
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.application = None;
        self.attachments.clear();
        self.close_requests.clear();
        self.nodes.clear();
        self.realizations.clear();
        self.source_revisions.clear();
        self.subscriptions.clear();
        self.windows.clear();
    }

    fn set_identity(&mut self, identity: WindowToken) {
        self.identity = Some(identity);
    }

    fn drain_realizations(&mut self) -> Vec<NativeWork<RealizationRequest>> {
        std::mem::take(&mut self.realizations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROOT: NodeId = NodeId::from_parts(0, 0);
    const CHILD: NodeId = NodeId::from_parts(1, 0);

    #[test]
    fn command_history_can_be_disabled() {
        let mut runtime = RecordingRuntime::default();
        runtime.record_commands(false);
        runtime
            .apply(&[Command::Create {
                node: ROOT,
                kind: MountedKind::TextBlock,
            }])
            .unwrap();

        assert!(runtime.commands().is_empty());
        assert!(runtime.node(ROOT).is_some());
    }

    #[test]
    fn records_tree_and_property_mutations() {
        let mut runtime = RecordingRuntime::default();
        runtime
            .apply(&[
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
            ])
            .unwrap();
        assert_eq!(runtime.batches, 1);
        assert_eq!(
            runtime.node(ROOT).unwrap().kind(),
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
    fn attach_realized_replaces_content_for_the_same_shell_lifetime() {
        let mut runtime = RecordingRuntime::default();
        let second = NodeId::from_parts(2, 0);
        let container = RealizedContainer(7);
        runtime
            .apply(&[
                Command::CreateVirtualCollection {
                    node: ROOT,
                    item_count: 1,
                    source_revision: 0,
                },
                Command::Create {
                    node: CHILD,
                    kind: MountedKind::TextBlock,
                },
                Command::Create {
                    node: second,
                    kind: MountedKind::Button,
                },
                Command::AttachRealized {
                    collection: ROOT,
                    container,
                    child: CHILD,
                },
                Command::AttachRealized {
                    collection: ROOT,
                    container,
                    child: second,
                },
            ])
            .unwrap();

        assert_eq!(runtime.node(ROOT).unwrap().children(), &[second]);
        assert_eq!(runtime.node(CHILD).unwrap().parent, None);
        assert_eq!(runtime.node(second).unwrap().parent, Some(ROOT));

        runtime
            .apply(&[Command::DetachRealized {
                collection: ROOT,
                container,
                child: second,
            }])
            .unwrap();
        assert!(runtime.node(ROOT).unwrap().children().is_empty());
    }

    #[test]
    fn records_clear_move_remove_and_child_first_destroy() {
        let mut runtime = RecordingRuntime::default();
        let second = NodeId::from_parts(2, 0);
        runtime
            .apply(&[
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
            ])
            .unwrap();

        runtime
            .apply(&[
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
            ])
            .unwrap();
        assert_eq!(runtime.node(ROOT).unwrap().children, [second]);
        assert!(runtime.node(CHILD).is_none());
    }

    #[test]
    fn failure_stops_before_later_commands() {
        let mut runtime = RecordingRuntime::default();
        let error = runtime
            .apply(&[
                Command::SetProperty {
                    node: CHILD,
                    property: PropertyId::TextBlockText,
                    value: PropertyValue::Str("missing".into()),
                },
                Command::Create {
                    node: ROOT,
                    kind: MountedKind::StackPanel,
                },
            ])
            .unwrap_err();

        assert_eq!(
            error,
            NativeApplyError {
                command: 0,
                error: RuntimeError::MissingNode(CHILD),
            }
        );
        assert!(runtime.node(ROOT).is_none());
    }

    #[test]
    fn structural_failure_skips_dependent_commands() {
        let mut runtime = RecordingRuntime::default();
        runtime.fail_at(0);

        let error = runtime
            .apply(&[
                Command::Create {
                    node: ROOT,
                    kind: MountedKind::StackPanel,
                },
                Command::Create {
                    node: CHILD,
                    kind: MountedKind::TextBlock,
                },
            ])
            .unwrap_err();

        assert_eq!(
            error,
            NativeApplyError {
                command: 0,
                error: RuntimeError::Injected,
            }
        );
        assert!(runtime.nodes.is_empty());
    }
}
