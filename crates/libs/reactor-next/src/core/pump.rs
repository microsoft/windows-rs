use std::collections::{HashMap, VecDeque};

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PumpError {
    AlreadyMounted,
    KindChanged,
    NotMounted,
    DuplicateKey(Key),
    RenderBudgetExceeded,
    RevisionExhausted,
    StructuralApplyFailed,
    StructureUnsupported,
    Tree(TreeError),
}

impl From<TreeError> for PumpError {
    fn from(value: TreeError) -> Self {
        Self::Tree(value)
    }
}

struct PropertyCommit {
    command: usize,
    node: NodeId,
    property: PropertyId,
    value: Option<PropertyValue>,
}

#[derive(Default)]
struct UpdatePlan {
    commands: Vec<Command>,
    undos: Vec<Vec<Command>>,
    commits: Vec<PropertyCommit>,
}

impl UpdatePlan {
    fn push(&mut self, command: Command, undo: Option<Command>) -> usize {
        self.push_many(command, undo.into_iter().collect())
    }

    fn push_many(&mut self, command: Command, undo: Vec<Command>) -> usize {
        let index = self.commands.len();
        self.commands.push(command);
        self.undos.push(undo);
        index
    }
}

pub struct Pump<R> {
    tree: Tree,
    runtime: R,
    root: Option<NodeId>,
    events: VecDeque<QueuedEvent>,
    version: u64,
}

pub struct QueuedEvent {
    pub node: NodeId,
    pub event: EventId,
    pub revision: u32,
    pub payload: EventPayload,
}

impl<R: NativeRuntime> Pump<R> {
    pub fn new(runtime: R) -> Self {
        Self {
            tree: Tree::new(),
            runtime,
            root: None,
            events: VecDeque::new(),
            version: 0,
        }
    }

    pub fn mount(&mut self, element: Element) -> Result<CommitReceipt, PumpError> {
        if self.root.is_some() {
            return Err(PumpError::AlreadyMounted);
        }
        let mut commands = Vec::new();
        let mut commits = Vec::new();
        let node = self.mount_element(None, None, element, &mut commands, &mut commits)?;

        let receipt = self.runtime.apply(&commands);
        let structural_failure = commands
            .iter()
            .enumerate()
            .any(|(index, command)| command.structural() && !receipt.applied(index));
        if structural_failure {
            let rollback = commands
                .iter()
                .enumerate()
                .rev()
                .filter(|(index, _)| receipt.applied(*index))
                .filter_map(|(_, command)| match command {
                    Command::Create { node, .. } => Some(Command::Destroy { node: *node }),
                    Command::InsertChild { parent, child, .. } => Some(Command::RemoveChild {
                        parent: *parent,
                        child: *child,
                    }),
                    _ => None,
                })
                .collect::<Vec<_>>();
            if !rollback.is_empty() {
                self.runtime.apply(&rollback);
            }
            self.tree.retire_subtree(node)?;
            return Ok(receipt);
        }
        self.commit_properties(&commits, &receipt)?;
        self.root = Some(node);
        self.advance_version()?;
        Ok(receipt)
    }

    pub fn update(&mut self, element: Element) -> Result<CommitReceipt, PumpError> {
        let node = self.root.ok_or(PumpError::NotMounted)?;
        let mut candidate = self.tree.clone();
        let mut plan = UpdatePlan::default();
        Self::reconcile_node(&mut candidate, node, element, &mut plan)?;
        if plan.commands.is_empty() {
            self.tree = candidate;
            self.advance_version()?;
            return Ok(CommitReceipt {
                outcomes: Vec::new(),
            });
        }

        let receipt = self.runtime.apply(&plan.commands);
        let structural_failure = plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, command)| command.structural() && !receipt.applied(index));
        if structural_failure {
            let rollback = plan
                .undos
                .iter()
                .enumerate()
                .rev()
                .filter(|(index, _)| receipt.applied(*index))
                .flat_map(|(_, undo)| undo.iter().cloned())
                .collect::<Vec<_>>();
            if !rollback.is_empty() {
                self.runtime.apply(&rollback);
            }
            return Ok(receipt);
        }

        Self::commit_tree_properties(&mut candidate, &plan.commits, &receipt)?;
        self.tree = candidate;
        self.advance_version()?;
        Ok(receipt)
    }

    pub fn runtime(&self) -> &R {
        &self.runtime
    }

    pub fn runtime_mut(&mut self) -> &mut R {
        &mut self.runtime
    }

    pub fn root(&self) -> Option<NodeId> {
        self.root
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn event_revision(&self, node: NodeId, event: EventId) -> Option<u32> {
        self.tree
            .native(node)
            .ok()?
            .events
            .get(&event)
            .filter(|state| state.active)
            .map(|state| state.revision)
    }

    pub fn queue_event(&mut self, event: QueuedEvent) {
        self.events.push_back(event);
    }

    pub fn dispatch_events(&mut self) -> usize {
        let mut dispatched = 0;
        while let Some(event) = self.events.pop_front() {
            let Ok(native) = self.tree.native(event.node) else {
                continue;
            };
            let Some(state) = native.events.get(&event.event) else {
                continue;
            };
            if state.active
                && state.revision == event.revision
                && native.desired.dispatch_event(event.event, &event.payload)
            {
                dispatched += 1;
            }
        }
        dispatched
    }

    fn commit_properties(
        &mut self,
        commits: &[PropertyCommit],
        receipt: &CommitReceipt,
    ) -> Result<(), PumpError> {
        for commit in commits {
            if receipt.applied(commit.command) {
                let committed = &mut self.tree.native_mut(commit.node)?.committed;
                if let Some(value) = &commit.value {
                    committed.insert(commit.property, value.clone());
                } else {
                    committed.remove(&commit.property);
                }
            }
        }
        Ok(())
    }

    fn commit_tree_properties(
        tree: &mut Tree,
        commits: &[PropertyCommit],
        receipt: &CommitReceipt,
    ) -> Result<(), PumpError> {
        for commit in commits {
            if receipt.applied(commit.command) {
                let committed = &mut tree.native_mut(commit.node)?.committed;
                if let Some(value) = &commit.value {
                    committed.insert(commit.property, value.clone());
                } else {
                    committed.remove(&commit.property);
                }
            }
        }
        Ok(())
    }

    fn reconcile_node(
        tree: &mut Tree,
        node: NodeId,
        element: Element,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let parts = element.into_parts();
        let NodeKind::Native(kind) = tree.kind(node)? else {
            return Err(PumpError::NotMounted);
        };
        if kind != parts.kind {
            return Err(PumpError::KindChanged);
        }

        let committed = tree.native(node)?.committed.clone();
        parts.props.visit_properties(&mut |property, value| {
            let changed = match &value {
                Some(value) => committed.get(&property) != Some(value),
                None => committed.contains_key(&property),
            };
            if !changed {
                return;
            }

            let undo = committed.get(&property).map_or_else(
                || Command::ClearProperty { node, property },
                |value| Command::SetProperty {
                    node,
                    property,
                    value: value.clone(),
                },
            );
            let command = match &value {
                Some(value) => Command::SetProperty {
                    node,
                    property,
                    value: value.clone(),
                },
                None => Command::ClearProperty { node, property },
            };
            let command = plan.push(command, Some(undo));
            plan.commits.push(PropertyCommit {
                command,
                node,
                property,
                value,
            });
        });
        Self::update_event_states(tree.native_mut(node)?, &parts.props)?;
        tree.native_mut(node)?.desired = parts.props;

        let current_children = tree.children(node)?.to_vec();
        match parts.structure {
            ElementStructure::None => {
                if !current_children.is_empty() {
                    return Err(PumpError::StructureUnsupported);
                }
            }
            ElementStructure::Content(content) => match (current_children.as_slice(), content) {
                ([], None) => {}
                ([child], Some(content)) => {
                    Self::reconcile_node(tree, *child, content, plan)?;
                }
                _ => return Err(PumpError::StructureUnsupported),
            },
            ElementStructure::Children(children) => {
                let mut old_keys = Vec::with_capacity(current_children.len());
                let mut nodes = HashMap::with_capacity(current_children.len());
                for child in current_children.iter().copied() {
                    let key = tree
                        .key(child)?
                        .cloned()
                        .ok_or(PumpError::StructureUnsupported)?;
                    old_keys.push(key.clone());
                    nodes.insert(key, child);
                }

                let children = children
                    .into_iter()
                    .map(KeyedElement::into_parts)
                    .collect::<Vec<_>>();
                let new_keys = children
                    .iter()
                    .map(|(key, _)| key.clone())
                    .collect::<Vec<_>>();
                let operations = diff(&old_keys, &new_keys)
                    .map_err(|KeyedError::DuplicateKey(key)| PumpError::DuplicateKey(key))?;

                let mut elements = children.into_iter().collect::<HashMap<_, _>>();
                for key in &new_keys {
                    if let Some(child_node) = nodes.get(key).copied() {
                        let child = elements
                            .remove(key)
                            .ok_or(PumpError::StructureUnsupported)?;
                        Self::reconcile_node(tree, child_node, child, plan)?;
                    }
                }

                let mut order = current_children;
                for operation in operations {
                    let (key, before, child, previous) = match operation {
                        KeyedOperation::Move { key, before } => {
                            let child = nodes
                                .get(&key)
                                .copied()
                                .ok_or(PumpError::StructureUnsupported)?;
                            let previous = order
                                .iter()
                                .position(|item| *item == child)
                                .ok_or(PumpError::StructureUnsupported)?;
                            order.remove(previous);
                            (key, before, child, Some(previous))
                        }
                        KeyedOperation::Insert { key, before } => {
                            let element = elements
                                .remove(&key)
                                .ok_or(PumpError::StructureUnsupported)?;
                            let child = Self::mount_planned_element(
                                tree,
                                Some(node),
                                Some(key.clone()),
                                element,
                                plan,
                            )?;
                            (key, before, child, None)
                        }
                        KeyedOperation::Remove { key } => {
                            let child =
                                nodes.remove(&key).ok_or(PumpError::StructureUnsupported)?;
                            let previous = order
                                .iter()
                                .position(|item| *item == child)
                                .ok_or(PumpError::StructureUnsupported)?;
                            order.remove(previous);
                            Self::retire_planned_subtree(tree, child, plan)?;
                            continue;
                        }
                    };
                    let index = if let Some(before) = before {
                        let before = nodes
                            .get(&before)
                            .copied()
                            .ok_or(PumpError::StructureUnsupported)?;
                        order
                            .iter()
                            .position(|item| *item == before)
                            .ok_or(PumpError::StructureUnsupported)?
                    } else {
                        order.len()
                    };
                    order.insert(index, child);
                    if let Some(previous) = previous {
                        plan.push(
                            Command::MoveChild {
                                parent: node,
                                child,
                                index,
                            },
                            Some(Command::MoveChild {
                                parent: node,
                                child,
                                index: previous,
                            }),
                        );
                    } else {
                        plan.push(
                            Command::InsertChild {
                                parent: node,
                                child,
                                index,
                            },
                            Some(Command::RemoveChild {
                                parent: node,
                                child,
                            }),
                        );
                    }
                    nodes.insert(key, child);
                }
                tree.set_children(node, order)?;
            }
        }
        Ok(())
    }

    fn update_event_states(
        native: &mut NativeState,
        desired: &MountedProps,
    ) -> Result<(), PumpError> {
        let mut desired_events = Vec::new();
        desired.visit_events(&mut |event, active| {
            desired_events.push((event, active));
        });
        for (event, active) in desired_events {
            let state = native.events.entry(event).or_insert(EventState {
                revision: 0,
                active: false,
            });
            if state.active != active {
                state.revision = state
                    .revision
                    .checked_add(1)
                    .ok_or(PumpError::RevisionExhausted)?;
                state.active = active;
            }
        }
        Ok(())
    }

    fn advance_version(&mut self) -> Result<(), PumpError> {
        self.version = self
            .version
            .checked_add(1)
            .ok_or(PumpError::RevisionExhausted)?;
        Ok(())
    }

    fn retire_planned_subtree(
        tree: &mut Tree,
        root: NodeId,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        for node in tree.subtree_postorder(root)? {
            if let Some(parent) = tree.parent(node)? {
                let index = tree
                    .children(parent)?
                    .iter()
                    .position(|child| *child == node)
                    .ok_or(PumpError::StructureUnsupported)?;
                plan.push(
                    Command::RemoveChild {
                        parent,
                        child: node,
                    },
                    Some(Command::InsertChild {
                        parent,
                        child: node,
                        index,
                    }),
                );
            }

            let NodeKind::Native(kind) = tree.kind(node)? else {
                return Err(PumpError::StructureUnsupported);
            };
            let mut undo = vec![Command::Create { node, kind }];
            for (property, value) in &tree.native(node)?.committed {
                undo.push(Command::SetProperty {
                    node,
                    property: *property,
                    value: value.clone(),
                });
            }
            plan.push_many(Command::Destroy { node }, undo);
        }
        tree.retire_subtree(root)?;
        Ok(())
    }

    fn mount_planned_element(
        tree: &mut Tree,
        parent: Option<NodeId>,
        key: Option<Key>,
        element: Element,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        let parts = element.into_parts();
        let node = tree.insert_native(parent, parts.kind, key, parts.props.clone())?;
        plan.push(
            Command::Create {
                node,
                kind: parts.kind,
            },
            Some(Command::Destroy { node }),
        );
        parts.props.visit_properties(&mut |property, value| {
            if let Some(value) = value {
                let command = plan.push(
                    Command::SetProperty {
                        node,
                        property,
                        value: value.clone(),
                    },
                    Some(Command::ClearProperty { node, property }),
                );
                plan.commits.push(PropertyCommit {
                    command,
                    node,
                    property,
                    value: Some(value),
                });
            }
        });

        match parts.structure {
            ElementStructure::None => {}
            ElementStructure::Content(content) => {
                if let Some(content) = content {
                    let child = Self::mount_planned_element(tree, Some(node), None, content, plan)?;
                    plan.push(
                        Command::InsertChild {
                            parent: node,
                            child,
                            index: 0,
                        },
                        Some(Command::RemoveChild {
                            parent: node,
                            child,
                        }),
                    );
                }
            }
            ElementStructure::Children(children) => {
                for (index, child) in children.into_iter().enumerate() {
                    let (key, child) = child.into_parts();
                    let child =
                        Self::mount_planned_element(tree, Some(node), Some(key), child, plan)?;
                    plan.push(
                        Command::InsertChild {
                            parent: node,
                            child,
                            index,
                        },
                        Some(Command::RemoveChild {
                            parent: node,
                            child,
                        }),
                    );
                }
            }
        }
        Ok(node)
    }

    fn mount_element(
        &mut self,
        parent: Option<NodeId>,
        key: Option<Key>,
        element: Element,
        commands: &mut Vec<Command>,
        commits: &mut Vec<PropertyCommit>,
    ) -> Result<NodeId, PumpError> {
        let parts = element.into_parts();
        let node = self
            .tree
            .insert_native(parent, parts.kind, key, parts.props.clone())?;
        commands.push(Command::Create {
            node,
            kind: parts.kind,
        });
        parts.props.visit_properties(&mut |property, value| {
            if let Some(value) = value {
                commits.push(PropertyCommit {
                    command: commands.len(),
                    node,
                    property,
                    value: Some(value.clone()),
                });
                commands.push(Command::SetProperty {
                    node,
                    property,
                    value,
                });
            }
        });

        match parts.structure {
            ElementStructure::None => {}
            ElementStructure::Content(content) => {
                if let Some(content) = content {
                    let child = self.mount_element(Some(node), None, content, commands, commits)?;
                    commands.push(Command::InsertChild {
                        parent: node,
                        child,
                        index: 0,
                    });
                }
            }
            ElementStructure::Children(children) => {
                for (index, child) in children.into_iter().enumerate() {
                    let (key, child) = child.into_parts();
                    let child =
                        self.mount_element(Some(node), Some(key), child, commands, commits)?;
                    commands.push(Command::InsertChild {
                        parent: node,
                        child,
                        index,
                    });
                }
            }
        }
        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    fn keyed_text(values: &[&str]) -> Element {
        StackPanel::new()
            .children(
                values
                    .iter()
                    .map(|value| KeyedElement::new(*value, TextBlock::new().text(*value))),
            )
            .into()
    }

    fn keyed_numbers(values: &[u64]) -> Element {
        StackPanel::new()
            .children(
                values.iter().map(|value| {
                    KeyedElement::new(*value, TextBlock::new().text(value.to_string()))
                }),
            )
            .into()
    }

    fn recorded_text(runtime: &RecordingRuntime, root: NodeId) -> Vec<String> {
        runtime
            .node(root)
            .unwrap()
            .children()
            .iter()
            .map(|child| {
                let PropertyValue::Str(value) = runtime
                    .node(*child)
                    .unwrap()
                    .property(PropertyId::TextBlockText)
                    .unwrap()
                else {
                    panic!("expected text");
                };
                value.clone()
            })
            .collect()
    }

    #[test]
    fn mount_update_clear_and_no_change_follow_receipts() {
        let mut pump = Pump::new(RecordingRuntime::default());
        let mounted = pump.mount(TextBlock::new().text("first").into()).unwrap();
        let root = pump.root().unwrap();

        assert_eq!(mounted.outcomes.len(), 2);
        assert_eq!(
            pump.runtime()
                .node(root)
                .unwrap()
                .property(PropertyId::TextBlockText),
            Some(&PropertyValue::Str("first".into()))
        );

        let updated = pump.update(TextBlock::new().text("second").into()).unwrap();
        assert_eq!(updated.outcomes, [CommandOutcome::Applied]);
        assert_eq!(
            pump.runtime()
                .node(root)
                .unwrap()
                .property(PropertyId::TextBlockText),
            Some(&PropertyValue::Str("second".into()))
        );

        let batches = pump.runtime().batches();
        let unchanged = pump.update(TextBlock::new().text("second").into()).unwrap();
        assert!(unchanged.outcomes.is_empty());
        assert_eq!(pump.runtime().batches(), batches);

        let cleared = pump.update(TextBlock::new().into()).unwrap();
        assert_eq!(cleared.outcomes, [CommandOutcome::Applied]);
        assert!(
            pump.runtime()
                .node(root)
                .unwrap()
                .property(PropertyId::TextBlockText)
                .is_none()
        );
    }

    #[test]
    fn failed_property_is_not_committed_and_retries() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(TextBlock::new().text("first").into()).unwrap();
        let root = pump.root().unwrap();
        pump.runtime_mut().fail_at(0);

        let failed = pump.update(TextBlock::new().text("second").into()).unwrap();
        assert_eq!(
            failed.outcomes,
            [CommandOutcome::Failed(RuntimeError::Injected)]
        );
        assert_eq!(
            pump.runtime()
                .node(root)
                .unwrap()
                .property(PropertyId::TextBlockText),
            Some(&PropertyValue::Str("first".into()))
        );

        let retried = pump.update(TextBlock::new().text("second").into()).unwrap();
        assert_eq!(retried.outcomes, [CommandOutcome::Applied]);
        assert_eq!(
            pump.runtime()
                .node(root)
                .unwrap()
                .property(PropertyId::TextBlockText),
            Some(&PropertyValue::Str("second".into()))
        );
    }

    #[test]
    fn failed_create_does_not_publish_a_root() {
        let mut runtime = RecordingRuntime::default();
        runtime.fail_at(0);
        let mut pump = Pump::new(runtime);

        let failed = pump.mount(TextBlock::new().text("first").into()).unwrap();

        assert_eq!(
            failed.outcomes,
            [
                CommandOutcome::Failed(RuntimeError::Injected),
                CommandOutcome::Skipped,
            ]
        );
        assert_eq!(pump.root(), None);
        assert!(pump.runtime().is_empty());
    }

    #[test]
    fn mounts_content_and_keyed_children_recursively() {
        let mut pump = Pump::new(RecordingRuntime::default());
        let tree = StackPanel::new()
            .child("text", TextBlock::new().text("value"))
            .child(
                "button",
                Button::new().content(TextBlock::new().text("increment")),
            );

        pump.mount(tree.into()).unwrap();

        let root = pump.root().unwrap();
        let children = pump.runtime().node(root).unwrap().children();
        assert_eq!(children.len(), 2);
        let button = children[1];
        assert_eq!(pump.runtime().node(button).unwrap().children().len(), 1);
    }

    #[test]
    fn structural_mount_failure_removes_created_nodes() {
        let mut runtime = RecordingRuntime::default();
        runtime.fail_at(1);
        let mut pump = Pump::new(runtime);
        let tree = StackPanel::new().child("text", TextBlock::new().text("value"));

        let failed = pump.mount(tree.into()).unwrap();

        assert!(matches!(
            failed.outcomes[1],
            CommandOutcome::Failed(RuntimeError::Injected)
        ));
        assert_eq!(pump.root(), None);
        assert!(pump.runtime().is_empty());
    }

    #[test]
    fn keyed_reorder_moves_survivors_without_recreation() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(keyed_text(&["a", "b", "c", "d"])).unwrap();
        let root = pump.root().unwrap();

        let receipt = pump.update(keyed_text(&["d", "c", "b", "a"])).unwrap();

        assert_eq!(receipt.outcomes.len(), 3);
        assert!(
            receipt
                .outcomes
                .iter()
                .all(|outcome| *outcome == CommandOutcome::Applied)
        );
        assert_eq!(recorded_text(pump.runtime(), root), ["d", "c", "b", "a"]);
    }

    #[test]
    fn retained_key_recurses_into_property_update() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(
            StackPanel::new()
                .child("value", TextBlock::new().text("first"))
                .into(),
        )
        .unwrap();
        let root = pump.root().unwrap();

        let receipt = pump
            .update(
                StackPanel::new()
                    .child("value", TextBlock::new().text("second"))
                    .into(),
            )
            .unwrap();

        assert_eq!(receipt.outcomes, [CommandOutcome::Applied]);
        assert_eq!(recorded_text(pump.runtime(), root), ["second"]);
    }

    #[test]
    fn failed_keyed_move_restores_native_and_arena_order() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(keyed_text(&["a", "b", "c", "d"])).unwrap();
        let root = pump.root().unwrap();
        pump.runtime_mut().fail_at(1);

        let failed = pump.update(keyed_text(&["d", "c", "b", "a"])).unwrap();

        assert!(matches!(
            failed.outcomes[1],
            CommandOutcome::Failed(RuntimeError::Injected)
        ));
        assert_eq!(recorded_text(pump.runtime(), root), ["a", "b", "c", "d"]);

        let retried = pump.update(keyed_text(&["d", "c", "b", "a"])).unwrap();
        assert_eq!(retried.outcomes.len(), 3);
        assert_eq!(recorded_text(pump.runtime(), root), ["d", "c", "b", "a"]);
    }

    #[test]
    fn keyed_insert_mounts_only_the_new_subtree() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(keyed_text(&["a", "c"])).unwrap();
        let root = pump.root().unwrap();

        let inserted = pump.update(keyed_text(&["a", "b", "c"])).unwrap();

        assert_eq!(inserted.outcomes.len(), 3);
        assert_eq!(recorded_text(pump.runtime(), root), ["a", "b", "c"]);
    }

    #[test]
    fn failed_keyed_insert_destroys_the_candidate_subtree() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(keyed_text(&["a", "c"])).unwrap();
        let root = pump.root().unwrap();
        pump.runtime_mut().fail_at(2);

        let failed = pump.update(keyed_text(&["a", "b", "c"])).unwrap();

        assert!(matches!(
            failed.outcomes[2],
            CommandOutcome::Failed(RuntimeError::Injected)
        ));
        assert_eq!(recorded_text(pump.runtime(), root), ["a", "c"]);

        let retried = pump.update(keyed_text(&["a", "b", "c"])).unwrap();
        assert_eq!(retried.outcomes.len(), 3);
        assert_eq!(recorded_text(pump.runtime(), root), ["a", "b", "c"]);
    }

    #[test]
    fn keyed_remove_retires_the_old_subtree_child_first() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(keyed_text(&["a", "b", "c"])).unwrap();
        let root = pump.root().unwrap();

        let removed = pump.update(keyed_text(&["a", "c"])).unwrap();

        assert_eq!(removed.outcomes.len(), 2);
        assert_eq!(recorded_text(pump.runtime(), root), ["a", "c"]);
    }

    #[test]
    fn failed_keyed_remove_recreates_destroyed_state() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(keyed_text(&["a", "b", "c"])).unwrap();
        let root = pump.root().unwrap();
        pump.runtime_mut().fail_at(1);

        let failed = pump.update(keyed_text(&["a", "c"])).unwrap();

        assert!(matches!(
            failed.outcomes[1],
            CommandOutcome::Failed(RuntimeError::Injected)
        ));
        assert_eq!(recorded_text(pump.runtime(), root), ["a", "b", "c"]);

        let retried = pump.update(keyed_text(&["a", "c"])).unwrap();
        assert_eq!(retried.outcomes.len(), 2);
        assert_eq!(recorded_text(pump.runtime(), root), ["a", "c"]);
    }

    #[test]
    fn randomized_keyed_updates_match_recording_tree() {
        let mut seed = 0x5eed_u64;
        let mut next_random = || {
            seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
            (seed >> 32) as usize
        };
        let mut current = (0_u64..8).collect::<Vec<_>>();
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(keyed_numbers(&current)).unwrap();
        let root = pump.root().unwrap();

        for _ in 0..1_000 {
            let mut pool = (0_u64..16).collect::<Vec<_>>();
            for index in (1..pool.len()).rev() {
                let other = next_random() % (index + 1);
                pool.swap(index, other);
            }
            current = pool[..next_random() % 16].to_vec();

            pump.update(keyed_numbers(&current)).unwrap();

            assert_eq!(
                recorded_text(pump.runtime(), root),
                current.iter().map(u64::to_string).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn queued_event_uses_latest_callback_without_revision_change() {
        let first = Rc::new(Cell::new(0));
        let first_capture = Rc::clone(&first);
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(
            Button::new()
                .on_click(move || first_capture.set(first_capture.get() + 1))
                .into(),
        )
        .unwrap();
        let root = pump.root().unwrap();
        let revision = pump.event_revision(root, EventId::ButtonClick).unwrap();
        pump.queue_event(QueuedEvent {
            node: root,
            event: EventId::ButtonClick,
            revision,
            payload: EventPayload::Unit,
        });

        let second = Rc::new(Cell::new(0));
        let second_capture = Rc::clone(&second);
        let updated = pump
            .update(
                Button::new()
                    .on_click(move || second_capture.set(second_capture.get() + 1))
                    .into(),
            )
            .unwrap();

        assert!(updated.outcomes.is_empty());
        assert_eq!(
            pump.event_revision(root, EventId::ButtonClick),
            Some(revision)
        );
        assert_eq!(pump.dispatch_events(), 1);
        assert_eq!(first.get(), 0);
        assert_eq!(second.get(), 1);
    }

    #[test]
    fn removed_callback_rejects_queued_revision() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(Button::new().on_click(|| {}).into()).unwrap();
        let root = pump.root().unwrap();
        let revision = pump.event_revision(root, EventId::ButtonClick).unwrap();
        pump.queue_event(QueuedEvent {
            node: root,
            event: EventId::ButtonClick,
            revision,
            payload: EventPayload::Unit,
        });

        pump.update(Button::new().into()).unwrap();

        assert_eq!(pump.event_revision(root, EventId::ButtonClick), None);
        assert_eq!(pump.dispatch_events(), 0);
    }

    #[test]
    fn retired_node_rejects_queued_event() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(
            StackPanel::new()
                .child("button", Button::new().on_click(|| {}))
                .into(),
        )
        .unwrap();
        let root = pump.root().unwrap();
        let button = pump.runtime().node(root).unwrap().children()[0];
        let revision = pump.event_revision(button, EventId::ButtonClick).unwrap();
        pump.queue_event(QueuedEvent {
            node: button,
            event: EventId::ButtonClick,
            revision,
            payload: EventPayload::Unit,
        });

        pump.update(StackPanel::new().into()).unwrap();

        assert_eq!(pump.dispatch_events(), 0);
    }

    #[test]
    fn generated_event_payload_reaches_callback() {
        let value = Rc::new(RefCell::new(String::new()));
        let capture = Rc::clone(&value);
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(
            TextBox::new()
                .on_text_changed(move |text| *capture.borrow_mut() = text)
                .into(),
        )
        .unwrap();
        let root = pump.root().unwrap();
        let revision = pump
            .event_revision(root, EventId::TextBoxTextChanged)
            .unwrap();

        pump.queue_event(QueuedEvent {
            node: root,
            event: EventId::TextBoxTextChanged,
            revision,
            payload: EventPayload::Str("updated".into()),
        });

        assert_eq!(pump.dispatch_events(), 1);
        assert_eq!(&*value.borrow(), "updated");
    }
}
