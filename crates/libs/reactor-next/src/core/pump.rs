use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PumpError {
    AlreadyMounted,
    ApplyReceiptMismatch,
    NotMounted,
    DuplicateKey(Key),
    EventReadFailed(RuntimeError),
    Poisoned,
    PropertyApplyFailed(CommitReceipt),
    RecoveredStructure(Box<StructuralRecovery>),
    RecoveryFailed(Box<StructuralRecovery>),
    RenderBudgetExceeded,
    RevisionExhausted,
    StructuralApplyFailed(CommitReceipt),
    StructureUnsupported,
    Tree(TreeError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuralRecovery {
    pub failure: CommitReceipt,
    pub recovery: CommitReceipt,
    pub root: NodeId,
}

impl PumpError {
    pub fn recoverable(&self) -> bool {
        matches!(
            self,
            Self::PropertyApplyFailed(_) | Self::RecoveredStructure(_)
        )
    }
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

struct UpdatePlan {
    commands: Vec<Command>,
    commits: Vec<PropertyCommit>,
    identity: NativeIdentity,
    retry_properties: bool,
}

impl UpdatePlan {
    fn new(identity: NativeIdentity) -> Self {
        Self {
            commands: Vec::new(),
            commits: Vec::new(),
            identity,
            retry_properties: false,
        }
    }

    fn push(&mut self, command: Command) -> usize {
        let index = self.commands.len();
        self.commands.push(command);
        index
    }
}

pub struct Pump<R: NativeRuntime> {
    application: Option<NodeId>,
    element: Option<Element>,
    tree: Tree,
    runtime: R,
    root: Option<NodeId>,
    events: VecDeque<NativeWork<QueuedEvent>>,
    identity: NativeIdentity,
    poisoned: bool,
    retry_pending: bool,
    version: u64,
    window: Option<NodeId>,
}

impl<R: NativeRuntime> Pump<R> {
    pub fn new(mut runtime: R) -> Self {
        let identity = NativeIdentity::new(WindowToken::new(WindowId::allocate()));
        runtime.set_identity(identity);
        Self {
            application: None,
            element: None,
            tree: Tree::new(),
            runtime,
            root: None,
            events: VecDeque::new(),
            identity,
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
        let desired = element.clone();
        let mut candidate = Tree::new();
        let mut plan = UpdatePlan::new(self.identity);
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
            self.element = None;
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
            self.element = None;
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
        self.element = Some(desired);
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
        if !self.retry_pending && self.element.as_ref() == Some(&element) {
            self.version = next_version;
            return Ok(CommitReceipt {
                outcomes: Vec::new(),
            });
        }
        let node = self.root.ok_or(PumpError::NotMounted)?;
        let recovery_element = element.clone();
        let mut candidate = self.tree.clone();
        let mut plan = UpdatePlan {
            retry_properties: self.retry_pending,
            ..UpdatePlan::new(self.identity)
        };
        let candidate_root = Self::reconcile_node(&mut candidate, node, element, &mut plan)?;
        if plan.commands.is_empty() {
            self.tree = candidate;
            self.root = Some(candidate_root);
            self.element = Some(recovery_element);
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
            return self.recover_structure(
                candidate,
                candidate_root,
                recovery_element,
                receipt,
                next_version,
            );
        }

        Self::commit_tree_properties(&mut candidate, &plan.commits, &receipt)?;
        self.tree = candidate;
        self.root = Some(candidate_root);
        self.element = Some(recovery_element);
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

    fn recover_structure(
        &mut self,
        mut candidate: Tree,
        failed_root: NodeId,
        element: Element,
        failure: CommitReceipt,
        next_version: u64,
    ) -> Result<CommitReceipt, PumpError> {
        let window = self.window.ok_or(PumpError::NotMounted)?;
        let desired = element.clone();
        candidate.retire_subtree(failed_root)?;

        let recovery_identity = self
            .identity
            .next_realization()
            .ok_or(PumpError::RevisionExhausted)?;
        let mut plan = UpdatePlan::new(recovery_identity);
        plan.push(Command::ResetWindowContent { window });
        let root =
            Self::mount_planned_element(&mut candidate, Some(window), None, element, &mut plan)?;
        plan.push(Command::InsertChild {
            parent: window,
            child: root,
            index: 0,
        });

        self.events.clear();
        self.identity = recovery_identity;
        self.runtime.set_identity(recovery_identity);
        let recovery = self.runtime.apply(&plan.commands);
        let attempt = |recovery| {
            Box::new(StructuralRecovery {
                failure: failure.clone(),
                recovery,
                root,
            })
        };
        if recovery.outcomes.len() != plan.commands.len() {
            self.poisoned = true;
            self.retry_pending = false;
            return Err(PumpError::RecoveryFailed(attempt(recovery)));
        }
        let structural_failure = plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, command)| command.structural() && !recovery.applied(index));
        if structural_failure {
            self.poisoned = true;
            self.retry_pending = false;
            return Err(PumpError::RecoveryFailed(attempt(recovery)));
        }

        Self::commit_tree_properties(&mut candidate, &plan.commits, &recovery)?;
        self.tree = candidate;
        self.element = Some(desired);
        self.root = Some(root);
        self.retry_pending = plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, command)| !command.structural() && !recovery.applied(index));
        if !self.retry_pending {
            self.version = next_version;
        }
        Err(PumpError::RecoveredStructure(attempt(recovery)))
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
        let identity = self.identity.next_window();
        self.runtime.reset();
        self.application = None;
        self.element = None;
        self.events.clear();
        self.retry_pending = false;
        self.root = None;
        self.tree = Tree::new();
        self.version = 0;
        self.window = None;
        if let Some(identity) = identity {
            self.identity = identity;
            self.runtime.set_identity(identity);
            self.poisoned = false;
        } else {
            self.poisoned = true;
        }
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
        self.events.push_back(NativeWork {
            identity: self.identity,
            work: event,
        });
    }

    pub fn native_identity(&self) -> NativeIdentity {
        self.identity
    }

    fn queue_event_with_identity(&mut self, identity: NativeIdentity, event: QueuedEvent) {
        self.events.push_back(NativeWork {
            identity,
            work: event,
        });
    }

    pub fn dispatch_events(&mut self) -> Result<usize, PumpError> {
        self.events.extend(self.runtime.drain_events());
        if self.poisoned {
            self.events.clear();
            _ = self.runtime.drain_event_errors();
            _ = self.runtime.drain_realizations();
            return Ok(0);
        }
        self.process_realizations()?;
        if let Some(error) = self.runtime.drain_event_errors().into_iter().next() {
            self.events.clear();
            return Err(PumpError::EventReadFailed(error));
        }
        let mut dispatched = 0;
        while let Some(queued) = self.events.pop_front() {
            if queued.identity != self.identity {
                continue;
            }
            let event = queued.work;
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

    pub fn process_realizations(&mut self) -> Result<Vec<RealizationOutcome>, PumpError> {
        if self.poisoned {
            _ = self.runtime.drain_realizations();
            return Err(PumpError::Poisoned);
        }
        let requests = self.runtime.drain_realizations();
        let mut outcomes = Vec::with_capacity(requests.len());
        let mut candidate = self.tree.clone();
        let mut plan = UpdatePlan::new(self.identity);
        for queued in requests {
            let request = queued.work;
            if queued.identity != self.identity {
                outcomes.push(RealizationOutcome::Rejected(request));
                continue;
            }
            let outcome = match request {
                RealizationRequest::Realize {
                    collection,
                    container,
                    index,
                } => {
                    let Ok(lease) = candidate
                        .virtual_model_mut(collection)
                        .and_then(|model| model.realize(index, container).map_err(TreeError::from))
                    else {
                        outcomes.push(RealizationOutcome::Rejected(request));
                        continue;
                    };
                    let element = candidate.virtual_item(collection, &lease.key)?.clone();
                    let stale = candidate
                        .children(collection)?
                        .iter()
                        .copied()
                        .filter(|child| {
                            candidate.key(*child).ok().flatten() == Some(&lease.key)
                                || candidate.realized(collection, container).ok().flatten()
                                    == Some(*child)
                        })
                        .collect::<Vec<_>>();
                    for old in stale {
                        Self::retire_planned_subtree(&mut candidate, old, &mut plan)?;
                    }
                    let child = Self::mount_planned_element(
                        &mut candidate,
                        Some(collection),
                        Some(lease.key.clone()),
                        element,
                        &mut plan,
                    )?;
                    candidate.set_realized(collection, container, child)?;
                    plan.push(Command::AttachRealized {
                        collection,
                        container,
                        child,
                    });
                    RealizationOutcome::Realized(lease)
                }
                RealizationRequest::Recycle {
                    collection,
                    container,
                } => {
                    let Some(child) = candidate.realized(collection, container)? else {
                        outcomes.push(RealizationOutcome::Rejected(request));
                        continue;
                    };
                    let Some(lease) = candidate
                        .virtual_model_mut(collection)
                        .ok()
                        .and_then(|model| model.recycle_container(container))
                    else {
                        outcomes.push(RealizationOutcome::Rejected(request));
                        continue;
                    };
                    Self::retire_planned_subtree(&mut candidate, child, &mut plan)?;
                    RealizationOutcome::Recycled(lease)
                }
            };
            outcomes.push(outcome);
        }
        if !plan.commands.is_empty() {
            self.apply_realization(candidate, &plan)?;
        }
        Ok(outcomes)
    }

    fn apply_realization(
        &mut self,
        mut candidate: Tree,
        plan: &UpdatePlan,
    ) -> Result<(), PumpError> {
        let receipt = self.runtime.apply(&plan.commands);
        if receipt.outcomes.len() != plan.commands.len() {
            self.poisoned = true;
            return Err(PumpError::ApplyReceiptMismatch);
        }
        if plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, _)| !receipt.applied(index))
        {
            self.poisoned = true;
            return Err(PumpError::StructuralApplyFailed(receipt));
        }
        Self::commit_tree_properties(&mut candidate, &plan.commits, &receipt)?;
        self.tree = candidate;
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
    ) -> Result<NodeId, PumpError> {
        let desired_kind = element.kind();
        let compatible = match tree.kind(node)? {
            NodeKind::Native(kind) => kind == desired_kind,
            NodeKind::VirtualCollection => desired_kind == MountedKind::ItemsRepeater,
            _ => false,
        };
        if !compatible {
            return Self::replace_planned_node(tree, node, element, plan);
        }
        if !plan.retry_properties && Self::node_matches_element(tree, node, &element)? {
            return Ok(node);
        }

        let parts = element.into_parts();
        if tree.kind(node)? == NodeKind::VirtualCollection {
            let ElementStructure::Virtual(items) = parts.structure else {
                return Err(PumpError::StructureUnsupported);
            };
            let old_keys = tree.virtual_model(node)?.keys();
            let keys_changed = old_keys.len() != items.len()
                || old_keys
                    .iter()
                    .zip(items.iter())
                    .any(|(old, new)| old != new.key());
            if keys_changed {
                for child in tree.children(node)?.to_vec() {
                    Self::retire_planned_subtree(tree, child, plan)?;
                }
                let keys = items.iter().map(|item| item.key().clone());
                tree.virtual_model_mut(node)?
                    .update(keys)
                    .map_err(TreeError::from)?;
                tree.update_virtual_items(node, items)?;
                tree.virtual_model_mut(node)?.clear();
                plan.push(Command::ResetVirtualCollection {
                    node,
                    item_count: tree.virtual_items(node)?.len(),
                });
            } else {
                tree.update_virtual_items(node, items)?;
                let realized = tree
                    .children(node)?
                    .iter()
                    .copied()
                    .map(|child| {
                        let key = tree
                            .key(child)?
                            .cloned()
                            .ok_or(PumpError::StructureUnsupported)?;
                        let element = tree.virtual_item(node, &key)?.clone();
                        Ok((child, element))
                    })
                    .collect::<Result<Vec<_>, PumpError>>()?;
                for (child, element) in realized {
                    Self::reconcile_node(tree, child, element, plan)?;
                }
            }
            return Ok(node);
        }
        let NodeKind::Native(kind) = tree.kind(node)? else {
            return Err(PumpError::NotMounted);
        };
        debug_assert_eq!(kind, parts.kind);

        let props_changed = tree.native(node)?.desired != parts.props;
        if props_changed || plan.retry_properties {
            let committed = &tree.native(node)?.committed;
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
        }
        if props_changed {
            Self::update_event_states(tree.native_mut(node)?, node, &parts.props, plan)?;
            tree.native_mut(node)?.desired = parts.props;
        }

        let current_children = tree.children(node)?.to_vec();
        match parts.structure {
            ElementStructure::None => {
                if !current_children.is_empty() {
                    return Err(PumpError::StructureUnsupported);
                }
            }
            ElementStructure::Content(content) => match (current_children.as_slice(), content) {
                ([], None) => {}
                ([], Some(content)) => {
                    let child = Self::mount_planned_element(tree, Some(node), None, content, plan)?;
                    plan.push(Command::InsertChild {
                        parent: node,
                        child,
                        index: 0,
                    });
                }
                ([child], None) => {
                    Self::retire_planned_subtree(tree, *child, plan)?;
                }
                ([child], Some(content)) => {
                    Self::reconcile_node(tree, *child, content, plan)?;
                }
                _ => return Err(PumpError::StructureUnsupported),
            },
            ElementStructure::Children(children) => {
                if current_children.len() == children.len()
                    && current_children
                        .iter()
                        .zip(children.iter())
                        .all(|(child, desired)| {
                            tree.key(*child).is_ok_and(|key| key == Some(desired.key()))
                        })
                {
                    let mut replacements = Vec::new();
                    for (index, (child, desired)) in current_children
                        .iter()
                        .copied()
                        .zip(children.iter())
                        .enumerate()
                    {
                        if !plan.retry_properties
                            && Self::node_matches_element(tree, child, desired.element())?
                        {
                        } else {
                            let reconciled =
                                Self::reconcile_node(tree, child, desired.element().clone(), plan)?;
                            if reconciled != child {
                                replacements.push((index, reconciled));
                            }
                        }
                    }
                    if !replacements.is_empty() {
                        let mut children = current_children;
                        for (index, replacement) in replacements {
                            children[index] = replacement;
                        }
                        tree.set_children(node, children)?;
                    }
                    return Ok(node);
                }

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

                let new_keys = children
                    .iter()
                    .map(|child| child.key().clone())
                    .collect::<Vec<_>>();
                let operations = diff(&old_keys, &new_keys)
                    .map_err(|KeyedError::DuplicateKey(key)| PumpError::DuplicateKey(key))?;

                let mut elements = children
                    .iter()
                    .map(|child| (child.key().clone(), child.element()))
                    .collect::<HashMap<_, _>>();
                let mut replacements = HashMap::new();
                for key in &new_keys {
                    if let Some(child_node) = nodes.get(key).copied() {
                        let child = elements
                            .remove(key)
                            .ok_or(PumpError::StructureUnsupported)?;
                        let reconciled = if !plan.retry_properties
                            && Self::node_matches_element(tree, child_node, child)?
                        {
                            child_node
                        } else {
                            Self::reconcile_node(tree, child_node, child.clone(), plan)?
                        };
                        if reconciled != child_node {
                            nodes.insert(key.clone(), reconciled);
                            replacements.insert(child_node, reconciled);
                        }
                    }
                }

                let mut order = current_children
                    .into_iter()
                    .map(|child| replacements.get(&child).copied().unwrap_or(child))
                    .collect::<Vec<_>>();
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
                                element.clone(),
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
            ElementStructure::Virtual(_) => return Err(PumpError::StructureUnsupported),
        }
        Ok(node)
    }

    fn node_matches_element(
        tree: &Tree,
        node: NodeId,
        element: &Element,
    ) -> Result<bool, PumpError> {
        let kind = tree.kind(node)?;
        let compatible = match kind {
            NodeKind::Native(mounted) => mounted == element.kind(),
            NodeKind::VirtualCollection => element.kind() == MountedKind::ItemsRepeater,
            _ => false,
        };
        if !compatible {
            return Ok(false);
        }
        if kind == NodeKind::VirtualCollection {
            let ElementStructureRef::Virtual(items) = element.structure() else {
                return Ok(false);
            };
            return Ok(tree.virtual_items(node)? == items);
        }
        if !element.props_match(&tree.native(node)?.desired) {
            return Ok(false);
        }

        let children = tree.children(node)?;
        match element.structure() {
            ElementStructureRef::None => Ok(children.is_empty()),
            ElementStructureRef::Content(content) => match (children, content) {
                ([], None) => Ok(true),
                ([child], Some(content)) => Self::node_matches_element(tree, *child, content),
                _ => Ok(false),
            },
            ElementStructureRef::Children(desired) => {
                if children.len() != desired.len() {
                    return Ok(false);
                }
                for (child, desired) in children.iter().zip(desired) {
                    if tree.key(*child)? != Some(desired.key())
                        || !Self::node_matches_element(tree, *child, desired.element())?
                    {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            ElementStructureRef::Virtual(_) => Ok(false),
        }
    }

    fn replace_planned_node(
        tree: &mut Tree,
        node: NodeId,
        element: Element,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        let parent = tree.parent(node)?.ok_or(PumpError::StructureUnsupported)?;
        let key = tree.key(node)?.cloned();
        let container = if tree.kind(parent)? == NodeKind::VirtualCollection {
            Some(
                tree.realized_container(parent, node)?
                    .ok_or(PumpError::StructureUnsupported)?,
            )
        } else {
            None
        };
        let index = tree
            .children(parent)?
            .iter()
            .position(|child| *child == node)
            .ok_or(PumpError::StructureUnsupported)?;
        Self::retire_planned_subtree(tree, node, plan)?;
        let replacement = Self::mount_planned_element(tree, Some(parent), key, element, plan)?;
        let mut children = tree.children(parent)?.to_vec();
        let appended = children
            .iter()
            .position(|child| *child == replacement)
            .ok_or(PumpError::StructureUnsupported)?;
        children.remove(appended);
        children.insert(index, replacement);
        tree.set_children(parent, children)?;
        if let Some(container) = container {
            tree.set_realized(parent, container, replacement)?;
            plan.push(Command::AttachRealized {
                collection: parent,
                container,
                child: replacement,
            });
        } else {
            plan.push(Command::InsertChild {
                parent,
                child: replacement,
                index,
            });
        }
        Ok(replacement)
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
        let nodes = tree.subtree_postorder(root)?;
        plan.commits.retain(|commit| !nodes.contains(&commit.node));
        for node in nodes {
            if let Some(parent) = tree.parent(node)? {
                if tree.kind(parent)? == NodeKind::VirtualCollection {
                    let container = tree
                        .realized_container(parent, node)?
                        .ok_or(PumpError::StructureUnsupported)?;
                    plan.push(Command::DetachRealized {
                        collection: parent,
                        container,
                        child: node,
                    });
                } else {
                    plan.push(Command::RemoveChild {
                        parent,
                        child: node,
                    });
                }
            }

            match tree.kind(node)? {
                NodeKind::Native(_) => {
                    for (event, state) in &tree.native(node)?.events {
                        if state.active {
                            plan.push(Command::UnsubscribeEvent {
                                node,
                                event: *event,
                            });
                        }
                    }
                }
                NodeKind::VirtualCollection => {}
                _ => return Err(PumpError::StructureUnsupported),
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
        if let ElementStructure::Virtual(items) = parts.structure {
            if parts.kind != MountedKind::ItemsRepeater {
                return Err(PumpError::StructureUnsupported);
            }
            let item_count = items.len();
            let node = tree.insert_virtual_items(plan.identity, parent, key, items)?;
            plan.push(Command::CreateVirtualCollection { node, item_count });
            return Ok(node);
        }
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
                let children = Rc::unwrap_or_clone(children);
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
            ElementStructure::Virtual(_) => return Err(PumpError::StructureUnsupported),
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
    use std::collections::HashSet;
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

    fn recovered_structure(error: PumpError) -> StructuralRecovery {
        let PumpError::RecoveredStructure(recovery) = error else {
            panic!("expected recovered structure");
        };
        *recovery
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
            let old_root = pump.root().unwrap();
            let application = pump.application();
            let window = pump.window();
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
                PumpError::RecoveredStructure(recovery) => {
                    assert!(matches!(
                        recovery.failure.outcomes[failed_index],
                        CommandOutcome::Failed(RuntimeError::Injected)
                    ));
                    assert_eq!(pump.version(), version + 1);
                    assert!(!pump.retry_pending());
                    assert!(!pump.poisoned());
                    assert_ne!(pump.root(), Some(old_root));
                    assert_eq!(pump.application(), application);
                    assert_eq!(pump.window(), window);
                    assert_eq!(
                        arena_keys(&pump),
                        [Key::from("c"), Key::from("d"), Key::from("a")]
                    );
                    assert_eq!(
                        recorded_text(pump.runtime(), pump.root().unwrap())[0],
                        "c updated"
                    );
                }
                error => panic!("unexpected update failure: {error:?}"),
            }
        }
    }

    #[test]
    fn every_recovery_command_failure_reaches_a_defined_state() {
        let before = keyed_text(&["a", "b", "c"]);
        let after: Element = StackPanel::new()
            .child("c", TextBlock::new().text("c updated"))
            .child("d", TextBlock::new().text("d"))
            .child("a", TextBlock::new().text("a"))
            .into();
        let mut baseline = Pump::new(RecordingRuntime::default());
        baseline.mount(before.clone()).unwrap();
        baseline.runtime_mut().fail_at(1);
        let recovered = recovered_structure(baseline.update(after.clone()).unwrap_err());
        let command_count = recovered.recovery.outcomes.len();
        let mut saw_property = false;
        let mut saw_structural = false;

        for failed_index in 0..command_count {
            let mut pump = Pump::new(RecordingRuntime::default());
            pump.mount(before.clone()).unwrap();
            let version = pump.version();
            let old_root = pump.root();
            pump.runtime_mut().fail_at(1);
            pump.runtime_mut().fail_after(1, failed_index);

            match pump.update(after.clone()).unwrap_err() {
                PumpError::RecoveredStructure(recovery) => {
                    saw_property = true;
                    assert!(matches!(
                        recovery.recovery.outcomes[failed_index],
                        CommandOutcome::Failed(RuntimeError::Injected)
                    ));
                    assert_eq!(pump.version(), version);
                    assert!(pump.retry_pending());
                    assert!(!pump.poisoned());
                    assert_ne!(pump.root(), old_root);
                }
                PumpError::RecoveryFailed(recovery) => {
                    saw_structural = true;
                    assert!(matches!(
                        recovery.recovery.outcomes[failed_index],
                        CommandOutcome::Failed(RuntimeError::Injected)
                    ));
                    assert_eq!(pump.version(), version);
                    assert!(!pump.retry_pending());
                    assert!(pump.poisoned());
                    assert_eq!(pump.root(), old_root);
                }
                error => panic!("unexpected recovery failure: {error:?}"),
            }
        }

        assert!(saw_property);
        assert!(saw_structural);
    }

    #[test]
    fn recovery_does_not_reuse_ids_created_by_failed_batch() {
        let before = keyed_text(&["a", "b", "c"]);
        let after: Element = StackPanel::new()
            .child("c", TextBlock::new().text("c"))
            .child("d", TextBlock::new().text("d"))
            .child("a", TextBlock::new().text("a"))
            .into();
        let mut probe = Pump::new(RecordingRuntime::default());
        probe.mount(before.clone()).unwrap();
        probe.update(after.clone()).unwrap();
        let update = &probe.runtime().commands()[1];
        let failed_index = update.iter().rposition(Command::structural).unwrap();

        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(before).unwrap();
        pump.runtime_mut().fail_at(failed_index);
        assert!(matches!(
            pump.update(after),
            Err(PumpError::RecoveredStructure(_))
        ));
        let batches = pump.runtime().commands();
        let failed_created = batches[1]
            .iter()
            .filter_map(|command| match command {
                Command::Create { node, .. } => Some(*node),
                _ => None,
            })
            .collect::<HashSet<_>>();
        let recovered_created = batches[2]
            .iter()
            .filter_map(|command| match command {
                Command::Create { node, .. } => Some(*node),
                _ => None,
            })
            .collect::<HashSet<_>>();

        assert!(!failed_created.is_empty());
        assert!(failed_created.is_disjoint(&recovered_created));
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
    fn failed_keyed_move_remounts_with_fresh_root() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(keyed_text(&["a", "b", "c", "d"])).unwrap();
        let version = pump.version();
        let old_root = pump.root().unwrap();
        pump.runtime_mut().fail_at(1);

        let recovered =
            recovered_structure(pump.update(keyed_text(&["d", "c", "b", "a"])).unwrap_err());

        assert!(matches!(
            recovered.failure.outcomes[1],
            CommandOutcome::Failed(RuntimeError::Injected)
        ));
        assert_eq!(pump.version(), version + 1);
        assert!(!pump.poisoned());
        assert_ne!(pump.root(), Some(old_root));
        assert_eq!(
            recorded_text(pump.runtime(), pump.root().unwrap()),
            ["d", "c", "b", "a"]
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
    fn failed_keyed_insert_remounts_with_fresh_root() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(keyed_text(&["a", "c"])).unwrap();
        let version = pump.version();
        let old_root = pump.root().unwrap();
        pump.runtime_mut().fail_at(2);

        let recovered = recovered_structure(pump.update(keyed_text(&["a", "b", "c"])).unwrap_err());

        assert!(matches!(
            recovered.failure.outcomes[2],
            CommandOutcome::Failed(RuntimeError::Injected)
        ));
        assert_eq!(pump.version(), version + 1);
        assert!(!pump.poisoned());
        assert_ne!(pump.root(), Some(old_root));
        assert_eq!(
            recorded_text(pump.runtime(), pump.root().unwrap()),
            ["a", "b", "c"]
        );
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
    fn failed_keyed_remove_remounts_with_fresh_root() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(keyed_text(&["a", "b", "c"])).unwrap();
        let version = pump.version();
        let old_root = pump.root().unwrap();
        pump.runtime_mut().fail_at(1);

        let recovered = recovered_structure(pump.update(keyed_text(&["a", "c"])).unwrap_err());

        assert!(matches!(
            recovered.failure.outcomes[1],
            CommandOutcome::Failed(RuntimeError::Injected)
        ));
        assert_eq!(pump.version(), version + 1);
        assert!(!pump.poisoned());
        assert_ne!(pump.root(), Some(old_root));
        assert_eq!(
            recorded_text(pump.runtime(), pump.root().unwrap()),
            ["a", "c"]
        );
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
    fn recovery_failure_poisons_and_discards_queued_events() {
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
        pump.runtime_mut().fail_after(1, 0);
        assert!(matches!(
            pump.update(Button::new().into()),
            Err(PumpError::RecoveryFailed(_))
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
    fn recovered_root_rejects_pre_failure_event() {
        let calls = Rc::new(Cell::new(0));
        let callback_calls = Rc::clone(&calls);
        let element = || {
            Button::new()
                .on_click({
                    let callback_calls = Rc::clone(&callback_calls);
                    move || callback_calls.set(callback_calls.get() + 1)
                })
                .into()
        };
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(element()).unwrap();
        let old_root = pump.root().unwrap();
        let revision = pump.event_revision(old_root, EventId::ButtonClick).unwrap();
        pump.queue_event(QueuedEvent {
            node: old_root,
            event: EventId::ButtonClick,
            revision,
            payload: EventPayload::Unit,
        });
        pump.runtime_mut().fail_at(0);

        assert!(matches!(
            pump.update(Button::new().into()),
            Err(PumpError::RecoveredStructure(_))
        ));
        assert_ne!(pump.root(), Some(old_root));
        assert_eq!(pump.dispatch_events(), Ok(0));
        assert_eq!(calls.get(), 0);
    }

    #[test]
    fn native_remount_rejects_late_work_without_replacing_window_identity() {
        let calls = Rc::new(Cell::new(0));
        let callback_calls = Rc::clone(&calls);
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(Button::new().on_click(|| {}).into()).unwrap();
        let old_identity = pump.native_identity();
        pump.runtime_mut().fail_at(0);

        assert!(matches!(
            pump.update(
                Button::new()
                    .content(TextBlock::new())
                    .on_click(move || callback_calls.set(callback_calls.get() + 1))
                    .into()
            ),
            Err(PumpError::RecoveredStructure(_))
        ));

        let new_identity = pump.native_identity();
        assert_eq!(old_identity.window(), new_identity.window());
        assert_ne!(
            old_identity.realization_epoch(),
            new_identity.realization_epoch()
        );
        let root = pump.root().unwrap();
        let revision = pump.event_revision(root, EventId::ButtonClick).unwrap();
        pump.queue_event_with_identity(
            old_identity,
            QueuedEvent {
                node: root,
                event: EventId::ButtonClick,
                revision,
                payload: EventPayload::Unit,
            },
        );

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

    #[test]
    fn realization_requests_are_checked_against_arena_and_container_generations() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(
            ItemsRepeater::new()
                .item("a", TextBlock::new().text("A"))
                .item("b", TextBlock::new().text("B"))
                .into(),
        )
        .unwrap();
        let collection = pump.root().unwrap();
        let container = RealizedContainer(7);
        pump.runtime_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container,
                index: 1,
            });

        let realized = pump.process_realizations().unwrap();
        let RealizationOutcome::Realized(lease) = &realized[0] else {
            panic!("expected realized lease");
        };
        assert_eq!(lease.key, Key::from("b"));
        assert_eq!(lease.container, container);

        pump.runtime_mut()
            .queue_realization(RealizationRequest::Recycle {
                collection,
                container,
            });
        pump.runtime_mut()
            .queue_realization(RealizationRequest::Recycle {
                collection,
                container,
            });
        assert!(matches!(
            pump.process_realizations().unwrap().as_slice(),
            [
                RealizationOutcome::Recycled(_),
                RealizationOutcome::Rejected(_)
            ]
        ));

        pump.tree.retire_subtree(collection).unwrap();
        pump.runtime_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container,
                index: 0,
            });
        assert_eq!(
            pump.process_realizations().unwrap(),
            [RealizationOutcome::Rejected(RealizationRequest::Realize {
                collection,
                container,
                index: 0,
            })]
        );
    }

    #[test]
    fn virtual_collection_mounts_without_eager_row_controls() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(
            ItemsRepeater::new()
                .item("a", TextBlock::new().text("A"))
                .item("b", TextBlock::new().text("B"))
                .into(),
        )
        .unwrap();

        let root = pump.root().unwrap();
        assert_eq!(pump.tree.kind(root), Ok(NodeKind::VirtualCollection));
        assert_eq!(pump.tree.virtual_items(root).unwrap().len(), 2);
        assert!(pump.runtime().commands()[0].iter().any(|command| {
            *command
                == Command::CreateVirtualCollection {
                    node: root,
                    item_count: 2,
                }
        }));
        assert!(!pump.runtime().commands()[0].iter().any(|command| {
            matches!(
                command,
                Command::Create {
                    kind: MountedKind::TextBlock,
                    ..
                }
            )
        }));
    }

    #[test]
    fn virtual_collection_update_resets_source_and_rejects_old_leases() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(
            ItemsRepeater::new()
                .item("a", TextBlock::new().text("A"))
                .item("b", TextBlock::new().text("B"))
                .into(),
        )
        .unwrap();
        let root = pump.root().unwrap();
        let old = pump
            .tree
            .virtual_model_mut(root)
            .unwrap()
            .realize(0, RealizedContainer(1))
            .unwrap();

        pump.update(
            ItemsRepeater::new()
                .item("b", TextBlock::new().text("B2"))
                .item("c", TextBlock::new().text("C"))
                .into(),
        )
        .unwrap();

        assert!(!pump.tree.virtual_model(root).unwrap().accepts(&old));
        assert_eq!(
            pump.runtime().commands().last().unwrap(),
            &[Command::ResetVirtualCollection {
                node: root,
                item_count: 2,
            }]
        );
    }

    #[test]
    fn every_realization_command_failure_poisoned_without_publication() {
        for command in 0..3 {
            let mut pump = Pump::new(RecordingRuntime::default());
            pump.mount(
                ItemsRepeater::new()
                    .item("a", TextBlock::new().text("A"))
                    .into(),
            )
            .unwrap();
            let collection = pump.root().unwrap();
            pump.runtime_mut().fail_after(0, command);
            pump.runtime_mut()
                .queue_realization(RealizationRequest::Realize {
                    collection,
                    container: RealizedContainer(1),
                    index: 0,
                });

            assert!(matches!(
                pump.process_realizations(),
                Err(PumpError::StructuralApplyFailed(_))
            ));
            assert!(pump.tree.children(collection).unwrap().is_empty());
            assert_eq!(pump.process_realizations(), Err(PumpError::Poisoned));
        }
    }

    #[test]
    fn root_kind_replacement_updates_arena_and_native_parent() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(TextBlock::new().text("first").into()).unwrap();
        let old = pump.root().unwrap();
        let window = pump.window().unwrap();

        pump.update(Button::new().into()).unwrap();

        let root = pump.root().unwrap();
        assert_ne!(root, old);
        assert_eq!(
            pump.tree.kind(root),
            Ok(NodeKind::Native(MountedKind::Button))
        );
        assert_eq!(pump.runtime().node(window).unwrap().children(), &[root]);
    }

    #[test]
    fn content_transitions_support_insert_replace_and_remove() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(Button::new().into()).unwrap();
        let root = pump.root().unwrap();

        pump.update(Button::new().content(TextBlock::new().text("text")).into())
            .unwrap();
        let text = pump.tree.children(root).unwrap()[0];
        assert_eq!(
            pump.tree.kind(text),
            Ok(NodeKind::Native(MountedKind::TextBlock))
        );

        pump.update(Button::new().content(Button::new()).into())
            .unwrap();
        let button = pump.tree.children(root).unwrap()[0];
        assert_ne!(button, text);
        assert_eq!(
            pump.tree.kind(button),
            Ok(NodeKind::Native(MountedKind::Button))
        );

        pump.update(Button::new().into()).unwrap();
        assert!(pump.tree.children(root).unwrap().is_empty());
        assert!(pump.runtime().node(root).unwrap().children().is_empty());
    }

    #[test]
    fn same_key_child_can_change_kind_without_replacing_panel() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(
            StackPanel::new()
                .child("item", TextBlock::new().text("text"))
                .into(),
        )
        .unwrap();
        let root = pump.root().unwrap();
        let old = pump.tree.children(root).unwrap()[0];

        pump.update(StackPanel::new().child("item", Button::new()).into())
            .unwrap();

        let child = pump.tree.children(root).unwrap()[0];
        assert_eq!(pump.root(), Some(root));
        assert_ne!(child, old);
        assert_eq!(
            pump.tree.kind(child),
            Ok(NodeKind::Native(MountedKind::Button))
        );
        assert_eq!(pump.runtime().node(root).unwrap().children(), &[child]);
    }

    #[test]
    fn failed_root_replacement_recovers_from_candidate_root() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(TextBlock::new().text("first").into()).unwrap();
        let old = pump.root().unwrap();
        pump.runtime_mut().fail_at(0);

        assert!(matches!(
            pump.update(Button::new().into()),
            Err(PumpError::RecoveredStructure(_))
        ));
        let root = pump.root().unwrap();
        assert_ne!(root, old);
        assert_eq!(
            pump.tree.kind(root),
            Ok(NodeKind::Native(MountedKind::Button))
        );
    }

    #[test]
    fn every_root_replacement_command_failure_recovers_desired_kind() {
        let command_count = {
            let mut pump = Pump::new(RecordingRuntime::default());
            pump.mount(TextBlock::new().into()).unwrap();
            pump.update(Button::new().into()).unwrap();
            pump.runtime().commands().last().unwrap().len()
        };

        for failed in 0..command_count {
            let mut pump = Pump::new(RecordingRuntime::default());
            pump.mount(TextBlock::new().into()).unwrap();
            pump.runtime_mut().fail_at(failed);

            assert!(matches!(
                pump.update(Button::new().into()),
                Err(PumpError::RecoveredStructure(_))
            ));
            assert_eq!(
                pump.tree.kind(pump.root().unwrap()),
                Ok(NodeKind::Native(MountedKind::Button)),
                "command {failed}"
            );
            assert!(!pump.poisoned(), "command {failed}");
        }
    }

    #[test]
    fn every_content_replacement_command_failure_recovers_desired_tree() {
        let before = || {
            Button::new()
                .content(TextBlock::new().text("before"))
                .into()
        };
        let after = || Button::new().content(Button::new()).into();
        let command_count = {
            let mut pump = Pump::new(RecordingRuntime::default());
            pump.mount(before()).unwrap();
            pump.update(after()).unwrap();
            pump.runtime().commands().last().unwrap().len()
        };

        for failed in 0..command_count {
            let mut pump = Pump::new(RecordingRuntime::default());
            pump.mount(before()).unwrap();
            pump.runtime_mut().fail_at(failed);

            assert!(matches!(
                pump.update(after()),
                Err(PumpError::RecoveredStructure(_))
            ));
            let root = pump.root().unwrap();
            let child = pump.tree.children(root).unwrap()[0];
            assert_eq!(
                pump.tree.kind(child),
                Ok(NodeKind::Native(MountedKind::Button)),
                "command {failed}"
            );
            assert!(!pump.poisoned(), "command {failed}");
        }
    }

    #[test]
    fn shutdown_discards_pending_realization_and_rejects_stale_collection() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(
            ItemsRepeater::new()
                .item("a", TextBlock::new().text("A"))
                .into(),
        )
        .unwrap();
        let collection = pump.root().unwrap();
        let request = RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
        };
        let old_identity = pump.native_identity();
        pump.runtime_mut().queue_realization(request);

        pump.shutdown();

        assert!(pump.process_realizations().unwrap().is_empty());
        pump.mount(
            ItemsRepeater::new()
                .item("a", TextBlock::new().text("A"))
                .into(),
        )
        .unwrap();
        assert_eq!(pump.root(), Some(collection));
        assert_ne!(pump.native_identity().window(), old_identity.window());
        pump.runtime_mut()
            .queue_realization_with_identity(old_identity, request);
        assert_eq!(
            pump.process_realizations().unwrap(),
            [RealizationOutcome::Rejected(request)]
        );
        pump.runtime_mut().queue_realization(request);
        assert!(matches!(
            pump.process_realizations().unwrap().as_slice(),
            [RealizationOutcome::Realized(_)]
        ));
    }

    #[test]
    fn repeated_virtual_mount_realize_recycle_shutdown_returns_to_zero() {
        let mut pump = Pump::new(RecordingRuntime::default());
        for cycle in 0..100 {
            pump.mount(
                ItemsRepeater::new()
                    .item("a", TextBlock::new().text("A"))
                    .item("b", Button::new())
                    .into(),
            )
            .unwrap();
            let collection = pump.root().unwrap();
            for index in 0..2 {
                pump.runtime_mut()
                    .queue_realization(RealizationRequest::Realize {
                        collection,
                        container: RealizedContainer(index),
                        index: index as usize,
                    });
            }
            assert_eq!(pump.process_realizations().unwrap().len(), 2);
            for index in 0..2 {
                pump.runtime_mut()
                    .queue_realization(RealizationRequest::Recycle {
                        collection,
                        container: RealizedContainer(index),
                    });
            }
            assert_eq!(pump.process_realizations().unwrap().len(), 2);

            pump.shutdown();

            assert_eq!(pump.tree.len(), 0, "cycle {cycle}");
            assert!(pump.runtime().is_empty(), "cycle {cycle}");
        }
    }

    #[test]
    fn virtual_source_reset_retires_realized_rows_before_clearing_leases() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(
            ItemsRepeater::new()
                .item("a", TextBlock::new().text("A"))
                .item("b", TextBlock::new().text("B"))
                .into(),
        )
        .unwrap();
        let collection = pump.root().unwrap();
        for index in 0..2 {
            pump.runtime_mut()
                .queue_realization(RealizationRequest::Realize {
                    collection,
                    container: RealizedContainer(index),
                    index: index as usize,
                });
        }
        pump.process_realizations().unwrap();
        let realized = pump.tree.children(collection).unwrap().to_vec();
        assert_eq!(realized.len(), 2);

        pump.update(
            ItemsRepeater::new()
                .item("z", TextBlock::new().text("Z"))
                .item("b", TextBlock::new().text("B"))
                .into(),
        )
        .unwrap();

        assert!(pump.tree.children(collection).unwrap().is_empty());
        assert_eq!(pump.tree.virtual_model(collection).unwrap().active_len(), 0);
        assert!(
            pump.runtime()
                .node(collection)
                .unwrap()
                .children()
                .is_empty()
        );
        for child in realized {
            assert!(pump.runtime().node(child).is_none());
        }
    }

    #[test]
    fn virtual_payload_change_reconciles_rows_without_resetting_source() {
        let first = Rc::new(Cell::new(0));
        let second = Rc::new(Cell::new(0));
        let first_callback = Rc::clone(&first);
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(
            ItemsRepeater::new()
                .item(
                    "button",
                    Button::new().on_click(move || first_callback.set(first_callback.get() + 1)),
                )
                .into(),
        )
        .unwrap();
        let collection = pump.root().unwrap();
        pump.runtime_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container: RealizedContainer(0),
                index: 0,
            });
        pump.process_realizations().unwrap();
        let child = pump.tree.children(collection).unwrap()[0];
        let revision = pump.event_revision(child, EventId::ButtonClick).unwrap();
        let batches = pump.runtime().batches();
        let second_callback = Rc::clone(&second);

        pump.update(
            ItemsRepeater::new()
                .item(
                    "button",
                    Button::new().on_click(move || second_callback.set(second_callback.get() + 1)),
                )
                .into(),
        )
        .unwrap();

        assert_eq!(pump.runtime().batches(), batches);
        assert_eq!(pump.tree.children(collection).unwrap(), &[child]);
        pump.queue_event(QueuedEvent {
            node: child,
            event: EventId::ButtonClick,
            revision,
            payload: EventPayload::Unit,
        });
        assert_eq!(pump.dispatch_events(), Ok(1));
        assert_eq!(first.get(), 0);
        assert_eq!(second.get(), 1);
    }

    #[test]
    fn same_batch_container_reuse_retires_earlier_row() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(
            ItemsRepeater::new()
                .item("a", TextBlock::new().text("A"))
                .item("b", TextBlock::new().text("B"))
                .into(),
        )
        .unwrap();
        let collection = pump.root().unwrap();
        let container = RealizedContainer(1);
        for index in 0..2 {
            pump.runtime_mut()
                .queue_realization(RealizationRequest::Realize {
                    collection,
                    container,
                    index,
                });
        }

        let outcomes = pump.process_realizations().unwrap();

        let RealizationOutcome::Realized(first) = &outcomes[0] else {
            panic!("expected first lease");
        };
        let RealizationOutcome::Realized(second) = &outcomes[1] else {
            panic!("expected second lease");
        };
        assert!(!pump.tree.virtual_model(collection).unwrap().accepts(first));
        assert!(pump.tree.virtual_model(collection).unwrap().accepts(second));
        assert_eq!(pump.tree.children(collection).unwrap().len(), 1);
        assert_eq!(
            pump.tree.key(pump.tree.children(collection).unwrap()[0]),
            Ok(Some(&Key::from("b")))
        );
        assert_eq!(pump.runtime().node(collection).unwrap().children().len(), 1);
    }
}
