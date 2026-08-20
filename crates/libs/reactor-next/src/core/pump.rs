use std::cmp::Reverse;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

use super::*;

const EVENT_WORK_BUDGET: usize = 64;
const REALIZATION_WORK_BUDGET: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PumpError {
    AlreadyMounted,
    ApplyReceiptMismatch,
    Component(ComponentStoreError),
    NotMounted,
    DuplicateKey(Key),
    EventReadFailed(RuntimeError),
    Poisoned,
    PropertyApplyFailed(CommitReceipt),
    PropertyRetriesExhausted(CommitReceipt),
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

impl From<ComponentStoreError> for PumpError {
    fn from(value: ComponentStoreError) -> Self {
        Self::Component(value)
    }
}

struct PropertyCommit {
    command: usize,
    node: NodeId,
    property: PropertyId,
    value: Option<PropertyValue>,
}

const MAX_PROPERTY_ATTEMPTS: u8 = 3;

struct UpdatePlan {
    commands: Vec<Command>,
    commits: Vec<PropertyCommit>,
    identity: NativeIdentity,
    retry_properties: bool,
}

#[derive(Default)]
struct ComponentChanges {
    composed: HashSet<ComponentToken>,
    reserved: Vec<ComponentToken>,
    retired: Vec<ComponentToken>,
}

enum LocalComponentUpdate {
    Plan(UpdatePlan),
    Fallback(View),
    Unavailable,
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

    fn synchronize_children(&mut self, parent: NodeId, children: Vec<NodeId>) {
        if let Some(index) = self.commands.iter().position(
            |command| matches!(command, Command::SynchronizeChildren { parent: current, .. } if *current == parent),
        ) {
            self.commands.remove(index);
            for commit in &mut self.commits {
                debug_assert_ne!(commit.command, index);
                if commit.command > index {
                    commit.command -= 1;
                }
            }
        }
        self.push(Command::SynchronizeChildren { parent, children });
    }
}

pub struct Pump<R: NativeRuntime> {
    application: Option<NodeId>,
    components: ComponentStore,
    dirty_components: HashSet<ComponentToken>,
    element: Option<Element>,
    tree: Tree,
    runtime: R,
    root: Option<NodeId>,
    events: VecDeque<NativeWork<QueuedEvent>>,
    identity: NativeIdentity,
    poisoned: bool,
    realizations: VecDeque<NativeWork<RealizationRequest>>,
    retry_pending: bool,
    version: u64,
    window: Option<NodeId>,
}

impl<R: NativeRuntime> Pump<R> {
    pub fn new(mut runtime: R) -> Self {
        let identity = NativeIdentity::new(WindowToken::new(WindowId::allocate()));
        runtime.set_identity(identity);
        let mut components = ComponentStore::new(identity.window());
        if let Some(wake) = runtime.component_waker() {
            components.set_waker(wake);
        }
        Self {
            application: None,
            components,
            dirty_components: HashSet::new(),
            element: None,
            tree: Tree::new(),
            runtime,
            root: None,
            events: VecDeque::new(),
            identity,
            poisoned: false,
            realizations: VecDeque::new(),
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
            self.realizations.clear();
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
            self.realizations.clear();
            self.poisoned = true;
            self.root = None;
            self.retry_pending = false;
            self.window = None;
            return Err(PumpError::StructuralApplyFailed(receipt));
        }
        let retries_exhausted =
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
            return Err(if retries_exhausted {
                PumpError::PropertyRetriesExhausted(receipt)
            } else {
                PumpError::PropertyApplyFailed(receipt)
            });
        }
        self.retry_pending = false;
        self.version = next_version;
        Ok(receipt)
    }

    pub fn mount_view(&mut self, view: View) -> Result<CommitReceipt, PumpError> {
        if self.poisoned {
            return Err(PumpError::Poisoned);
        }
        if self.root.is_some() {
            return Err(PumpError::AlreadyMounted);
        }
        let next_version = self.next_version()?;
        let mut candidate = Tree::new();
        let mut plan = UpdatePlan::new(self.identity);
        let mut changes = ComponentChanges::default();
        let application = candidate.insert(None, NodeKind::Application)?;
        plan.push(Command::CreateApplication { node: application });
        let window = candidate.insert(Some(application), NodeKind::Window)?;
        plan.push(Command::CreateWindow { node: window });
        let mounted = Self::mount_planned_view(
            &mut candidate,
            Some(window),
            None,
            view,
            &mut self.components,
            &mut changes,
            &mut plan,
        );
        let (root, native_roots) = match mounted {
            Ok(mounted) => mounted,
            Err(error) => {
                Self::remove_reservations(&mut self.components, &changes.reserved);
                return Err(error);
            }
        };
        match native_roots.as_slice() {
            [] => {}
            [native_root] => {
                plan.push(Command::InsertChild {
                    parent: window,
                    child: *native_root,
                    index: 0,
                });
            }
            _ => {
                Self::remove_reservations(&mut self.components, &changes.reserved);
                return Err(PumpError::StructureUnsupported);
            }
        }
        plan.push(Command::ActivateWindow { node: window });

        let receipt = self.runtime.apply(&plan.commands);
        if receipt.outcomes.len() != plan.commands.len()
            || plan
                .commands
                .iter()
                .enumerate()
                .any(|(index, command)| command.structural() && !receipt.applied(index))
        {
            self.runtime.reset();
            Self::remove_reservations(&mut self.components, &changes.reserved);
            self.poisoned = true;
            return Err(if receipt.outcomes.len() != plan.commands.len() {
                PumpError::ApplyReceiptMismatch
            } else {
                PumpError::StructuralApplyFailed(receipt)
            });
        }

        let retries_exhausted =
            match Self::commit_tree_properties(&mut candidate, &plan.commits, &receipt) {
                Ok(retries_exhausted) => retries_exhausted,
                Err(error) => {
                    self.runtime.reset();
                    Self::remove_reservations(&mut self.components, &changes.reserved);
                    self.poisoned = true;
                    return Err(error);
                }
            };
        for token in changes.reserved.iter().copied() {
            self.components.publish(token)?;
        }
        self.tree = candidate;
        self.application = Some(application);
        self.root = Some(root);
        self.window = Some(window);
        for token in changes.reserved {
            self.components.commit_effects(token)?;
        }
        if plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, command)| !command.structural() && !receipt.applied(index))
        {
            self.retry_pending = true;
            return Err(if retries_exhausted {
                PumpError::PropertyRetriesExhausted(receipt)
            } else {
                PumpError::PropertyApplyFailed(receipt)
            });
        }
        self.retry_pending = false;
        self.version = next_version;
        Ok(receipt)
    }

    pub fn update_view(&mut self, view: View) -> Result<CommitReceipt, PumpError> {
        if self.poisoned {
            return Err(PumpError::Poisoned);
        }
        let next_version = self.next_version()?;
        let root = self.root.ok_or(PumpError::NotMounted)?;
        let mut candidate = self.tree.clone();
        let mut plan = UpdatePlan {
            retry_properties: self.retry_pending,
            ..UpdatePlan::new(self.identity)
        };
        let mut changes = ComponentChanges::default();
        if let Err(error) = Self::reconcile_planned_view(
            &mut candidate,
            root,
            view,
            &mut self.components,
            &mut changes,
            &mut plan,
        ) {
            Self::remove_reservations(&mut self.components, &changes.reserved);
            return Err(error);
        }
        let window = self.window.ok_or(PumpError::NotMounted)?;
        let [candidate_root] = candidate.children(window)? else {
            Self::remove_reservations(&mut self.components, &changes.reserved);
            return Err(PumpError::StructureUnsupported);
        };
        let candidate_root = *candidate_root;
        self.apply_component_candidate(candidate, candidate_root, plan, changes, next_version)
    }

    pub fn dispatch_components(&mut self, budget: usize) -> Result<usize, PumpError> {
        if self.poisoned {
            return Err(PumpError::Poisoned);
        }
        let report = self.components.drain(budget)?;
        for token in report.dirty {
            self.dirty_components.insert(token);
        }
        if self.dirty_components.is_empty() && self.retry_pending {
            let root = self.root.ok_or(PumpError::NotMounted)?;
            for node in self.tree.subtree_postorder(root)? {
                if self.tree.kind(node)? == NodeKind::Component {
                    self.dirty_components
                        .insert(self.components.token(self.tree.component_scope(node)?)?);
                }
            }
        }
        if self.dirty_components.is_empty() {
            return Ok(report.dispatched);
        }

        let next_version = self.next_version()?;
        let mut composed_view = None;
        if self.dirty_components.len() == 1 {
            let Some(token) = self.dirty_components.iter().next().copied() else {
                return Ok(report.dispatched);
            };
            match self.try_local_component_update(token)? {
                LocalComponentUpdate::Plan(plan) => {
                    self.apply_local_component_plan(token, plan, next_version)?;
                    self.dirty_components.clear();
                    return Ok(report.dispatched);
                }
                LocalComponentUpdate::Fallback(view) => composed_view = Some((token, view)),
                LocalComponentUpdate::Unavailable => {}
            }
        }

        let mut candidate = self.tree.clone();
        let mut plan = UpdatePlan {
            retry_properties: self.retry_pending,
            ..UpdatePlan::new(self.identity)
        };
        let mut changes = ComponentChanges::default();
        let mut dirty = self
            .dirty_components
            .iter()
            .copied()
            .map(|token| {
                let depth = if let Some(node) = candidate.component_node(token.scope())? {
                    candidate.depth(node)?
                } else {
                    usize::MAX
                };
                Ok((depth, token))
            })
            .collect::<Result<Vec<_>, PumpError>>()?;
        dirty.sort_unstable_by_key(|(depth, _)| *depth);
        for (_, token) in dirty {
            if changes.composed.contains(&token) {
                continue;
            }
            let Some(node) = candidate.component_node(token.scope())? else {
                if changes.retired.contains(&token) {
                    continue;
                }
                Self::remove_reservations(&mut self.components, &changes.reserved);
                return Err(PumpError::StructureUnsupported);
            };
            let result = if composed_view
                .as_ref()
                .is_some_and(|(cached, _)| *cached == token)
            {
                let (_, view) = composed_view.take().unwrap();
                changes.composed.insert(token);
                Self::recompose_component_view(
                    &mut candidate,
                    node,
                    view,
                    &mut self.components,
                    &mut changes,
                    &mut plan,
                )
            } else {
                Self::recompose_component(
                    &mut candidate,
                    node,
                    token,
                    &mut self.components,
                    &mut changes,
                    &mut plan,
                )
            };
            if let Err(error) = result {
                Self::remove_reservations(&mut self.components, &changes.reserved);
                return Err(error);
            }
        }
        let root = self.root.ok_or(PumpError::NotMounted)?;
        match self.apply_component_candidate(candidate, root, plan, changes, next_version) {
            Ok(_) => {
                self.dirty_components.clear();
                Ok(report.dispatched)
            }
            Err(error @ PumpError::RecoveredStructure(_)) => {
                self.dirty_components.clear();
                Err(error)
            }
            Err(error) => Err(error),
        }
    }

    fn try_local_component_update(
        &mut self,
        token: ComponentToken,
    ) -> Result<LocalComponentUpdate, PumpError> {
        let Some(node) = self.tree.component_node(token.scope())? else {
            return Ok(LocalComponentUpdate::Unavailable);
        };
        let [slot] = self.tree.children(node)? else {
            return Ok(LocalComponentUpdate::Unavailable);
        };
        let native = match self.tree.children(*slot)? {
            [native] => *native,
            _ => return Ok(LocalComponentUpdate::Unavailable),
        };
        if !matches!(self.tree.kind(native)?, NodeKind::Native(_))
            || !self.tree.children(native)?.is_empty()
        {
            return Ok(LocalComponentUpdate::Unavailable);
        }
        let view = self.components.view(token)?;
        let View::Native(element) = view else {
            return Ok(LocalComponentUpdate::Fallback(view));
        };
        if self.tree.kind(native)? != NodeKind::Native(element.kind())
            || !self.tree.children(native)?.is_empty()
            || !matches!(element.structure(), ElementStructureRef::None)
        {
            return Ok(LocalComponentUpdate::Fallback(View::Native(element)));
        }
        let mut event_activity_matches = true;
        element.visit_events(&mut |event, active| {
            event_activity_matches &= self
                .tree
                .native(native)
                .ok()
                .and_then(|state| state.events.get(&event))
                .is_some_and(|state| state.active == active);
        });
        if !event_activity_matches {
            return Ok(LocalComponentUpdate::Fallback(View::Native(element)));
        }
        let mut plan = UpdatePlan {
            retry_properties: self.retry_pending,
            ..UpdatePlan::new(self.identity)
        };
        Self::reconcile_shallow_control(&mut self.tree, native, element, &mut plan)?;
        debug_assert!(plan.commands.iter().all(|command| !command.structural()));
        Ok(LocalComponentUpdate::Plan(plan))
    }

    fn apply_local_component_plan(
        &mut self,
        token: ComponentToken,
        plan: UpdatePlan,
        next_version: u64,
    ) -> Result<CommitReceipt, PumpError> {
        self.components.prepare_effects(token)?;
        if plan.commands.is_empty() {
            self.components.commit_effects(token)?;
            self.retry_pending = false;
            self.version = next_version;
            return Ok(CommitReceipt {
                outcomes: Vec::new(),
            });
        }
        if plan.commands.iter().any(Command::structural) {
            return Err(PumpError::StructureUnsupported);
        }
        let receipt = self.runtime.apply(&plan.commands);
        if receipt.outcomes.len() != plan.commands.len() {
            self.poisoned = true;
            return Err(PumpError::ApplyReceiptMismatch);
        }
        let retries_exhausted =
            Self::commit_tree_properties(&mut self.tree, &plan.commits, &receipt)?;
        self.components.commit_effects(token)?;
        if plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, _)| !receipt.applied(index))
        {
            self.retry_pending = true;
            return Err(if retries_exhausted {
                PumpError::PropertyRetriesExhausted(receipt)
            } else {
                PumpError::PropertyApplyFailed(receipt)
            });
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
            self.realizations.clear();
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

        let retries_exhausted =
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
            return Err(if retries_exhausted {
                PumpError::PropertyRetriesExhausted(receipt)
            } else {
                PumpError::PropertyApplyFailed(receipt)
            });
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
        self.realizations.clear();
        self.identity = recovery_identity;
        self.runtime.set_identity(recovery_identity);
        self.refresh_component_waker();
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
        if let Some(root) = self.root {
            for node in self.tree.subtree_postorder(root).unwrap() {
                if self.tree.kind(node).unwrap() == NodeKind::Component {
                    let token = self
                        .components
                        .token(self.tree.component_scope(node).unwrap())
                        .unwrap();
                    self.components.cleanup_effects(token).unwrap();
                }
            }
        }
        self.runtime.reset();
        self.application = None;
        self.element = None;
        self.dirty_components.clear();
        self.events.clear();
        self.realizations.clear();
        self.retry_pending = false;
        self.root = None;
        self.tree = Tree::new();
        self.version = 0;
        self.window = None;
        if let Some(identity) = identity {
            self.identity = identity;
            let mut components = ComponentStore::new(identity.window());
            if let Some(wake) = self.runtime.component_waker() {
                components.set_waker(wake);
            }
            self.components = components;
            self.runtime.set_identity(identity);
            self.poisoned = false;
        } else {
            self.poisoned = true;
        }
    }

    pub fn root(&self) -> Option<NodeId> {
        self.root
    }

    #[cfg(feature = "test")]
    pub(crate) fn root_native(&self) -> Option<NodeId> {
        Self::native_root(&self.tree, self.root?).ok()
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

    pub(crate) fn components(&self) -> &ComponentStore {
        &self.components
    }

    pub(crate) fn components_mut(&mut self) -> &mut ComponentStore {
        &mut self.components
    }

    pub fn native_work_pending(&self) -> bool {
        !self.events.is_empty()
            || !self.realizations.is_empty()
            || self.components.pending() != 0
            || !self.dirty_components.is_empty()
            || self.retry_pending
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
            self.realizations.clear();
            _ = self.runtime.drain_event_errors();
            _ = self.runtime.drain_realizations();
            return Ok(0);
        }
        self.process_realizations()?;
        for queued in self.runtime.drain_event_errors() {
            if queued.identity != self.identity {
                continue;
            }
            let error = queued.work;
            let Ok(native) = self.tree.native(error.node) else {
                continue;
            };
            let Some(state) = native.events.get(&error.event) else {
                continue;
            };
            if !state.active || state.revision != error.revision {
                continue;
            }
            self.events.clear();
            return Err(PumpError::EventReadFailed(error.error));
        }
        let mut dispatched = 0;
        for _ in 0..EVENT_WORK_BUDGET {
            let Some(queued) = self.events.pop_front() else {
                break;
            };
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
            if !state.active || state.revision != event.revision {
                continue;
            }
            let observation = native.desired.observe_event(event.event, &event.payload);
            if let Some((property, value)) = observation {
                self.tree
                    .native_mut(event.node)?
                    .properties
                    .insert(property, NativePropertyState::Known(Some(value)));
                self.retry_pending = true;
            }
            if self
                .tree
                .native(event.node)?
                .desired
                .dispatch_event(event.event, &event.payload)
            {
                dispatched += 1;
            }
        }
        Ok(dispatched)
    }

    pub fn process_realizations(&mut self) -> Result<Vec<RealizationOutcome>, PumpError> {
        if self.poisoned {
            self.realizations.clear();
            _ = self.runtime.drain_realizations();
            return Err(PumpError::Poisoned);
        }
        self.realizations.extend(self.runtime.drain_realizations());
        let mut outcomes = Vec::with_capacity(self.realizations.len().min(REALIZATION_WORK_BUDGET));
        let mut candidate = self.tree.clone();
        let mut plan = UpdatePlan::new(self.identity);
        for _ in 0..REALIZATION_WORK_BUDGET {
            let Some(queued) = self.realizations.pop_front() else {
                break;
            };
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

    fn element_structure_is_empty(element: &Element) -> bool {
        match element.structure() {
            ElementStructureRef::None | ElementStructureRef::Content(None) => true,
            ElementStructureRef::Children(children) => children.is_empty(),
            ElementStructureRef::Virtual(_) => false,
            ElementStructureRef::Content(Some(_)) => false,
        }
    }

    fn control_has_role(kind: MountedKind, role: ControlRole) -> bool {
        CONTROLS
            .iter()
            .find(|control| control.kind == kind)
            .is_some_and(|control| control.role == role)
    }

    fn native_root(tree: &Tree, node: NodeId) -> Result<NodeId, PumpError> {
        let roots = Self::native_roots(tree, node)?;
        let [root] = roots.as_slice() else {
            return Err(PumpError::StructureUnsupported);
        };
        Ok(*root)
    }

    fn native_roots(tree: &Tree, node: NodeId) -> Result<Vec<NodeId>, PumpError> {
        match tree.kind(node)? {
            NodeKind::Native(_) | NodeKind::VirtualCollection => Ok(vec![node]),
            NodeKind::Component | NodeKind::Fragment | NodeKind::Slot => {
                let mut roots = Vec::new();
                for child in tree.children(node)?.iter().copied() {
                    roots.extend(Self::native_roots(tree, child)?);
                }
                Ok(roots)
            }
            NodeKind::Application | NodeKind::Window => Err(PumpError::StructureUnsupported),
        }
    }

    fn commit_tree_properties(
        tree: &mut Tree,
        commits: &[PropertyCommit],
        receipt: &CommitReceipt,
    ) -> Result<bool, PumpError> {
        let mut retries_exhausted = false;
        for commit in commits {
            let state = if receipt.applied(commit.command) {
                NativePropertyState::Known(commit.value.clone())
            } else {
                let attempts = match tree.native(commit.node)?.properties.get(&commit.property) {
                    Some(NativePropertyState::Divergent { attempts }) => attempts.saturating_add(1),
                    _ => 1,
                };
                retries_exhausted |= attempts >= MAX_PROPERTY_ATTEMPTS;
                NativePropertyState::Divergent { attempts }
            };
            tree.native_mut(commit.node)?
                .properties
                .insert(commit.property, state);
        }
        Ok(retries_exhausted)
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
            let properties = &tree.native(node)?.properties;
            parts.props.visit_properties(&mut |property, value| {
                let changed = properties.get(&property).map_or_else(
                    || value.is_some(),
                    |native| native != &NativePropertyState::Known(value.clone()),
                );
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

                if operations.len() >= 256 && operations.len() * 4 > new_keys.len() {
                    let old_key_set = old_keys.iter().cloned().collect::<HashSet<_>>();
                    let new_key_set = new_keys.iter().cloned().collect::<HashSet<_>>();
                    if old_key_set == new_key_set {
                        let order = new_keys
                            .iter()
                            .map(|key| {
                                nodes
                                    .get(key)
                                    .copied()
                                    .ok_or(PumpError::StructureUnsupported)
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        plan.push(Command::ResetChildren { parent: node });
                        for (index, key) in new_keys.iter().enumerate() {
                            let child = nodes
                                .get(key)
                                .copied()
                                .ok_or(PumpError::StructureUnsupported)?;
                            plan.push(Command::InsertChild {
                                parent: node,
                                child,
                                index,
                            });
                        }
                        tree.set_children(node, order)?;
                        return Ok(node);
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

    fn refresh_component_waker(&mut self) {
        if let Some(wake) = self.runtime.component_waker() {
            self.components.set_waker(wake);
        }
    }

    fn apply_component_candidate(
        &mut self,
        mut candidate: Tree,
        candidate_root: NodeId,
        plan: UpdatePlan,
        changes: ComponentChanges,
        next_version: u64,
    ) -> Result<CommitReceipt, PumpError> {
        self.prepare_component_effects(&changes)?;
        if plan.commands.is_empty() {
            self.finalize_component_changes(&changes)?;
            self.tree = candidate;
            self.root = Some(candidate_root);
            self.commit_component_effects(&changes)?;
            self.retry_pending = false;
            self.version = next_version;
            return Ok(CommitReceipt {
                outcomes: Vec::new(),
            });
        }

        let receipt = self.runtime.apply(&plan.commands);
        if receipt.outcomes.len() != plan.commands.len() {
            Self::remove_reservations(&mut self.components, &changes.reserved);
            self.poisoned = true;
            return Err(PumpError::ApplyReceiptMismatch);
        }
        if plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, command)| command.structural() && !receipt.applied(index))
        {
            return self.recover_component_structure(
                candidate,
                candidate_root,
                receipt,
                changes,
                next_version,
            );
        }
        let retries_exhausted =
            match Self::commit_tree_properties(&mut candidate, &plan.commits, &receipt) {
                Ok(retries_exhausted) => retries_exhausted,
                Err(error) => {
                    self.runtime.reset();
                    Self::remove_reservations(&mut self.components, &changes.reserved);
                    self.poisoned = true;
                    return Err(error);
                }
            };
        self.finalize_component_changes(&changes)?;
        self.tree = candidate;
        self.root = Some(candidate_root);
        self.commit_component_effects(&changes)?;
        if plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, _)| !receipt.applied(index))
        {
            self.retry_pending = true;
            return Err(if retries_exhausted {
                PumpError::PropertyRetriesExhausted(receipt)
            } else {
                PumpError::PropertyApplyFailed(receipt)
            });
        }
        self.retry_pending = false;
        self.version = next_version;
        Ok(receipt)
    }

    fn prepare_component_effects(&self, changes: &ComponentChanges) -> Result<(), PumpError> {
        for token in changes.retired.iter().copied() {
            self.components.cleanup_effects(token)?;
        }
        let retired = changes.retired.iter().copied().collect::<HashSet<_>>();
        let mut composed = changes
            .composed
            .iter()
            .copied()
            .filter(|token| !retired.contains(token))
            .map(|token| {
                let node = self
                    .tree
                    .component_node(token.scope())?
                    .ok_or(PumpError::StructureUnsupported)?;
                Ok((self.tree.depth(node)?, token))
            })
            .collect::<Result<Vec<_>, PumpError>>()?;
        composed.sort_unstable_by_key(|(depth, _)| Reverse(*depth));
        for (_, token) in composed {
            self.components.prepare_effects(token)?;
        }
        Ok(())
    }

    fn commit_component_effects(&self, changes: &ComponentChanges) -> Result<(), PumpError> {
        let retired = changes.retired.iter().copied().collect::<HashSet<_>>();
        let mut tokens = changes
            .reserved
            .iter()
            .chain(changes.composed.iter())
            .copied()
            .filter(|token| !retired.contains(token))
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|token| {
                let node = self
                    .tree
                    .component_node(token.scope())?
                    .ok_or(PumpError::StructureUnsupported)?;
                Ok((self.tree.depth(node)?, token))
            })
            .collect::<Result<Vec<_>, PumpError>>()?;
        tokens.sort_unstable_by_key(|(depth, _)| *depth);
        for (_, token) in tokens {
            self.components.commit_effects(token)?;
        }
        Ok(())
    }

    fn finalize_component_changes(&mut self, changes: &ComponentChanges) -> Result<(), PumpError> {
        for token in changes.reserved.iter().copied() {
            if let Err(error) = self.components.publish(token) {
                self.poisoned = true;
                return Err(error.into());
            }
        }
        for token in changes.retired.iter().copied() {
            if let Err(error) = self
                .components
                .retire(token)
                .and_then(|()| self.components.remove(token))
            {
                self.poisoned = true;
                return Err(error.into());
            }
        }
        Ok(())
    }

    fn recover_component_structure(
        &mut self,
        mut candidate: Tree,
        candidate_root: NodeId,
        failure: CommitReceipt,
        changes: ComponentChanges,
        next_version: u64,
    ) -> Result<CommitReceipt, PumpError> {
        let window = self.window.ok_or(PumpError::NotMounted)?;
        let recovery_identity = self
            .identity
            .next_realization()
            .ok_or(PumpError::RevisionExhausted)?;
        let mut plan = UpdatePlan::new(recovery_identity);
        plan.push(Command::ResetWindowContent { window });
        let native_roots = match Self::plan_existing_subtree(&candidate, candidate_root, &mut plan)
        {
            Ok(native_roots) => native_roots,
            Err(error) => {
                self.runtime.reset();
                Self::remove_reservations(&mut self.components, &changes.reserved);
                self.poisoned = true;
                return Err(error);
            }
        };
        match native_roots.as_slice() {
            [] => {}
            [native_root] => {
                plan.push(Command::InsertChild {
                    parent: window,
                    child: *native_root,
                    index: 0,
                });
            }
            _ => {
                Self::remove_reservations(&mut self.components, &changes.reserved);
                return Err(PumpError::StructureUnsupported);
            }
        }

        self.events.clear();
        self.realizations.clear();
        self.identity = recovery_identity;
        self.runtime.set_identity(recovery_identity);
        self.refresh_component_waker();
        let recovery = self.runtime.apply(&plan.commands);
        let attempt = |recovery| {
            Box::new(StructuralRecovery {
                failure: failure.clone(),
                recovery,
                root: candidate_root,
            })
        };
        if recovery.outcomes.len() != plan.commands.len()
            || plan
                .commands
                .iter()
                .enumerate()
                .any(|(index, command)| command.structural() && !recovery.applied(index))
        {
            Self::remove_reservations(&mut self.components, &changes.reserved);
            self.poisoned = true;
            self.retry_pending = false;
            return Err(PumpError::RecoveryFailed(attempt(recovery)));
        }

        Self::commit_tree_properties(&mut candidate, &plan.commits, &recovery)?;
        self.finalize_component_changes(&changes)?;
        self.tree = candidate;
        self.root = Some(candidate_root);
        self.commit_component_effects(&changes)?;
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

    fn plan_existing_subtree(
        tree: &Tree,
        node: NodeId,
        plan: &mut UpdatePlan,
    ) -> Result<Vec<NodeId>, PumpError> {
        match tree.kind(node)? {
            NodeKind::Component | NodeKind::Fragment | NodeKind::Slot => {
                let mut roots = Vec::new();
                for child in tree.children(node)?.iter().copied() {
                    roots.extend(Self::plan_existing_subtree(tree, child, plan)?);
                }
                Ok(roots)
            }
            NodeKind::Native(kind) => {
                let desired = tree.native(node)?.desired.clone();
                let events = tree.native(node)?.events.clone();
                plan.push(Command::Create { node, kind });
                desired.visit_properties(&mut |property, value| {
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
                for (event, state) in events {
                    if state.active {
                        plan.push(Command::SubscribeEvent {
                            node,
                            event,
                            revision: state.revision,
                        });
                    }
                }
                let mut index = 0;
                for child in tree.children(node)?.iter().copied() {
                    for child in Self::plan_existing_subtree(tree, child, plan)? {
                        plan.push(Command::InsertChild {
                            parent: node,
                            child,
                            index,
                        });
                        index += 1;
                    }
                }
                Ok(vec![node])
            }
            NodeKind::VirtualCollection => Err(PumpError::StructureUnsupported),
            NodeKind::Application | NodeKind::Window => Err(PumpError::StructureUnsupported),
        }
    }

    fn reconcile_planned_view(
        tree: &mut Tree,
        node: NodeId,
        view: View,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        match view {
            View::Native(element) => {
                if tree.kind(node)? != NodeKind::Native(element.kind())
                    || !tree.children(node)?.is_empty()
                    || !matches!(element.structure(), ElementStructureRef::None)
                {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::Native(element),
                        components,
                        changes,
                        plan,
                    );
                }
                Self::reconcile_node(tree, node, element, plan)?;
                Ok(())
            }
            View::Component(component) => {
                if tree.kind(node)? != NodeKind::Component
                    || tree.component_type(node)? != component.component_type()
                {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::Component(component),
                        components,
                        changes,
                        plan,
                    );
                }
                let token = components.token(tree.component_scope(node)?)?;
                if component.apply_props(components, token)? {
                    Self::recompose_component(tree, node, token, components, changes, plan)
                } else {
                    Ok(())
                }
            }
            View::Empty => {
                if tree.kind(node)? != NodeKind::Fragment {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::Empty,
                        components,
                        changes,
                        plan,
                    );
                }
                Self::reconcile_fragment(tree, node, &[], components, changes, plan)
            }
            View::Fragment(children) => {
                if tree.kind(node)? != NodeKind::Fragment {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::Fragment(children),
                        components,
                        changes,
                        plan,
                    );
                }
                Self::reconcile_fragment(tree, node, &children, components, changes, plan)
            }
            View::Content { control, content } => {
                if !Self::control_has_role(control.kind(), ControlRole::Content) {
                    return Err(PumpError::StructureUnsupported);
                }
                if tree.kind(node)? != NodeKind::Native(control.kind()) {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::Content { control, content },
                        components,
                        changes,
                        plan,
                    );
                }
                Self::reconcile_shallow_control(tree, node, control, plan)?;
                let [child] = tree.children(node)? else {
                    return Err(PumpError::StructureUnsupported);
                };
                Self::reconcile_planned_view(tree, *child, *content, components, changes, plan)
            }
            View::Children { control, children } => {
                if !Self::control_has_role(control.kind(), ControlRole::Children) {
                    return Err(PumpError::StructureUnsupported);
                }
                if tree.kind(node)? != NodeKind::Native(control.kind()) {
                    return Self::replace_planned_view(
                        tree,
                        node,
                        View::Children { control, children },
                        components,
                        changes,
                        plan,
                    );
                }
                Self::reconcile_shallow_control(tree, node, control, plan)?;
                let old_native = Self::native_children(tree, node)?;
                let current = tree.children(node)?.to_vec();
                let mut requires_sync = current.iter().any(|child| {
                    Self::native_roots(tree, *child).map_or(true, |roots| roots.len() != 1)
                });
                let old_keys = current
                    .iter()
                    .map(|child| {
                        tree.key(*child)?
                            .cloned()
                            .ok_or(PumpError::StructureUnsupported)
                    })
                    .collect::<Result<Vec<_>, PumpError>>()?;
                let new_keys = children
                    .iter()
                    .map(|child| child.key().clone())
                    .collect::<Vec<_>>();
                diff(&old_keys, &new_keys)
                    .map_err(|KeyedError::DuplicateKey(key)| PumpError::DuplicateKey(key))?;

                for (key, child) in old_keys.iter().zip(current.iter().copied()) {
                    if !new_keys.contains(key) {
                        Self::collect_retired_components(tree, child, components, changes)?;
                        Self::retire_planned_subtree(tree, child, plan)?;
                    }
                }
                for (index, child) in children.iter().enumerate() {
                    let existing =
                        tree.children(node)?.iter().copied().find(|candidate| {
                            tree.key(*candidate).ok().flatten() == Some(child.key())
                        });
                    let (child_node, inserted) = if let Some(child_node) = existing {
                        let old_roots = Self::native_roots(tree, child_node)?;
                        Self::reconcile_planned_view(
                            tree,
                            child_node,
                            child.view().clone(),
                            components,
                            changes,
                            plan,
                        )?;
                        let child_node = tree
                            .children(node)?
                            .iter()
                            .copied()
                            .find(|candidate| {
                                tree.key(*candidate).ok().flatten() == Some(child.key())
                            })
                            .ok_or(PumpError::StructureUnsupported)?;
                        requires_sync |= old_roots.len() != 1
                            || Self::native_roots(tree, child_node)?.len() != 1;
                        (child_node, false)
                    } else {
                        let (child_node, native) = Self::mount_planned_view(
                            tree,
                            Some(node),
                            Some(child.key().clone()),
                            child.view().clone(),
                            components,
                            changes,
                            plan,
                        )?;
                        if let [native] = native.as_slice()
                            && !requires_sync
                        {
                            plan.push(Command::InsertChild {
                                parent: node,
                                child: *native,
                                index,
                            });
                        } else {
                            requires_sync = true;
                        }
                        (child_node, true)
                    };

                    let mut order = tree.children(node)?.to_vec();
                    let previous = order
                        .iter()
                        .position(|node| *node == child_node)
                        .ok_or(PumpError::StructureUnsupported)?;
                    if previous != index {
                        order.remove(previous);
                        order.insert(index, child_node);
                        if !inserted && !requires_sync {
                            plan.push(Command::MoveChild {
                                parent: node,
                                child: Self::native_root(tree, child_node)?,
                                index,
                            });
                        }
                        tree.set_children(node, order)?;
                    }
                }
                let new_native = Self::native_children(tree, node)?;
                if requires_sync && old_native != new_native {
                    plan.synchronize_children(node, new_native);
                }
                Ok(())
            }
            View::VirtualItems { .. } => Err(PumpError::StructureUnsupported),
        }
    }

    fn reconcile_fragment(
        tree: &mut Tree,
        node: NodeId,
        children: &[KeyedView],
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let old_native = Self::native_roots(tree, node)?;
        let current = tree.children(node)?.to_vec();
        let old_keys = current
            .iter()
            .map(|child| {
                tree.key(*child)?
                    .cloned()
                    .ok_or(PumpError::StructureUnsupported)
            })
            .collect::<Result<Vec<_>, PumpError>>()?;
        let new_keys = children
            .iter()
            .map(|child| child.key().clone())
            .collect::<Vec<_>>();
        diff(&old_keys, &new_keys)
            .map_err(|KeyedError::DuplicateKey(key)| PumpError::DuplicateKey(key))?;

        for (key, child) in old_keys.iter().zip(current) {
            if !new_keys.contains(key) {
                Self::collect_retired_components(tree, child, components, changes)?;
                Self::retire_planned_subtree(tree, child, plan)?;
            }
        }
        let mut order = Vec::with_capacity(children.len());
        for child in children {
            let existing = tree
                .children(node)?
                .iter()
                .copied()
                .find(|candidate| tree.key(*candidate).ok().flatten() == Some(child.key()));
            let child_node = if let Some(child_node) = existing {
                Self::reconcile_planned_view(
                    tree,
                    child_node,
                    child.view().clone(),
                    components,
                    changes,
                    plan,
                )?;
                tree.children(node)?
                    .iter()
                    .copied()
                    .find(|candidate| tree.key(*candidate).ok().flatten() == Some(child.key()))
                    .ok_or(PumpError::StructureUnsupported)?
            } else {
                Self::mount_planned_view(
                    tree,
                    Some(node),
                    Some(child.key().clone()),
                    child.view().clone(),
                    components,
                    changes,
                    plan,
                )?
                .0
            };
            order.push(child_node);
        }
        tree.set_children(node, order)?;

        let new_native = Self::native_roots(tree, node)?;
        if old_native != new_native {
            let (native_parent, _) = Self::native_location(tree, node)?;
            let native = Self::native_children(tree, native_parent)?;
            Self::validate_native_arity(tree, native_parent, &native)?;
            plan.synchronize_children(native_parent, native);
        }
        Ok(())
    }

    fn replace_planned_view(
        tree: &mut Tree,
        node: NodeId,
        view: View,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let parent = tree.parent(node)?.ok_or(PumpError::StructureUnsupported)?;
        let key = tree.key(node)?.cloned();
        let index = tree
            .children(parent)?
            .iter()
            .position(|child| *child == node)
            .ok_or(PumpError::StructureUnsupported)?;
        let (native_parent, native_index) = Self::native_location(tree, node)?;
        if tree.kind(native_parent)? == NodeKind::VirtualCollection {
            return Err(PumpError::StructureUnsupported);
        }

        Self::collect_retired_components(tree, node, components, changes)?;
        Self::retire_planned_subtree(tree, node, plan)?;
        let (replacement, native) =
            Self::mount_planned_view(tree, Some(parent), key, view, components, changes, plan)?;
        let mut children = tree.children(parent)?.to_vec();
        let appended = children
            .iter()
            .position(|child| *child == replacement)
            .ok_or(PumpError::StructureUnsupported)?;
        children.remove(appended);
        children.insert(index, replacement);
        tree.set_children(parent, children)?;
        let native_children = Self::native_children(tree, native_parent)?;
        Self::validate_native_arity(tree, native_parent, &native_children)?;
        for (index, child) in native.into_iter().enumerate() {
            plan.push(Command::InsertChild {
                parent: native_parent,
                child,
                index: native_index + index,
            });
        }
        Ok(())
    }

    fn collect_retired_components(
        tree: &Tree,
        root: NodeId,
        components: &ComponentStore,
        changes: &mut ComponentChanges,
    ) -> Result<(), PumpError> {
        for node in tree.subtree_postorder(root)? {
            if tree.kind(node)? == NodeKind::Component {
                let token = components.token(tree.component_scope(node)?)?;
                if !changes.retired.contains(&token) {
                    changes.retired.push(token);
                }
            }
        }
        Ok(())
    }

    fn native_container(tree: &Tree, node: NodeId) -> Result<NodeId, PumpError> {
        let mut current = node;
        loop {
            match tree.kind(current)? {
                NodeKind::Native(_)
                | NodeKind::VirtualCollection
                | NodeKind::Window
                | NodeKind::Application => return Ok(current),
                NodeKind::Component | NodeKind::Fragment | NodeKind::Slot => {
                    current = tree
                        .parent(current)?
                        .ok_or(PumpError::StructureUnsupported)?;
                }
            }
        }
    }

    fn native_location(tree: &Tree, node: NodeId) -> Result<(NodeId, usize), PumpError> {
        let mut current = node;
        let mut offset = 0;
        loop {
            let parent = tree
                .parent(current)?
                .ok_or(PumpError::StructureUnsupported)?;
            for sibling in tree.children(parent)? {
                if *sibling == current {
                    break;
                }
                offset += Self::native_roots(tree, *sibling)?.len();
            }
            match tree.kind(parent)? {
                NodeKind::Native(_)
                | NodeKind::VirtualCollection
                | NodeKind::Window
                | NodeKind::Application => return Ok((parent, offset)),
                NodeKind::Component | NodeKind::Fragment | NodeKind::Slot => current = parent,
            }
        }
    }

    fn native_children(tree: &Tree, parent: NodeId) -> Result<Vec<NodeId>, PumpError> {
        let mut native = Vec::new();
        for child in tree.children(parent)?.iter().copied() {
            native.extend(Self::native_roots(tree, child)?);
        }
        Ok(native)
    }

    fn validate_native_arity(
        tree: &Tree,
        parent: NodeId,
        native: &[NodeId],
    ) -> Result<(), PumpError> {
        let allows_many = match tree.kind(parent)? {
            NodeKind::Native(kind) => Self::control_has_role(kind, ControlRole::Children),
            NodeKind::Window => false,
            _ => return Err(PumpError::StructureUnsupported),
        };
        if allows_many || native.len() <= 1 {
            Ok(())
        } else {
            Err(PumpError::StructureUnsupported)
        }
    }

    fn reconcile_shallow_control(
        tree: &mut Tree,
        node: NodeId,
        control: Element,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        if !Self::element_structure_is_empty(&control) {
            return Err(PumpError::StructureUnsupported);
        }
        let parts = control.into_parts();
        Self::reconcile_shallow_parts(tree, node, parts, plan)
    }

    fn reconcile_shallow_parts(
        tree: &mut Tree,
        node: NodeId,
        parts: ElementParts,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        if tree.kind(node)? != NodeKind::Native(parts.kind) {
            return Err(PumpError::StructureUnsupported);
        }

        let props_changed = tree.native(node)?.desired != parts.props;
        if props_changed || plan.retry_properties {
            let properties = &tree.native(node)?.properties;
            parts.props.visit_properties(&mut |property, value| {
                let changed = properties.get(&property).map_or_else(
                    || value.is_some(),
                    |native| native != &NativePropertyState::Known(value.clone()),
                );
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
        Ok(())
    }

    fn recompose_component(
        tree: &mut Tree,
        node: NodeId,
        token: ComponentToken,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        if !changes.composed.insert(token) {
            return Ok(());
        }
        let view = components.view(token)?;
        Self::recompose_component_view(tree, node, view, components, changes, plan)
    }

    fn recompose_component_view(
        tree: &mut Tree,
        node: NodeId,
        view: View,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let [slot] = tree.children(node)? else {
            return Err(PumpError::StructureUnsupported);
        };
        if tree.kind(*slot)? != NodeKind::Slot {
            return Err(PumpError::StructureUnsupported);
        }
        let [child] = tree.children(*slot)? else {
            return Err(PumpError::StructureUnsupported);
        };
        Self::reconcile_planned_view(tree, *child, view, components, changes, plan)
    }

    fn mount_planned_view(
        tree: &mut Tree,
        logical_parent: Option<NodeId>,
        key: Option<Key>,
        view: View,
        components: &mut ComponentStore,
        changes: &mut ComponentChanges,
        plan: &mut UpdatePlan,
    ) -> Result<(NodeId, Vec<NodeId>), PumpError> {
        match view {
            View::Native(element) => {
                let node = Self::mount_planned_element(tree, logical_parent, key, element, plan)?;
                Ok((node, vec![node]))
            }
            View::Component(component) => {
                let token = component.reserve(components)?;
                changes.reserved.push(token);
                let node = tree.insert_component(
                    logical_parent,
                    key,
                    token.scope(),
                    component.component_type(),
                )?;
                let slot = tree.insert(Some(node), NodeKind::Slot)?;
                let view = components.view(token)?;
                let (_, native) = Self::mount_planned_view(
                    tree,
                    Some(slot),
                    None,
                    view,
                    components,
                    changes,
                    plan,
                )?;
                Ok((node, native))
            }
            View::Empty => {
                let node = tree.insert_fragment(logical_parent, key)?;
                Ok((node, Vec::new()))
            }
            View::Fragment(children) => {
                let node = tree.insert_fragment(logical_parent, key)?;
                let children = Rc::unwrap_or_clone(children);
                let keys = children
                    .iter()
                    .map(|child| child.key().clone())
                    .collect::<Vec<_>>();
                diff(&[], &keys)
                    .map_err(|KeyedError::DuplicateKey(key)| PumpError::DuplicateKey(key))?;
                let mut native = Vec::new();
                for child in children {
                    let (key, view) = child.into_parts();
                    let (_, child_native) = Self::mount_planned_view(
                        tree,
                        Some(node),
                        Some(key),
                        view,
                        components,
                        changes,
                        plan,
                    )?;
                    native.extend(child_native);
                }
                Ok((node, native))
            }
            View::Content { control, content } => {
                if !Self::element_structure_is_empty(&control)
                    || !Self::control_has_role(control.kind(), ControlRole::Content)
                {
                    return Err(PumpError::StructureUnsupported);
                }
                let node = Self::mount_planned_element(tree, logical_parent, key, control, plan)?;
                let (_, native) = Self::mount_planned_view(
                    tree,
                    Some(node),
                    None,
                    *content,
                    components,
                    changes,
                    plan,
                )?;
                match native.as_slice() {
                    [] => {}
                    [child] => {
                        plan.push(Command::InsertChild {
                            parent: node,
                            child: *child,
                            index: 0,
                        });
                    }
                    _ => return Err(PumpError::StructureUnsupported),
                }
                Ok((node, vec![node]))
            }
            View::Children { control, children } => {
                if !Self::element_structure_is_empty(&control)
                    || !Self::control_has_role(control.kind(), ControlRole::Children)
                {
                    return Err(PumpError::StructureUnsupported);
                }
                let node = Self::mount_planned_element(tree, logical_parent, key, control, plan)?;
                let children = Rc::unwrap_or_clone(children);
                let keys = children
                    .iter()
                    .map(|child| child.key().clone())
                    .collect::<Vec<_>>();
                diff(&[], &keys)
                    .map_err(|KeyedError::DuplicateKey(key)| PumpError::DuplicateKey(key))?;
                let mut native_index = 0;
                for child in children {
                    let (key, view) = child.into_parts();
                    let (_, native) = Self::mount_planned_view(
                        tree,
                        Some(node),
                        Some(key),
                        view,
                        components,
                        changes,
                        plan,
                    )?;
                    for child in native {
                        plan.push(Command::InsertChild {
                            parent: node,
                            child,
                            index: native_index,
                        });
                        native_index += 1;
                    }
                }
                Ok((node, vec![node]))
            }
            View::VirtualItems { .. } => Err(PumpError::StructureUnsupported),
        }
    }

    fn remove_reservations(components: &mut ComponentStore, reserved: &[ComponentToken]) {
        for token in reserved.iter().rev().copied() {
            _ = components.remove(token);
        }
    }

    fn retire_planned_subtree(
        tree: &mut Tree,
        root: NodeId,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let nodes = tree.subtree_postorder(root)?;
        plan.commits.retain(|commit| !nodes.contains(&commit.node));
        for node in nodes {
            match tree.kind(node)? {
                NodeKind::Native(_) => {
                    if let Some(parent) = tree.parent(node)? {
                        let parent = Self::native_container(tree, parent)?;
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
                NodeKind::VirtualCollection => {
                    if let Some(parent) = tree.parent(node)? {
                        let parent = Self::native_container(tree, parent)?;
                        plan.push(Command::RemoveChild {
                            parent,
                            child: node,
                        });
                    }
                    plan.push(Command::Destroy { node });
                }
                NodeKind::Component | NodeKind::Fragment | NodeKind::Slot => {}
                NodeKind::Application | NodeKind::Window => {
                    return Err(PumpError::StructureUnsupported);
                }
            }
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
    use std::any::TypeId;
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
        error: Option<NativeWork<QueuedEventError>>,
        identity: Option<NativeIdentity>,
    }

    impl NativeRuntime for EventErrorRuntime {
        fn apply(&mut self, commands: &[Command]) -> CommitReceipt {
            CommitReceipt {
                outcomes: vec![CommandOutcome::Applied; commands.len()],
            }
        }

        fn reset(&mut self) {}

        fn set_identity(&mut self, identity: NativeIdentity) {
            self.identity = Some(identity);
        }

        fn drain_event_errors(&mut self) -> Vec<NativeWork<QueuedEventError>> {
            self.error.take().into_iter().collect()
        }
    }

    struct Leaf {
        text: String,
    }

    impl Component for Leaf {
        type Props = String;
        type Message = ();

        fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
            Self {
                text: props.clone(),
            }
        }

        fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.text.clone_from(props);
        }

        fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            View::native(TextBlock::new().text(self.text.clone()))
        }
    }

    struct Root {
        text: String,
    }

    impl Component for Root {
        type Props = String;
        type Message = String;

        fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
            context.sender().send("message".to_string());
            Self {
                text: props.clone(),
            }
        }

        fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.text.clone_from(props);
        }

        fn update(&mut self, message: Self::Message, _context: &mut ComponentContext<Self>) {
            self.text = message;
        }

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            View::component::<Leaf>(self.text.clone())
        }
    }

    struct List {
        items: Vec<(u64, String)>,
    }

    impl Component for List {
        type Props = Vec<(u64, String)>;
        type Message = ();

        fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
            Self {
                items: props.clone(),
            }
        }

        fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.items.clone_from(props);
        }

        fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            View::children(
                StackPanel::new(),
                self.items
                    .iter()
                    .map(|(key, text)| KeyedView::new(*key, View::component::<Leaf>(text.clone()))),
            )
        }
    }

    struct AltLeaf {
        text: String,
    }

    impl Component for AltLeaf {
        type Props = String;
        type Message = ();

        fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
            Self {
                text: props.clone(),
            }
        }

        fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.text.clone_from(props);
        }

        fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            View::native(TextBlock::new().text(format!("alt:{}", self.text)))
        }
    }

    struct MixedList {
        alt: bool,
    }

    impl Component for MixedList {
        type Props = bool;
        type Message = bool;

        fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
            Self { alt: *props }
        }

        fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.alt = *props;
        }

        fn update(&mut self, message: Self::Message, _context: &mut ComponentContext<Self>) {
            self.alt = message;
        }

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            let child = if self.alt {
                View::component::<AltLeaf>("value".to_string())
            } else {
                View::component::<Leaf>("value".to_string())
            };
            View::children(StackPanel::new(), [KeyedView::new(1u64, child)])
        }
    }

    #[derive(Clone)]
    struct ViewCounts {
        child: Rc<Cell<u32>>,
        parent: Rc<Cell<u32>>,
    }

    impl PartialEq for ViewCounts {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.child, &other.child) && Rc::ptr_eq(&self.parent, &other.parent)
        }
    }

    struct CountingChild {
        views: Rc<Cell<u32>>,
    }

    impl Component for CountingChild {
        type Props = Rc<Cell<u32>>;
        type Message = ();

        fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
            Self {
                views: Rc::clone(props),
            }
        }

        fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.views = Rc::clone(props);
        }

        fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            self.views.set(self.views.get() + 1);
            View::native(TextBlock::new())
        }
    }

    struct CountingParent {
        counts: ViewCounts,
    }

    impl Component for CountingParent {
        type Props = ViewCounts;
        type Message = ();

        fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
            Self {
                counts: props.clone(),
            }
        }

        fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.counts = props.clone();
        }

        fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            self.counts.parent.set(self.counts.parent.get() + 1);
            View::component::<CountingChild>(Rc::clone(&self.counts.child))
        }
    }

    #[test]
    fn mounts_a_component_chain_into_the_authoritative_tree() {
        let mut pump = Pump::new(RecordingRuntime::default());

        pump.mount_view(View::component::<Root>("leaf".to_string()))
            .unwrap();

        let root = pump.root().unwrap();
        assert_eq!(pump.tree.kind(root), Ok(NodeKind::Component));
        assert_eq!(pump.tree.component_type(root), Ok(TypeId::of::<Root>()));
        let root_scope = pump.tree.component_scope(root).unwrap();
        let root_slot = pump.tree.children(root).unwrap()[0];
        assert_eq!(pump.tree.kind(root_slot), Ok(NodeKind::Slot));
        let leaf = pump.tree.children(root_slot).unwrap()[0];
        assert_eq!(pump.tree.component_type(leaf), Ok(TypeId::of::<Leaf>()));
        let leaf_scope = pump.tree.component_scope(leaf).unwrap();
        let leaf_slot = pump.tree.children(leaf).unwrap()[0];
        let native = pump.tree.children(leaf_slot).unwrap()[0];
        assert_eq!(
            pump.tree.kind(native),
            Ok(NodeKind::Native(MountedKind::TextBlock))
        );
        assert_eq!(
            pump.runtime()
                .node(native)
                .unwrap()
                .property(PropertyId::TextBlockText),
            Some(&PropertyValue::Str("leaf".to_string()))
        );
        pump.update_view(View::component::<Root>("props".to_string()))
            .unwrap();
        assert_eq!(pump.root(), Some(root));
        assert_eq!(pump.tree.component_scope(root), Ok(root_scope));
        assert_eq!(pump.tree.component_scope(leaf), Ok(leaf_scope));
        assert_eq!(
            pump.runtime()
                .node(native)
                .unwrap()
                .property(PropertyId::TextBlockText),
            Some(&PropertyValue::Str("props".to_string()))
        );

        assert_eq!(pump.dispatch_components(10), Ok(1));
        assert_eq!(
            pump.runtime()
                .node(native)
                .unwrap()
                .property(PropertyId::TextBlockText),
            Some(&PropertyValue::Str("message".to_string()))
        );
    }

    #[test]
    fn structural_mount_failure_discards_reserved_component_scopes() {
        let mut runtime = RecordingRuntime::default();
        runtime.fail_at(0);
        let mut pump = Pump::new(runtime);

        assert!(matches!(
            pump.mount_view(View::component::<Root>("leaf".to_string())),
            Err(PumpError::StructuralApplyFailed(_))
        ));
        assert_eq!(pump.components().pending(), 0);
        assert_eq!(pump.components_mut().drain(10).unwrap().dropped, 0);
    }

    #[test]
    fn component_slot_adapters_reject_incompatible_control_roles() {
        let mut children = Pump::new(RecordingRuntime::default());
        assert_eq!(
            children.mount_view(View::Children {
                control: TextBlock::new().into(),
                children: Rc::new(Vec::new()),
            }),
            Err(PumpError::StructureUnsupported)
        );
        assert!(!children.poisoned());

        let mut content = Pump::new(RecordingRuntime::default());
        assert_eq!(
            content.mount_view(View::Content {
                control: StackPanel::new().into(),
                content: Box::new(View::native(TextBlock::new())),
            }),
            Err(PumpError::StructureUnsupported)
        );
        assert!(!content.poisoned());
    }

    #[test]
    fn empty_root_mounts_without_native_window_content_and_can_toggle() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::Empty).unwrap();
        let root = pump.root().unwrap();
        let window = pump.window().unwrap();

        assert_eq!(pump.tree.kind(root), Ok(NodeKind::Fragment));
        assert!(pump.runtime().node(window).unwrap().children().is_empty());

        pump.update_view(View::native(TextBlock::new().text("visible")))
            .unwrap();
        assert_eq!(pump.runtime().node(window).unwrap().children().len(), 1);
        pump.update_view(View::Empty).unwrap();
        assert!(pump.runtime().node(window).unwrap().children().is_empty());
    }

    #[test]
    fn multi_root_fragment_is_rejected_in_window_and_content_slots() {
        let fragment = || {
            View::fragment([
                KeyedView::new("a", View::native(TextBlock::new().text("A"))),
                KeyedView::new("b", View::native(TextBlock::new().text("B"))),
            ])
        };
        let mut window = Pump::new(RecordingRuntime::default());
        assert_eq!(
            window.mount_view(fragment()),
            Err(PumpError::StructureUnsupported)
        );
        assert!(window.root().is_none());

        let mut content = Pump::new(RecordingRuntime::default());
        assert_eq!(
            content.mount_view(View::content(Button::new(), fragment())),
            Err(PumpError::StructureUnsupported)
        );
        assert!(content.root().is_none());
    }

    #[test]
    fn fragment_splices_into_children_and_retains_keyed_component_scope() {
        let view = |reverse: bool| {
            let fragment = if reverse {
                View::fragment([
                    KeyedView::new("text", View::native(TextBlock::new().text("text"))),
                    KeyedView::new("leaf", View::component::<Leaf>("leaf".to_string())),
                ])
            } else {
                View::fragment([
                    KeyedView::new("leaf", View::component::<Leaf>("leaf".to_string())),
                    KeyedView::new("text", View::native(TextBlock::new().text("text"))),
                ])
            };
            View::children(
                StackPanel::new(),
                [
                    KeyedView::new("empty", View::Empty),
                    KeyedView::new("group", fragment),
                ],
            )
        };
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(view(false)).unwrap();
        let root = pump.root().unwrap();
        let fragment = pump.tree.children(root).unwrap()[1];
        let leaf = pump.tree.children(fragment).unwrap()[0];
        let scope = pump.tree.component_scope(leaf).unwrap();

        assert_eq!(recorded_text(pump.runtime(), root), ["leaf", "text"]);
        pump.update_view(view(true)).unwrap();

        let fragment = pump.tree.children(root).unwrap()[1];
        let leaf = pump.tree.children(fragment).unwrap()[1];
        assert_eq!(pump.tree.component_scope(leaf), Ok(scope));
        assert_eq!(recorded_text(pump.runtime(), root), ["text", "leaf"]);
    }

    #[test]
    fn fragment_synchronization_failure_recovers_desired_native_order() {
        let view = |reverse: bool| {
            let children = if reverse {
                [
                    KeyedView::new("b", View::native(TextBlock::new().text("B"))),
                    KeyedView::new("a", View::native(TextBlock::new().text("A"))),
                ]
            } else {
                [
                    KeyedView::new("a", View::native(TextBlock::new().text("A"))),
                    KeyedView::new("b", View::native(TextBlock::new().text("B"))),
                ]
            };
            View::children(
                StackPanel::new(),
                [KeyedView::new("fragment", View::fragment(children))],
            )
        };
        let mut probe = Pump::new(RecordingRuntime::default());
        probe.mount_view(view(false)).unwrap();
        probe.update_view(view(true)).unwrap();
        let synchronize = probe.runtime().commands()[1]
            .iter()
            .enumerate()
            .filter(|(_, command)| matches!(command, Command::SynchronizeChildren { .. }))
            .collect::<Vec<_>>();
        assert_eq!(synchronize.len(), 1);
        let failed_index = synchronize[0].0;

        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(view(false)).unwrap();
        pump.runtime_mut().fail_at(failed_index);
        assert!(matches!(
            pump.update_view(view(true)),
            Err(PumpError::RecoveredStructure(_))
        ));
        assert_eq!(
            recorded_text(pump.runtime(), pump.root().unwrap()),
            ["B", "A"]
        );
        assert!(!pump.poisoned());
    }

    #[test]
    fn component_can_toggle_between_empty_and_one_native_root() {
        #[derive(Clone)]
        struct Props(Rc<RefCell<Option<LocalSender<bool>>>>);

        impl PartialEq for Props {
            fn eq(&self, other: &Self) -> bool {
                Rc::ptr_eq(&self.0, &other.0)
            }
        }

        struct OptionalLeaf {
            visible: bool,
        }

        impl Component for OptionalLeaf {
            type Message = bool;
            type Props = Props;

            fn create(props: &Props, context: &mut ComponentContext<Self>) -> Self {
                *props.0.borrow_mut() = Some(context.sender());
                Self { visible: false }
            }

            fn changed(&mut self, _props: &Props, _context: &mut ComponentContext<Self>) {}

            fn update(&mut self, visible: bool, _context: &mut ComponentContext<Self>) {
                self.visible = visible;
            }

            fn view(&self, _context: &mut ViewContext<Self>) -> View {
                if self.visible {
                    View::native(TextBlock::new().text("visible"))
                } else {
                    View::Empty
                }
            }
        }

        let sender = Rc::new(RefCell::new(None));
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::children(
            StackPanel::new(),
            [KeyedView::new(
                "optional",
                View::component::<OptionalLeaf>(Props(Rc::clone(&sender))),
            )],
        ))
        .unwrap();
        let root = pump.root().unwrap();
        assert!(pump.runtime().node(root).unwrap().children().is_empty());

        assert!(sender.borrow().as_ref().unwrap().send(true));
        pump.dispatch_components(1).unwrap();
        assert_eq!(recorded_text(pump.runtime(), root), ["visible"]);

        assert!(sender.borrow().as_ref().unwrap().send(false));
        pump.dispatch_components(1).unwrap();
        assert!(pump.runtime().node(root).unwrap().children().is_empty());
    }

    #[test]
    fn local_probe_fallback_composes_once() {
        #[derive(Clone)]
        struct Props {
            sender: Rc<RefCell<Option<LocalSender<bool>>>>,
            views: Rc<Cell<u8>>,
        }

        impl PartialEq for Props {
            fn eq(&self, other: &Self) -> bool {
                Rc::ptr_eq(&self.sender, &other.sender) && Rc::ptr_eq(&self.views, &other.views)
            }
        }

        struct ShapeChange {
            expanded: bool,
            views: Rc<Cell<u8>>,
        }

        impl Component for ShapeChange {
            type Props = Props;
            type Message = bool;

            fn create(props: &Props, context: &mut ComponentContext<Self>) -> Self {
                *props.sender.borrow_mut() = Some(context.sender());
                Self {
                    expanded: false,
                    views: Rc::clone(&props.views),
                }
            }

            fn changed(&mut self, _props: &Props, _context: &mut ComponentContext<Self>) {}

            fn update(&mut self, expanded: bool, _context: &mut ComponentContext<Self>) {
                self.expanded = expanded;
            }

            fn view(&self, _context: &mut ViewContext<Self>) -> View {
                self.views.set(self.views.get() + 1);
                if self.expanded {
                    View::children(
                        StackPanel::new(),
                        [KeyedView::new(
                            "child",
                            View::native(TextBlock::new().text("expanded")),
                        )],
                    )
                } else {
                    View::native(TextBlock::new().text("collapsed"))
                }
            }
        }

        let sender = Rc::new(RefCell::new(None));
        let views = Rc::new(Cell::new(0));
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<ShapeChange>(Props {
            sender: Rc::clone(&sender),
            views: Rc::clone(&views),
        }))
        .unwrap();
        assert_eq!(views.get(), 1);

        assert!(sender.borrow().as_ref().unwrap().send(true));
        pump.dispatch_components(1).unwrap();
        assert_eq!(views.get(), 2);
    }

    #[test]
    fn recovered_component_candidate_retires_its_dirty_token() {
        #[derive(Clone)]
        struct Props(Rc<RefCell<Option<LocalSender<bool>>>>);

        impl PartialEq for Props {
            fn eq(&self, other: &Self) -> bool {
                Rc::ptr_eq(&self.0, &other.0)
            }
        }

        struct ShapeChange(bool);

        impl Component for ShapeChange {
            type Props = Props;
            type Message = bool;

            fn create(props: &Props, context: &mut ComponentContext<Self>) -> Self {
                *props.0.borrow_mut() = Some(context.sender());
                Self(false)
            }

            fn changed(&mut self, _props: &Props, _context: &mut ComponentContext<Self>) {}

            fn update(&mut self, changed: bool, _context: &mut ComponentContext<Self>) {
                self.0 = changed;
            }

            fn view(&self, _context: &mut ViewContext<Self>) -> View {
                if self.0 {
                    View::native(Button::new())
                } else {
                    View::native(TextBlock::new())
                }
            }
        }

        let sender = Rc::new(RefCell::new(None));
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<ShapeChange>(Props(Rc::clone(&sender))))
            .unwrap();
        assert!(sender.borrow().as_ref().unwrap().send(true));
        pump.runtime_mut().fail_at(0);
        assert!(matches!(
            pump.dispatch_components(1),
            Err(PumpError::RecoveredStructure(_))
        ));
        assert!(pump.dirty_components.is_empty());

        let batches = pump.runtime().batches();
        assert_eq!(pump.dispatch_components(1), Ok(0));
        assert_eq!(pump.runtime().batches(), batches);
    }

    #[test]
    fn keyed_component_siblings_retain_scopes_across_prop_updates() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<List>(vec![
            (1, "one".to_string()),
            (2, "two".to_string()),
        ]))
        .unwrap();

        let root = pump.root().unwrap();
        let slot = pump.tree.children(root).unwrap()[0];
        let panel = pump.tree.children(slot).unwrap()[0];
        let children = pump.tree.children(panel).unwrap().to_vec();
        let scopes = children
            .iter()
            .map(|node| pump.tree.component_scope(*node).unwrap())
            .collect::<Vec<_>>();

        pump.update_view(View::component::<List>(vec![
            (1, "first".to_string()),
            (2, "second".to_string()),
        ]))
        .unwrap();

        assert_eq!(pump.tree.children(panel), Ok(children.as_slice()));
        assert_eq!(
            children
                .iter()
                .map(|node| pump.tree.component_scope(*node).unwrap())
                .collect::<Vec<_>>(),
            scopes
        );
        assert_eq!(
            recorded_text(pump.runtime(), panel),
            vec!["first".to_string(), "second".to_string()]
        );

        pump.update_view(View::component::<List>(vec![
            (2, "second".to_string()),
            (1, "first".to_string()),
        ]))
        .unwrap();

        assert_eq!(
            pump.tree.children(panel),
            Ok(&[children[1], children[0]][..])
        );
        assert_eq!(pump.tree.component_scope(children[0]), Ok(scopes[0]));
        assert_eq!(pump.tree.component_scope(children[1]), Ok(scopes[1]));
        assert_eq!(
            recorded_text(pump.runtime(), panel),
            vec!["second".to_string(), "first".to_string()]
        );

        let removed = pump
            .components()
            .token(pump.tree.component_scope(children[0]).unwrap())
            .unwrap();
        let removed_sender = pump.components().sender::<()>(removed).unwrap();
        pump.update_view(View::component::<List>(vec![
            (2, "second".to_string()),
            (3, "third".to_string()),
        ]))
        .unwrap();

        let updated = pump.tree.children(panel).unwrap();
        assert_eq!(updated.len(), 2);
        assert_eq!(pump.tree.component_scope(updated[0]), Ok(scopes[1]));
        assert_ne!(pump.tree.component_scope(updated[1]), Ok(scopes[0]));
        assert_eq!(
            recorded_text(pump.runtime(), panel),
            vec!["second".to_string(), "third".to_string()]
        );
        removed_sender.send(());
        assert_eq!(pump.components_mut().drain(1).unwrap().dropped, 0);
    }

    #[test]
    fn same_key_different_component_type_replaces_and_retires_scope() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<MixedList>(false))
            .unwrap();
        let root = pump.root().unwrap();
        let slot = pump.tree.children(root).unwrap()[0];
        let panel = pump.tree.children(slot).unwrap()[0];
        let old = pump.tree.children(panel).unwrap()[0];
        let old_scope = pump.tree.component_scope(old).unwrap();
        let old_token = pump.components().token(old_scope).unwrap();
        let old_sender = pump.components().sender::<()>(old_token).unwrap();

        pump.update_view(View::component::<MixedList>(true))
            .unwrap();

        let replacement = pump.tree.children(panel).unwrap()[0];
        assert_eq!(
            pump.tree.component_type(replacement),
            Ok(TypeId::of::<AltLeaf>())
        );
        assert_ne!(pump.tree.component_scope(replacement), Ok(old_scope));
        assert_eq!(
            recorded_text(pump.runtime(), panel),
            vec!["alt:value".to_string()]
        );
        old_sender.send(());
        assert_eq!(pump.components_mut().drain(1).unwrap().dropped, 0);
    }

    #[test]
    fn failed_type_replacement_recovers_and_commits_scope_transaction() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<MixedList>(false))
            .unwrap();
        let root = pump.root().unwrap();
        let slot = pump.tree.children(root).unwrap()[0];
        let panel = pump.tree.children(slot).unwrap()[0];
        let old = pump.tree.children(panel).unwrap()[0];
        let old_token = pump
            .components()
            .token(pump.tree.component_scope(old).unwrap())
            .unwrap();
        let old_sender = pump.components().sender::<()>(old_token).unwrap();
        let identity = pump.native_identity();
        pump.runtime_mut().fail_after(0, 0);

        assert!(matches!(
            pump.update_view(View::component::<MixedList>(true)),
            Err(PumpError::RecoveredStructure(_))
        ));
        assert!(!pump.poisoned());
        assert_eq!(pump.native_identity().window(), identity.window());
        assert_ne!(
            pump.native_identity().realization_epoch(),
            identity.realization_epoch()
        );
        assert_eq!(
            recorded_text(pump.runtime(), panel),
            vec!["alt:value".to_string()]
        );
        old_sender.send(());
        assert_eq!(pump.components_mut().drain(1).unwrap().dropped, 0);
    }

    #[test]
    fn failed_component_recovery_discards_new_scope_without_retiring_old_scope() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<MixedList>(false))
            .unwrap();
        let root = pump.root().unwrap();
        let slot = pump.tree.children(root).unwrap()[0];
        let panel = pump.tree.children(slot).unwrap()[0];
        let old = pump.tree.children(panel).unwrap()[0];
        let old_token = pump
            .components()
            .token(pump.tree.component_scope(old).unwrap())
            .unwrap();
        let old_sender = pump.components().sender::<()>(old_token).unwrap();
        pump.runtime_mut().fail_after(0, 0);
        pump.runtime_mut().fail_after(1, 0);

        assert!(matches!(
            pump.update_view(View::component::<MixedList>(true)),
            Err(PumpError::RecoveryFailed(_))
        ));
        assert!(pump.poisoned());
        old_sender.send(());
        assert_eq!(pump.components_mut().drain(1).unwrap().dispatched, 1);
    }

    #[test]
    fn parent_replacement_discards_dirty_work_for_the_retired_child() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<MixedList>(false))
            .unwrap();
        let root = pump.root().unwrap();
        let root_token = pump
            .components()
            .token(pump.tree.component_scope(root).unwrap())
            .unwrap();
        let root_sender = pump.components().sender::<bool>(root_token).unwrap();
        let slot = pump.tree.children(root).unwrap()[0];
        let panel = pump.tree.children(slot).unwrap()[0];
        let child = pump.tree.children(panel).unwrap()[0];
        let child_token = pump
            .components()
            .token(pump.tree.component_scope(child).unwrap())
            .unwrap();
        let child_sender = pump.components().sender::<()>(child_token).unwrap();

        root_sender.send(true);
        child_sender.send(());
        assert_eq!(pump.dispatch_components(10), Ok(2));

        assert_eq!(
            recorded_text(pump.runtime(), panel),
            vec!["alt:value".to_string()]
        );
        child_sender.send(());
        assert_eq!(pump.components_mut().drain(1).unwrap().dropped, 0);
    }

    #[test]
    fn dirty_parent_and_child_each_compose_once_parent_first() {
        let counts = ViewCounts {
            child: Rc::new(Cell::new(0)),
            parent: Rc::new(Cell::new(0)),
        };
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<CountingParent>(counts.clone()))
            .unwrap();
        let parent = pump.root().unwrap();
        let parent_token = pump
            .components()
            .token(pump.tree.component_scope(parent).unwrap())
            .unwrap();
        let slot = pump.tree.children(parent).unwrap()[0];
        let child = pump.tree.children(slot).unwrap()[0];
        let child_token = pump
            .components()
            .token(pump.tree.component_scope(child).unwrap())
            .unwrap();

        pump.components()
            .sender::<()>(parent_token)
            .unwrap()
            .send(());
        pump.components()
            .sender::<()>(child_token)
            .unwrap()
            .send(());
        assert_eq!(pump.dispatch_components(10), Ok(2));
        assert_eq!(counts.parent.get(), 2);
        assert_eq!(counts.child.get(), 2);
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
            pump.tree
                .native(root)
                .unwrap()
                .properties
                .get(&PropertyId::TextBlockText),
            Some(&NativePropertyState::Divergent { attempts: 1 })
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
        assert_eq!(pump.version(), version + 1);
        assert!(!pump.retry_pending());
        assert_eq!(
            pump.runtime()
                .node(root)
                .unwrap()
                .property(PropertyId::TextBlockText),
            Some(&PropertyValue::Str("second".into()))
        );
        assert_eq!(
            pump.tree
                .native(root)
                .unwrap()
                .properties
                .get(&PropertyId::TextBlockText),
            Some(&NativePropertyState::Known(Some(PropertyValue::Str(
                "second".into()
            ))))
        );
    }

    #[test]
    fn property_retry_exhaustion_is_tracked_by_the_property() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(TextBlock::new().text("first").into()).unwrap();

        for attempt in 1..=MAX_PROPERTY_ATTEMPTS {
            pump.runtime_mut().fail_at(0);
            let error = pump
                .update(TextBlock::new().text("second").into())
                .unwrap_err();
            if attempt < MAX_PROPERTY_ATTEMPTS {
                assert!(matches!(error, PumpError::PropertyApplyFailed(_)));
            } else {
                assert!(matches!(error, PumpError::PropertyRetriesExhausted(_)));
                assert!(!error.recoverable());
            }
        }
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
    fn dense_keyed_reorder_resets_collection_without_recreating_children() {
        let labels = (0..512).map(|index| index.to_string()).collect::<Vec<_>>();
        let mut reversed = labels.clone();
        reversed.reverse();
        let element =
            |labels: &[String]| {
                StackPanel::new()
                    .children(labels.iter().map(|label| {
                        KeyedElement::new(label.clone(), TextBlock::new().text(label))
                    }))
                    .into()
            };
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(element(&labels)).unwrap();
        let root = pump.root().unwrap();
        let original = pump
            .tree
            .children(root)
            .unwrap()
            .iter()
            .copied()
            .collect::<HashSet<_>>();

        pump.update(element(&reversed)).unwrap();

        assert!(pump.runtime().commands()[1].contains(&Command::ResetChildren { parent: root }));
        assert_eq!(
            pump.tree
                .children(root)
                .unwrap()
                .iter()
                .copied()
                .collect::<HashSet<_>>(),
            original
        );
        assert_eq!(
            pump.runtime().node(root).unwrap().children(),
            pump.tree.children(root).unwrap()
        );
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
        pump.mount(TextBox::new().on_text_changed(|_| {}).into())
            .unwrap();
        let root = pump.root().unwrap();
        let revision = pump
            .event_revision(root, EventId::TextBoxTextChanged)
            .unwrap();
        let identity = pump.native_identity();
        pump.runtime_mut().error = Some(NativeWork {
            identity,
            work: QueuedEventError {
                node: root,
                event: EventId::TextBoxTextChanged,
                revision,
                error: RuntimeError::Injected,
            },
        });

        assert_eq!(
            pump.dispatch_events(),
            Err(PumpError::EventReadFailed(RuntimeError::Injected))
        );
    }

    #[test]
    fn old_window_event_payload_read_failure_is_ignored() {
        let mut pump = Pump::new(EventErrorRuntime::default());
        pump.mount(TextBox::new().into()).unwrap();
        let root = pump.root().unwrap();
        let revision = pump
            .event_revision(root, EventId::TextBoxTextChanged)
            .unwrap();
        let stale_identity = pump.native_identity();
        pump.shutdown();
        pump.mount(TextBox::new().into()).unwrap();
        assert_eq!(pump.root(), Some(root));
        pump.runtime_mut().error = Some(NativeWork {
            identity: stale_identity,
            work: QueuedEventError {
                node: root,
                event: EventId::TextBoxTextChanged,
                revision,
                error: RuntimeError::Injected,
            },
        });

        assert_eq!(pump.dispatch_events(), Ok(0));
    }

    #[test]
    fn old_realization_event_payload_read_failure_is_ignored() {
        let mut pump = Pump::new(EventErrorRuntime::default());
        pump.mount(TextBox::new().into()).unwrap();
        let root = pump.root().unwrap();
        let revision = pump
            .event_revision(root, EventId::TextBoxTextChanged)
            .unwrap();
        let stale_identity = pump.native_identity();
        pump.identity = stale_identity.next_realization().unwrap();
        let identity = pump.identity;
        pump.runtime_mut().set_identity(identity);
        pump.runtime_mut().error = Some(NativeWork {
            identity: stale_identity,
            work: QueuedEventError {
                node: root,
                event: EventId::TextBoxTextChanged,
                revision,
                error: RuntimeError::Injected,
            },
        });

        assert_eq!(pump.dispatch_events(), Ok(0));
    }

    #[test]
    fn retired_subscription_event_payload_read_failure_is_ignored() {
        let mut pump = Pump::new(EventErrorRuntime::default());
        pump.mount(Button::new().on_click(|| {}).into()).unwrap();
        let root = pump.root().unwrap();
        let revision = pump.event_revision(root, EventId::ButtonClick).unwrap();
        let identity = pump.native_identity();
        pump.update(Button::new().into()).unwrap();
        pump.runtime_mut().error = Some(NativeWork {
            identity,
            work: QueuedEventError {
                node: root,
                event: EventId::ButtonClick,
                revision,
                error: RuntimeError::Injected,
            },
        });

        assert_eq!(pump.dispatch_events(), Ok(0));
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
    fn event_work_budget_preserves_and_reports_pending_work() {
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
        for _ in 0..=EVENT_WORK_BUDGET {
            pump.queue_event(QueuedEvent {
                node: root,
                event: EventId::ButtonClick,
                revision,
                payload: EventPayload::Unit,
            });
        }

        assert_eq!(pump.dispatch_events(), Ok(EVENT_WORK_BUDGET));
        assert_eq!(calls.get(), EVENT_WORK_BUDGET);
        assert!(pump.native_work_pending());
        assert_eq!(pump.dispatch_events(), Ok(1));
        assert_eq!(calls.get(), EVENT_WORK_BUDGET + 1);
        assert!(!pump.native_work_pending());
    }

    #[test]
    fn rejected_controlled_edit_restores_the_desired_value() {
        let observed = Rc::new(RefCell::new(String::new()));
        let capture = Rc::clone(&observed);
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(
            TextBox::new()
                .text("desired")
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
            payload: EventPayload::Str("native".into()),
        });

        assert_eq!(pump.dispatch_events(), Ok(1));
        assert_eq!(&*observed.borrow(), "native");
        assert!(pump.retry_pending());
        assert_eq!(
            pump.tree
                .native(root)
                .unwrap()
                .properties
                .get(&PropertyId::TextBoxText),
            Some(&NativePropertyState::Known(Some(PropertyValue::Str(
                "native".into()
            ))))
        );

        let restored = pump
            .update(
                TextBox::new()
                    .text("desired")
                    .on_text_changed(|_| {})
                    .into(),
            )
            .unwrap();

        assert_eq!(restored.outcomes, [CommandOutcome::Applied]);
        assert!(!pump.retry_pending());
        assert_eq!(
            pump.runtime()
                .node(root)
                .unwrap()
                .property(PropertyId::TextBoxText),
            Some(&PropertyValue::Str("desired".into()))
        );
    }

    #[test]
    fn component_rejected_controlled_edit_restores_the_desired_value() {
        struct Controlled;

        impl Component for Controlled {
            type Message = String;
            type Props = ();

            fn create(_props: &(), _context: &mut ComponentContext<Self>) -> Self {
                Self
            }

            fn changed(&mut self, _props: &(), _context: &mut ComponentContext<Self>) {}

            fn update(&mut self, _message: String, _context: &mut ComponentContext<Self>) {}

            fn view(&self, context: &mut ViewContext<Self>) -> View {
                let sender = context.sender();
                View::native(
                    TextBox::new()
                        .text("desired")
                        .on_text_changed(move |value| {
                            _ = sender.send(value);
                        }),
                )
            }
        }

        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<Controlled>(())).unwrap();
        let root = Pump::<RecordingRuntime>::native_root(&pump.tree, pump.root().unwrap()).unwrap();
        let revision = pump
            .event_revision(root, EventId::TextBoxTextChanged)
            .unwrap();
        pump.queue_event(QueuedEvent {
            node: root,
            event: EventId::TextBoxTextChanged,
            revision,
            payload: EventPayload::Str("native".into()),
        });

        assert_eq!(pump.dispatch_events(), Ok(1));
        assert_eq!(pump.dispatch_components(1), Ok(1));
        assert_eq!(
            pump.runtime()
                .node(root)
                .unwrap()
                .property(PropertyId::TextBoxText),
            Some(&PropertyValue::Str("desired".into()))
        );
        assert!(!pump.retry_pending());
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
    fn realization_work_budget_preserves_and_reports_pending_work() {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(ItemsRepeater::new().into()).unwrap();
        let missing = NodeId::from_parts(u32::MAX, 0);
        for index in 0..=REALIZATION_WORK_BUDGET {
            pump.runtime_mut()
                .queue_realization(RealizationRequest::Realize {
                    collection: missing,
                    container: RealizedContainer(index as u64),
                    index,
                });
        }

        assert_eq!(
            pump.process_realizations().unwrap().len(),
            REALIZATION_WORK_BUDGET
        );
        assert!(pump.native_work_pending());
        assert_eq!(pump.process_realizations().unwrap().len(), 1);
        assert!(!pump.native_work_pending());
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
    fn component_view_reuses_a_virtual_collection_shell_immediately() {
        struct VirtualRoot;

        impl Component for VirtualRoot {
            type Message = ();
            type Props = ();

            fn create(_props: &(), _context: &mut ComponentContext<Self>) -> Self {
                Self
            }

            fn changed(&mut self, _props: &(), _context: &mut ComponentContext<Self>) {}

            fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

            fn view(&self, _context: &mut ViewContext<Self>) -> View {
                View::native(
                    ScrollViewer::new().content(
                        ItemsRepeater::new()
                            .item("a", TextBlock::new().text("A"))
                            .item("b", TextBlock::new().text("B")),
                    ),
                )
            }
        }

        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<VirtualRoot>(())).unwrap();
        let scroll =
            Pump::<RecordingRuntime>::native_root(&pump.tree, pump.root().unwrap()).unwrap();
        let collection = pump.tree.children(scroll).unwrap()[0];
        let container = RealizedContainer(1);
        pump.runtime_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container,
                index: 0,
            });
        let first = pump.process_realizations().unwrap();
        let first_child = pump.tree.children(collection).unwrap()[0];
        pump.runtime_mut()
            .queue_realization(RealizationRequest::Recycle {
                collection,
                container,
            });
        pump.process_realizations().unwrap();
        pump.runtime_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container,
                index: 1,
            });
        let second = pump.process_realizations().unwrap();

        let [RealizationOutcome::Realized(first)] = first.as_slice() else {
            panic!("expected first realization");
        };
        let [RealizationOutcome::Realized(second)] = second.as_slice() else {
            panic!("expected second realization");
        };
        assert_eq!(first.container, container);
        assert_eq!(second.container, container);
        assert_eq!(first.key, Key::from("a"));
        assert_eq!(second.key, Key::from("b"));
        assert_eq!(pump.tree.children(collection).unwrap().len(), 1);
        assert_ne!(pump.tree.children(collection).unwrap()[0], first_child);
        assert_eq!(recorded_text(pump.runtime(), collection), ["B"]);
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

    #[test]
    fn component_effects_commit_after_mount_and_cleanup_once() {
        #[derive(Clone)]
        struct Props {
            log: Rc<RefCell<Vec<String>>>,
            sender: Rc<RefCell<Option<LocalSender<u32>>>>,
        }

        impl PartialEq for Props {
            fn eq(&self, other: &Self) -> bool {
                Rc::ptr_eq(&self.log, &other.log) && Rc::ptr_eq(&self.sender, &other.sender)
            }
        }

        struct EffectComponent {
            log: Rc<RefCell<Vec<String>>>,
            value: u32,
        }

        impl Component for EffectComponent {
            type Message = u32;
            type Props = Props;

            fn create(props: &Props, cx: &mut ComponentContext<Self>) -> Self {
                *props.sender.borrow_mut() = Some(cx.sender());
                Self {
                    log: Rc::clone(&props.log),
                    value: 0,
                }
            }

            fn update(&mut self, message: u32, _cx: &mut ComponentContext<Self>) {
                self.value = message;
            }

            fn changed(&mut self, _props: &Props, _cx: &mut ComponentContext<Self>) {}

            fn view(&self, cx: &mut ViewContext<Self>) -> View {
                let log = Rc::clone(&self.log);
                let value = self.value;
                cx.use_effect(value, move || {
                    log.borrow_mut().push(format!("setup {value}"));
                    Some(Box::new(move || {
                        log.borrow_mut().push(format!("cleanup {value}"));
                    }))
                });
                Element::from(TextBlock::new().text(value.to_string())).into()
            }
        }

        let log = Rc::new(RefCell::new(Vec::new()));
        let sender = Rc::new(RefCell::new(None));
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<EffectComponent>(Props {
            log: Rc::clone(&log),
            sender: Rc::clone(&sender),
        }))
        .unwrap();
        assert_eq!(&*log.borrow(), &["setup 0"]);

        sender.borrow().as_ref().unwrap().send(1);
        pump.dispatch_components(1).unwrap();
        assert_eq!(&*log.borrow(), &["setup 0", "cleanup 0", "setup 1"]);

        pump.shutdown();
        assert_eq!(
            &*log.borrow(),
            &["setup 0", "cleanup 0", "setup 1", "cleanup 1"]
        );
        drop(pump);
        assert_eq!(
            &*log.borrow(),
            &["setup 0", "cleanup 0", "setup 1", "cleanup 1"]
        );
    }

    #[test]
    fn component_host_retries_initial_property_failure_without_a_message() {
        let mut probe = Pump::new(RecordingRuntime::default());
        probe
            .mount_view(View::component::<Leaf>("value".to_string()))
            .unwrap();
        let failed = probe.runtime().commands()[0]
            .iter()
            .position(|command| matches!(command, Command::SetProperty { .. }))
            .unwrap();

        let mut runtime = RecordingRuntime::default();
        runtime.fail_at(failed);
        let mut pump = Pump::new(runtime);
        assert!(matches!(
            pump.mount_view(View::component::<Leaf>("value".to_string())),
            Err(PumpError::PropertyApplyFailed(_))
        ));
        assert!(pump.native_work_pending());

        assert_eq!(pump.dispatch_components(64), Ok(0));
        assert!(!pump.retry_pending());
        assert!(!pump.native_work_pending());
    }

    #[test]
    fn failed_component_recovery_does_not_commit_pending_effects() {
        #[derive(Clone)]
        struct Props {
            alternate: bool,
            log: Rc<RefCell<Vec<String>>>,
        }

        impl PartialEq for Props {
            fn eq(&self, other: &Self) -> bool {
                self.alternate == other.alternate && Rc::ptr_eq(&self.log, &other.log)
            }
        }

        struct EffectComponent(Props);

        impl Component for EffectComponent {
            type Message = ();
            type Props = Props;

            fn create(props: &Props, _cx: &mut ComponentContext<Self>) -> Self {
                Self(props.clone())
            }

            fn update(&mut self, _message: (), _cx: &mut ComponentContext<Self>) {}

            fn changed(&mut self, props: &Props, _cx: &mut ComponentContext<Self>) {
                self.0 = props.clone();
            }

            fn view(&self, cx: &mut ViewContext<Self>) -> View {
                let alternate = self.0.alternate;
                let log = Rc::clone(&self.0.log);
                cx.use_effect(alternate, move || {
                    log.borrow_mut().push(format!("setup {alternate}"));
                    Some(Box::new(move || {
                        log.borrow_mut().push(format!("cleanup {alternate}"));
                    }))
                });
                if alternate {
                    Element::from(Button::new()).into()
                } else {
                    Element::from(TextBlock::new()).into()
                }
            }
        }

        let log = Rc::new(RefCell::new(Vec::new()));
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<EffectComponent>(Props {
            alternate: false,
            log: Rc::clone(&log),
        }))
        .unwrap();
        pump.runtime_mut().fail_after(0, 0);
        pump.runtime_mut().fail_after(1, 0);

        assert!(matches!(
            pump.update_view(View::component::<EffectComponent>(Props {
                alternate: true,
                log: Rc::clone(&log),
            })),
            Err(PumpError::RecoveryFailed(_))
        ));
        assert_eq!(&*log.borrow(), &["setup false", "cleanup false"]);
    }

    #[test]
    fn retired_component_effects_cleanup_child_first() {
        #[derive(Clone)]
        struct Props {
            child: bool,
            log: Rc<RefCell<Vec<&'static str>>>,
            name: &'static str,
        }

        impl PartialEq for Props {
            fn eq(&self, other: &Self) -> bool {
                self.child == other.child
                    && self.name == other.name
                    && Rc::ptr_eq(&self.log, &other.log)
            }
        }

        struct EffectTree(Props);

        impl Component for EffectTree {
            type Message = ();
            type Props = Props;

            fn create(props: &Props, _cx: &mut ComponentContext<Self>) -> Self {
                Self(props.clone())
            }

            fn update(&mut self, _message: (), _cx: &mut ComponentContext<Self>) {}

            fn changed(&mut self, props: &Props, _cx: &mut ComponentContext<Self>) {
                self.0 = props.clone();
            }

            fn view(&self, cx: &mut ViewContext<Self>) -> View {
                let cleanup = self.0.name;
                let log = Rc::clone(&self.0.log);
                cx.use_effect((), move || {
                    Some(Box::new(move || {
                        log.borrow_mut().push(cleanup);
                    }))
                });
                if self.0.child {
                    View::component::<Self>(Props {
                        child: false,
                        log: Rc::clone(&self.0.log),
                        name: "child",
                    })
                } else {
                    View::native(TextBlock::new())
                }
            }
        }

        let log = Rc::new(RefCell::new(Vec::new()));
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<EffectTree>(Props {
            child: true,
            log: Rc::clone(&log),
            name: "parent",
        }))
        .unwrap();

        pump.update_view(View::native(TextBlock::new())).unwrap();
        assert_eq!(&*log.borrow(), &["child", "parent"]);
    }
}
