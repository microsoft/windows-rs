use std::collections::{HashMap, VecDeque};

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PumpError {
    AlreadyMounted,
    ApplyReceiptMismatch,
    KindChanged,
    NotMounted,
    DuplicateKey(Key),
    EventReadFailed(RuntimeError),
    Poisoned,
    PropertyApplyFailed(CommitReceipt),
    RenderBudgetExceeded,
    RevisionExhausted,
    StructuralApplyFailed(CommitReceipt),
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
    commits: Vec<PropertyCommit>,
}

impl UpdatePlan {
    fn push(&mut self, command: Command) -> usize {
        let index = self.commands.len();
        self.commands.push(command);
        index
    }
}

pub struct Pump<R: NativeRuntime> {
    application: Option<NodeId>,
    tree: Tree,
    runtime: R,
    root: Option<NodeId>,
    events: VecDeque<QueuedEvent>,
    poisoned: bool,
    retry_pending: bool,
    version: u64,
    window: Option<NodeId>,
}

impl<R: NativeRuntime> Pump<R> {
    pub fn new(runtime: R) -> Self {
        Self {
            application: None,
            tree: Tree::new(),
            runtime,
            root: None,
            events: VecDeque::new(),
            poisoned: false,
            retry_pending: false,
            version: 0,
            window: None,
        }
    }

    pub fn mount(&mut self, element: Element) -> Result<CommitReceipt, PumpError> {
        if self.poisoned {
            return Err(PumpError::Poisoned);
        }
        if self.root.is_some() {
            return Err(PumpError::AlreadyMounted);
        }
        let next_version = self.next_version()?;
        let mut candidate = Tree::new();
        let mut plan = UpdatePlan::default();
        let application = candidate.insert(None, NodeKind::Application)?;
        plan.push(Command::CreateApplication { node: application });
        let window = candidate.insert(Some(application), NodeKind::Window)?;
        plan.push(Command::CreateWindow { node: window });
        let node =
            Self::mount_planned_element(&mut candidate, Some(window), None, element, &mut plan)?;
        plan.push(Command::InsertChild {
            parent: window,
            child: node,
            index: 0,
        });
        plan.push(Command::ActivateWindow { node: window });

        let receipt = self.runtime.apply(&plan.commands);
        if receipt.outcomes.len() != plan.commands.len() {
            self.runtime.reset();
            self.tree = Tree::new();
            self.application = None;
            self.events.clear();
            self.poisoned = true;
            self.root = None;
            self.retry_pending = false;
            self.window = None;
            return Err(PumpError::ApplyReceiptMismatch);
        }
        let structural_failure = plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, command)| command.structural() && !receipt.applied(index));
        if structural_failure {
            self.runtime.reset();
            self.tree = Tree::new();
            self.application = None;
            self.events.clear();
            self.poisoned = true;
            self.root = None;
            self.retry_pending = false;
            self.window = None;
            return Err(PumpError::StructuralApplyFailed(receipt));
        }
        Self::commit_tree_properties(&mut candidate, &plan.commits, &receipt)?;
        self.tree = candidate;
        self.application = Some(application);
        self.root = Some(node);
        self.window = Some(window);
        if plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, command)| !command.structural() && !receipt.applied(index))
        {
            self.retry_pending = true;
            return Err(PumpError::PropertyApplyFailed(receipt));
        }
        self.retry_pending = false;
        self.version = next_version;
        Ok(receipt)
    }

    pub fn update(&mut self, element: Element) -> Result<CommitReceipt, PumpError> {
        if self.poisoned {
            return Err(PumpError::Poisoned);
        }
        let next_version = self.next_version()?;
        let node = self.root.ok_or(PumpError::NotMounted)?;
        let mut candidate = self.tree.clone();
        let mut plan = UpdatePlan::default();
        Self::reconcile_node(&mut candidate, node, element, &mut plan)?;
        if plan.commands.is_empty() {
            self.tree = candidate;
            self.retry_pending = false;
            self.version = next_version;
            return Ok(CommitReceipt {
                outcomes: Vec::new(),
            });
        }

        let receipt = self.runtime.apply(&plan.commands);
        if receipt.outcomes.len() != plan.commands.len() {
            self.poisoned = true;
            self.events.clear();
            self.retry_pending = false;
            return Err(PumpError::ApplyReceiptMismatch);
        }
        let structural_failure = plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, command)| command.structural() && !receipt.applied(index));
        if structural_failure {
            self.poisoned = true;
            self.events.clear();
            self.retry_pending = false;
            return Err(PumpError::StructuralApplyFailed(receipt));
        }

        Self::commit_tree_properties(&mut candidate, &plan.commits, &receipt)?;
        self.tree = candidate;
        if plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, command)| !command.structural() && !receipt.applied(index))
        {
            self.retry_pending = true;
            return Err(PumpError::PropertyApplyFailed(receipt));
        }
        self.retry_pending = false;
        self.version = next_version;
        Ok(receipt)
    }

    pub fn runtime(&self) -> &R {
        &self.runtime
    }

    pub fn application(&self) -> Option<NodeId> {
        self.application
    }

    pub fn runtime_mut(&mut self) -> &mut R {
        &mut self.runtime
    }

    pub fn shutdown(&mut self) {
        self.runtime.reset();
        self.application = None;
        self.events.clear();
        self.poisoned = false;
        self.retry_pending = false;
        self.root = None;
        self.tree = Tree::new();
        self.version = 0;
        self.window = None;
    }

    pub fn root(&self) -> Option<NodeId> {
        self.root
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn window(&self) -> Option<NodeId> {
        self.window
    }

    pub fn poisoned(&self) -> bool {
        self.poisoned
    }

    pub fn retry_pending(&self) -> bool {
        self.retry_pending
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

    pub fn dispatch_events(&mut self) -> Result<usize, PumpError> {
        self.events.extend(self.runtime.drain_events());
        if self.poisoned {
            self.events.clear();
            _ = self.runtime.drain_event_errors();
            return Ok(0);
        }
        if let Some(error) = self.runtime.drain_event_errors().into_iter().next() {
            self.events.clear();
            return Err(PumpError::EventReadFailed(error));
        }
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
        Ok(dispatched)
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

            let command = match &value {
                Some(value) => Command::SetProperty {
                    node,
                    property,
                    value: value.clone(),
                },
                None => Command::ClearProperty { node, property },
            };
            let command = plan.push(command);
            plan.commits.push(PropertyCommit {
                command,
                node,
                property,
                value,
            });
        });
        Self::update_event_states(tree.native_mut(node)?, node, &parts.props, plan)?;
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
                    let (key, before, child, moved) = match operation {
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
                            (key, before, child, true)
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
                            (key, before, child, false)
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
                    if moved {
                        plan.push(Command::MoveChild {
                            parent: node,
                            child,
                            index,
                        });
                    } else {
                        plan.push(Command::InsertChild {
                            parent: node,
                            child,
                            index,
                        });
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
        node: NodeId,
        desired: &MountedProps,
        plan: &mut UpdatePlan,
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
                if active {
                    plan.push(Command::SubscribeEvent {
                        node,
                        event,
                        revision: state.revision,
                    });
                } else {
                    plan.push(Command::UnsubscribeEvent { node, event });
                }
            }
        }
        Ok(())
    }

    fn next_version(&self) -> Result<u64, PumpError> {
        self.version
            .checked_add(1)
            .ok_or(PumpError::RevisionExhausted)
    }

    fn retire_planned_subtree(
        tree: &mut Tree,
        root: NodeId,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        for node in tree.subtree_postorder(root)? {
            if let Some(parent) = tree.parent(node)? {
                plan.push(Command::RemoveChild {
                    parent,
                    child: node,
                });
            }

            let NodeKind::Native(_) = tree.kind(node)? else {
                return Err(PumpError::StructureUnsupported);
            };
            for (event, state) in &tree.native(node)?.events {
                if state.active {
                    plan.push(Command::UnsubscribeEvent {
                        node,
                        event: *event,
                    });
                }
            }
            plan.push(Command::Destroy { node });
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
        plan.push(Command::Create {
            node,
            kind: parts.kind,
        });
        parts.props.visit_properties(&mut |property, value| {
            if let Some(value) = value {
                let command = plan.push(Command::SetProperty {
                    node,
                    property,
                    value: value.clone(),
                });
                plan.commits.push(PropertyCommit {
                    command,
                    node,
                    property,
                    value: Some(value),
                });
            }
        });
        for (event, state) in &tree.native(node)?.events {
            if state.active {
                plan.push(Command::SubscribeEvent {
                    node,
                    event: *event,
                    revision: state.revision,
                });
            }
        }

        match parts.structure {
            ElementStructure::None => {}
            ElementStructure::Content(content) => {
                if let Some(content) = content {
                    let child = Self::mount_planned_element(tree, Some(node), None, content, plan)?;
                    plan.push(Command::InsertChild {
                        parent: node,
                        child,
                        index: 0,
                    });
                }
            }
            ElementStructure::Children(children) => {
                let keys = children
                    .iter()
                    .map(|child| child.key().clone())
                    .collect::<Vec<_>>();
                diff(&[], &keys)
                    .map_err(|KeyedError::DuplicateKey(key)| PumpError::DuplicateKey(key))?;
                for (index, child) in children.into_iter().enumerate() {
                    let (key, child) = child.into_parts();
                    let child =
                        Self::mount_planned_element(tree, Some(node), Some(key), child, plan)?;
                    plan.push(Command::InsertChild {
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

impl<R: NativeRuntime> Drop for Pump<R> {
    fn drop(&mut self) {
        self.runtime.reset();
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

    fn representative_tree() -> Element {
        StackPanel::new()
            .spacing(8.0)
            .child(
                "button",
                Button::new()
                    .is_enabled(true)
                    .on_click(|| {})
                    .content(TextBlock::new().text("increment")),
            )
            .into()
    }

    fn arena_keys(pump: &Pump<RecordingRuntime>) -> Vec<Key> {
        pump.tree
            .children(pump.root().unwrap())
            .unwrap()
            .iter()
            .map(|node| pump.tree.key(*node).unwrap().unwrap().clone())
            .collect()
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

    fn structural_receipt(error: PumpError) -> CommitReceipt {
        let PumpError::StructuralApplyFailed(receipt) = error else {
            panic!("expected structural apply failure");
        };
        receipt
    }

    #[derive(Default)]
    struct ShortReceiptRuntime {
        inner: RecordingRuntime,
        short_next: bool,
    }

    impl NativeRuntime for ShortReceiptRuntime {
        fn apply(&mut self, commands: &[Command]) -> CommitReceipt {
            let mut receipt = self.inner.apply(commands);
            if self.short_next {
                self.short_next = false;
                receipt.outcomes.pop();
            }
            receipt
        }

        fn reset(&mut self) {
            self.inner.reset();
        }
    }

    #[derive(Default)]
    struct EventErrorRuntime {
        error: Option<RuntimeError>,
    }

    impl NativeRuntime for EventErrorRuntime {
        fn apply(&mut self, commands: &[Command]) -> CommitReceipt {
            CommitReceipt {
                outcomes: vec![CommandOutcome::Applied; commands.len()],
            }
        }

        fn reset(&mut self) {}

        fn drain_event_errors(&mut self) -> Vec<RuntimeError> {
            self.error.take().into_iter().collect()
        }
    }

    #[test]
    fn mount_update_clear_and_no_change_follow_receipts() {
        let mut pump = Pump::new(RecordingRuntime::default());
        let mounted = pump.mount(TextBlock::new().text("first").into()).unwrap();
        let root = pump.root().unwrap();

        assert_eq!(mounted.outcomes.len(), 6);
        assert!(pump.application().is_some());
        assert!(pump.window().is_some());
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
        let version = pump.version();
        pump.runtime_mut().fail_at(0);

        let failed = pump
            .update(TextBlock::new().text("second").into())
            .unwrap_err();
        let PumpError::PropertyApplyFailed(failed) = failed else {
            panic!("expected property apply failure");
        };
        assert_eq!(
            failed.outcomes,
            [CommandOutcome::Failed(RuntimeError::Injected)]
        );
        assert_eq!(pump.version(), version);
        assert!(pump.retry_pending());
        assert_eq!(
            pump.runtime()
                .node(root)
                .unwrap()
                .property(PropertyId::TextBlockText),
            Some(&PropertyValue::Str("first".into()))
        );

        let retried = pump.update(TextBlock::new().text("second").into()).unwrap();
        assert_eq!(retried.outcomes, [CommandOutcome::Applied]);
        assert_eq!(pump.version(), version + 1);
        assert!(!pump.retry_pending());
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

        let failed = structural_receipt(
            pump.mount(TextBlock::new().text("first").into())
                .unwrap_err(),
        );

        assert_eq!(
            failed.outcomes,
            [
                CommandOutcome::Failed(RuntimeError::Injected),
                CommandOutcome::Skipped,
                CommandOutcome::Skipped,
                CommandOutcome::Skipped,
                CommandOutcome::Skipped,
                CommandOutcome::Skipped,
            ]
        );
        assert_eq!(pump.root(), None);
        assert!(pump.runtime().is_empty());
        assert_eq!(pump.version(), 0);
        assert!(!pump.retry_pending());
        assert!(pump.poisoned());
        assert_eq!(
            pump.mount(TextBlock::new().text("first").into()),
            Err(PumpError::Poisoned)
        );
    }

    #[test]
    fn duplicate_mount_keys_fail_before_native_apply() {
        let mut pump = Pump::new(RecordingRuntime::default());

        assert_eq!(
            pump.mount(
                StackPanel::new()
                    .child("duplicate", TextBlock::new())
                    .child("duplicate", TextBlock::new())
                    .into()
            ),
            Err(PumpError::DuplicateKey(Key::from("duplicate")))
        );
        assert_eq!(pump.runtime().batches(), 0);
        assert!(pump.runtime().is_empty());
        assert_eq!(pump.root(), None);
    }

    #[test]
    fn malformed_update_receipt_poisons_without_advancing_version() {
        let mut pump = Pump::new(ShortReceiptRuntime::default());
        pump.mount(TextBlock::new().text("first").into()).unwrap();
        let version = pump.version();
        pump.runtime_mut().short_next = true;

        assert_eq!(
            pump.update(TextBlock::new().text("second").into()),
            Err(PumpError::ApplyReceiptMismatch)
        );
        assert_eq!(pump.version(), version);
        assert!(pump.poisoned());
    }

    #[test]
    fn every_mount_command_failure_reaches_a_defined_state() {
        let mut baseline = Pump::new(RecordingRuntime::default());
        let command_count = baseline
            .mount(representative_tree())
            .unwrap()
            .outcomes
            .len();
        assert!(command_count > 1);

        for failed_index in 0..command_count {
            let mut runtime = RecordingRuntime::default();
            runtime.fail_at(failed_index);
            let mut pump = Pump::new(runtime);
            let error = pump.mount(representative_tree()).unwrap_err();

            match error {
                PumpError::PropertyApplyFailed(receipt) => {
                    assert!(matches!(
                        receipt.outcomes[failed_index],
                        CommandOutcome::Failed(RuntimeError::Injected)
                    ));
                    assert_eq!(pump.version(), 0);
                    assert!(pump.retry_pending());
                    assert!(!pump.poisoned());
                    assert!(pump.root().is_some());

                    pump.update(representative_tree()).unwrap();
                    assert_eq!(pump.version(), 1);
                    assert!(!pump.retry_pending());
                }
                PumpError::StructuralApplyFailed(receipt) => {
                    assert!(matches!(
                        receipt.outcomes[failed_index],
                        CommandOutcome::Failed(RuntimeError::Injected)
                    ));
                    assert_eq!(pump.version(), 0);
                    assert!(!pump.retry_pending());
                    assert!(pump.poisoned());
                    assert_eq!(pump.root(), None);
                    assert!(pump.runtime().is_empty());
                    assert_eq!(pump.mount(representative_tree()), Err(PumpError::Poisoned));
                }
                error => panic!("unexpected mount failure: {error:?}"),
            }
        }
    }

    #[test]
    fn every_update_command_failure_reaches_a_defined_state() {
        let before = keyed_text(&["a", "b", "c"]);
        let after: Element = StackPanel::new()
            .child("c", TextBlock::new().text("c updated"))
            .child("d", TextBlock::new().text("d"))
            .child("a", TextBlock::new().text("a"))
            .into();
        let mut baseline = Pump::new(RecordingRuntime::default());
        baseline.mount(before.clone()).unwrap();
        let command_count = baseline.update(after.clone()).unwrap().outcomes.len();
        assert!(command_count > 1);

        for failed_index in 0..command_count {
            let mut pump = Pump::new(RecordingRuntime::default());
            pump.mount(before.clone()).unwrap();
            let version = pump.version();
            let old_keys = arena_keys(&pump);
            pump.runtime_mut().fail_at(failed_index);
            let error = pump.update(after.clone()).unwrap_err();

            match error {
                PumpError::PropertyApplyFailed(receipt) => {
                    assert!(matches!(
                        receipt.outcomes[failed_index],
                        CommandOutcome::Failed(RuntimeError::Injected)
                    ));
                    assert_eq!(pump.version(), version);
                    assert!(pump.retry_pending());
                    assert!(!pump.poisoned());
                    assert_eq!(
                        arena_keys(&pump),
                        [Key::from("c"), Key::from("d"), Key::from("a")]
                    );

                    pump.update(after.clone()).unwrap();
                    assert_eq!(pump.version(), version + 1);
                    assert!(!pump.retry_pending());
                    assert_eq!(
                        recorded_text(pump.runtime(), pump.root().unwrap())[0],
                        "c updated"
                    );
                }
                PumpError::StructuralApplyFailed(receipt) => {
                    assert!(matches!(
                        receipt.outcomes[failed_index],
                        CommandOutcome::Failed(RuntimeError::Injected)
                    ));
                    assert_eq!(pump.version(), version);
                    assert!(!pump.retry_pending());
                    assert!(pump.poisoned());
                    assert_eq!(arena_keys(&pump), old_keys);
                    assert_eq!(pump.update(after.clone()), Err(PumpError::Poisoned));
                }
                error => panic!("unexpected update failure: {error:?}"),
            }
        }
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
    fn application_window_and_root_share_one_arena_lifetime() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(TextBlock::new().text("root").into()).unwrap();
        let application = pump.application().unwrap();
        let window = pump.window().unwrap();
        let root = pump.root().unwrap();

        assert_eq!(pump.tree.parent(application), Ok(None));
        assert_eq!(pump.tree.parent(window), Ok(Some(application)));
        assert_eq!(pump.tree.parent(root), Ok(Some(window)));
        assert!(pump.runtime().node(application).is_some());
        assert!(pump.runtime().node(window).is_some());
        assert!(pump.runtime().node(root).is_some());

        pump.shutdown();

        assert_eq!(pump.application(), None);
        assert_eq!(pump.window(), None);
        assert_eq!(pump.root(), None);
        assert_eq!(pump.version(), 0);
        assert!(pump.runtime().is_empty());
    }

    #[test]
    fn structural_mount_failure_removes_created_nodes() {
        let mut runtime = RecordingRuntime::default();
        runtime.fail_at(1);
        let mut pump = Pump::new(runtime);
        let tree = StackPanel::new().child("text", TextBlock::new().text("value"));

        let failed = structural_receipt(pump.mount(tree.into()).unwrap_err());

        assert!(matches!(
            failed.outcomes[1],
            CommandOutcome::Failed(RuntimeError::Injected)
        ));
        assert_eq!(pump.root(), None);
        assert!(pump.runtime().is_empty());
        assert!(pump.poisoned());
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
    fn failed_keyed_move_poisons_without_publishing_candidate() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(keyed_text(&["a", "b", "c", "d"])).unwrap();
        let version = pump.version();
        pump.runtime_mut().fail_at(1);

        let failed =
            structural_receipt(pump.update(keyed_text(&["d", "c", "b", "a"])).unwrap_err());

        assert!(matches!(
            failed.outcomes[1],
            CommandOutcome::Failed(RuntimeError::Injected)
        ));
        assert_eq!(pump.version(), version);
        assert!(pump.poisoned());
        assert_eq!(
            pump.update(keyed_text(&["d", "c", "b", "a"])),
            Err(PumpError::Poisoned)
        );
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
    fn failed_keyed_insert_poisons_without_publishing_candidate() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(keyed_text(&["a", "c"])).unwrap();
        let version = pump.version();
        pump.runtime_mut().fail_at(2);

        let failed = structural_receipt(pump.update(keyed_text(&["a", "b", "c"])).unwrap_err());

        assert!(matches!(
            failed.outcomes[2],
            CommandOutcome::Failed(RuntimeError::Injected)
        ));
        assert_eq!(pump.version(), version);
        assert!(pump.poisoned());
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
    fn failed_keyed_remove_poisons_without_publishing_candidate() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(keyed_text(&["a", "b", "c"])).unwrap();
        let version = pump.version();
        pump.runtime_mut().fail_at(1);

        let failed = structural_receipt(pump.update(keyed_text(&["a", "c"])).unwrap_err());

        assert!(matches!(
            failed.outcomes[1],
            CommandOutcome::Failed(RuntimeError::Injected)
        ));
        assert_eq!(pump.version(), version);
        assert!(pump.poisoned());
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
        assert_eq!(pump.dispatch_events(), Ok(1));
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
        assert_eq!(pump.dispatch_events(), Ok(0));
    }

    #[test]
    fn poisoned_pump_discards_queued_events() {
        let calls = Rc::new(Cell::new(0));
        let callback_calls = Rc::clone(&calls);
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(
            Button::new()
                .on_click(move || callback_calls.set(callback_calls.get() + 1))
                .into(),
        )
        .unwrap();
        let root = pump.root().unwrap();
        let revision = pump.event_revision(root, EventId::ButtonClick).unwrap();
        pump.runtime_mut().fail_at(0);
        assert!(matches!(
            pump.update(Button::new().into()),
            Err(PumpError::StructuralApplyFailed(_))
        ));
        pump.queue_event(QueuedEvent {
            node: root,
            event: EventId::ButtonClick,
            revision,
            payload: EventPayload::Unit,
        });

        assert_eq!(pump.dispatch_events(), Ok(0));
        assert_eq!(calls.get(), 0);
    }

    #[test]
    fn event_payload_read_failure_is_reported() {
        let mut pump = Pump::new(EventErrorRuntime::default());
        pump.mount(TextBox::new().into()).unwrap();
        pump.runtime_mut().error = Some(RuntimeError::Injected);

        assert_eq!(
            pump.dispatch_events(),
            Err(PumpError::EventReadFailed(RuntimeError::Injected))
        );
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

        assert_eq!(pump.dispatch_events(), Ok(0));
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

        assert_eq!(pump.dispatch_events(), Ok(1));
        assert_eq!(&*value.borrow(), "updated");
    }
}
