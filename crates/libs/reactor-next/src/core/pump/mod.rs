use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

use super::*;

mod lifecycle;
mod native_work;
mod plan;
mod planner;
mod publish;
#[cfg(test)]
mod tests;
mod turn;

use plan::*;

#[cfg(test)]
use native_work::{EVENT_WORK_BUDGET, REALIZATION_WORK_BUDGET};

#[cfg(test)]
use plan::RECOVERY_COMMAND_BUDGET;

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
    RecoveryPending,
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
            Self::PropertyApplyFailed(_) | Self::RecoveredStructure(_) | Self::RecoveryPending
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

const MAX_PROPERTY_ATTEMPTS: u8 = 3;

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
    pending_recovery: Option<PendingRecovery>,
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
            pending_recovery: None,
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
        if self.pending_recovery.is_some() {
            return Err(PumpError::RecoveryPending);
        }
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

    pub fn update(&mut self, element: Element) -> Result<CommitReceipt, PumpError> {
        if self.pending_recovery.is_some() {
            return Err(PumpError::RecoveryPending);
        }
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
        self.publish_candidate(
            CandidateState::Tree {
                tree: candidate,
                root: candidate_root,
            },
            plan,
            FrontendChanges::Element(recovery_element),
            next_version,
        )
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
        self.pending_recovery = None;
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

    pub fn recovery_pending(&self) -> bool {
        self.pending_recovery.is_some()
    }

    pub(crate) fn components(&self) -> &ComponentStore {
        &self.components
    }

    pub(crate) fn components_mut(&mut self) -> &mut ComponentStore {
        &mut self.components
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

    fn commit_candidate_properties(
        &mut self,
        candidate: &mut CandidateState,
        commits: &[PropertyCommit],
        receipt: &CommitReceipt,
    ) -> Result<bool, PumpError> {
        match candidate {
            CandidateState::Tree { tree, .. } => {
                Self::commit_tree_properties(tree, commits, receipt)
            }
            CandidateState::Native { node, .. } => {
                let native = self.tree.native_mut(*node)?;
                let mut retries_exhausted = false;
                for commit in commits {
                    if commit.node != *node {
                        return Err(PumpError::StructureUnsupported);
                    }
                    let state = if receipt.applied(commit.command) {
                        NativePropertyState::Known(commit.value.clone())
                    } else {
                        let attempts = match native.properties.get(&commit.property) {
                            Some(NativePropertyState::Divergent { attempts }) => {
                                attempts.saturating_add(1)
                            }
                            _ => 1,
                        };
                        retries_exhausted |= attempts >= MAX_PROPERTY_ATTEMPTS;
                        NativePropertyState::Divergent { attempts }
                    };
                    native.properties.insert(commit.property, state);
                }
                Ok(retries_exhausted)
            }
        }
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
        candidate: Tree,
        candidate_root: NodeId,
        plan: UpdatePlan,
        changes: ComponentChanges,
        next_version: u64,
    ) -> Result<CommitReceipt, PumpError> {
        self.publish_candidate(
            CandidateState::Tree {
                tree: candidate,
                root: candidate_root,
            },
            plan,
            FrontendChanges::Component(changes),
            next_version,
        )
    }
}

impl<R: NativeRuntime> Drop for Pump<R> {
    fn drop(&mut self) {
        self.runtime.reset();
    }
}
