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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PumpError {
    AlreadyMounted,
    Component(ComponentStoreError),
    NotMounted,
    DuplicateKey(Key),
    EventReadFailed(RuntimeError),
    NativeApplyFailed(NativeApplyError),
    Poisoned,
    RenderBudgetExceeded,
    RevisionExhausted,
    StructureUnsupported,
    Tree(TreeError),
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

pub struct Pump<R: NativeRuntime> {
    application: Option<NodeId>,
    components: ComponentStore,
    dirty_components: HashSet<ComponentToken>,
    element: Option<Element>,
    tree: Tree,
    runtime: R,
    root: Option<NodeId>,
    events: VecDeque<NativeWork<QueuedEvent>>,
    identity: WindowToken,
    native_observation_pending: bool,
    planning_dirty: HashSet<ComponentToken>,
    poisoned: bool,
    realizations: VecDeque<NativeWork<RealizationRequest>>,
    reset_on_drop: bool,
    version: u64,
    window: Option<NodeId>,
}

impl<R: NativeRuntime> Pump<R> {
    pub fn new(mut runtime: R) -> Self {
        let identity = WindowToken::new(WindowId::allocate());
        runtime.set_identity(identity);
        let mut components = ComponentStore::new(identity);
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
            native_observation_pending: false,
            planning_dirty: HashSet::new(),
            poisoned: false,
            realizations: VecDeque::new(),
            reset_on_drop: true,
            version: 0,
            window: None,
        }
    }

    pub fn mount(&mut self, element: Element) -> Result<(), PumpError> {
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

        self.apply_native_commands(&plan.commands)?;
        Self::commit_tree_properties(&mut candidate, &plan.commits)?;
        self.tree = candidate;
        self.application = Some(application);
        self.element = Some(desired);
        self.root = Some(node);
        self.window = Some(window);
        self.version = next_version;
        Ok(())
    }

    pub fn mount_view(&mut self, view: View) -> Result<(), PumpError> {
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

        self.apply_native_commands(&plan.commands)?;

        Self::commit_tree_properties(&mut candidate, &plan.commits)?;
        self.finalize_component_changes(&changes)?;
        self.tree = candidate;
        self.application = Some(application);
        self.root = Some(root);
        self.window = Some(window);
        self.commit_component_effects(&changes)?;
        self.version = next_version;
        Ok(())
    }

    pub fn update_view(&mut self, view: View) -> Result<(), PumpError> {
        if self.poisoned {
            return Err(PumpError::Poisoned);
        }
        let next_version = self.next_version()?;
        let root = self.root.ok_or(PumpError::NotMounted)?;
        let mut candidate = self.tree.clone();
        let mut plan = UpdatePlan {
            reconcile_observations: self.native_observation_pending,
            ..UpdatePlan::new(self.identity)
        };
        let mut changes = ComponentChanges {
            retry: self.planning_dirty.clone(),
            ..ComponentChanges::default()
        };
        if let Err(error) = Self::reconcile_planned_view(
            &mut candidate,
            root,
            view,
            &mut self.components,
            &mut changes,
            &mut plan,
        ) {
            self.planning_dirty.extend(changes.touched.iter().copied());
            Self::remove_reservations(&mut self.components, &changes.reserved);
            return Err(error);
        }
        let window = self.window.ok_or(PumpError::NotMounted)?;
        let [candidate_root] = candidate.children(window)? else {
            self.planning_dirty.extend(changes.touched.iter().copied());
            Self::remove_reservations(&mut self.components, &changes.reserved);
            return Err(PumpError::StructureUnsupported);
        };
        let candidate_root = *candidate_root;
        self.apply_component_candidate(candidate, candidate_root, plan, changes, next_version)
    }

    pub fn update(&mut self, element: Element) -> Result<(), PumpError> {
        self.update_element(element, None)
    }

    pub(crate) fn update_with_hook_effects(
        &mut self,
        element: Element,
        effects: HookEffects,
    ) -> Result<(), PumpError> {
        self.update_element(element, Some(effects))
    }

    fn update_element(
        &mut self,
        element: Element,
        mut effects: Option<HookEffects>,
    ) -> Result<(), PumpError> {
        if self.poisoned {
            return Err(PumpError::Poisoned);
        }
        let next_version = self.next_version()?;
        if !self.native_observation_pending
            && self.element.as_ref() == Some(&element)
            && Self::node_matches_element(
                &self.tree,
                self.root.ok_or(PumpError::NotMounted)?,
                &element,
            )?
        {
            if let Some(effects) = effects.as_mut() {
                effects.prepare();
            }
            if let Some(effects) = effects {
                effects.commit();
            }
            self.version = next_version;
            return Ok(());
        }
        let node = self.root.ok_or(PumpError::NotMounted)?;
        let desired_element = element.clone();
        let mut candidate = self.tree.clone();
        let mut plan = UpdatePlan {
            reconcile_observations: self.native_observation_pending,
            ..UpdatePlan::new(self.identity)
        };
        let candidate_root = Self::reconcile_node(&mut candidate, node, element, &mut plan)?;
        let changes = match effects {
            Some(effects) => FrontendChanges::Hooks {
                element: desired_element,
                effects,
            },
            None => FrontendChanges::Element(desired_element),
        };
        self.publish_candidate(
            CandidateState::Tree {
                tree: candidate,
                root: candidate_root,
            },
            plan,
            changes,
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
        let identity = self.identity.next();
        self.cleanup_component_effects().unwrap();
        self.runtime.reset();
        self.application = None;
        self.element = None;
        self.dirty_components.clear();
        self.events.clear();
        self.realizations.clear();
        self.native_observation_pending = false;
        self.planning_dirty.clear();
        self.root = None;
        self.tree = Tree::new();
        self.version = 0;
        self.window = None;
        if let Some(identity) = identity {
            self.identity = identity;
            let mut components = ComponentStore::new(identity);
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

    fn cleanup_component_effects(&mut self) -> Result<(), PumpError> {
        let Some(root) = self.root else {
            return Ok(());
        };
        for node in self.tree.subtree_postorder(root)? {
            if self.tree.kind(node)? == NodeKind::Component {
                let scope = self.tree.component_scope(node)?;
                let token = self.components.token(scope)?;
                self.components.cleanup_effects(token)?;
            }
        }
        Ok(())
    }

    pub(crate) fn native_window_closed(&mut self) {
        self.cleanup_component_effects().unwrap();
        self.components.close();
        self.runtime.native_window_closed();
        self.reset_on_drop = false;
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

    pub(crate) fn components(&self) -> &ComponentStore {
        &self.components
    }

    pub(crate) fn components_mut(&mut self) -> &mut ComponentStore {
        &mut self.components
    }

    fn commit_tree_properties(
        tree: &mut Tree,
        commits: &[PropertyCommit],
    ) -> Result<(), PumpError> {
        for commit in commits {
            tree.native_mut(commit.node)?
                .properties
                .insert(commit.property, commit.value.clone());
        }
        Ok(())
    }

    fn commit_candidate_properties(
        &mut self,
        candidate: &mut CandidateState,
        commits: &[PropertyCommit],
    ) -> Result<(), PumpError> {
        match candidate {
            CandidateState::Tree { tree, .. } => Self::commit_tree_properties(tree, commits),
            CandidateState::Native { node, .. } => {
                let native = self.tree.native_mut(*node)?;
                for commit in commits {
                    if commit.node != *node {
                        return Err(PumpError::StructureUnsupported);
                    }
                    native
                        .properties
                        .insert(commit.property, commit.value.clone());
                }
                Ok(())
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
    ) -> Result<(), PumpError> {
        let resolved = changes
            .composed
            .iter()
            .chain(changes.retired.iter())
            .copied()
            .collect::<HashSet<_>>();
        self.publish_candidate(
            CandidateState::Tree {
                tree: candidate,
                root: candidate_root,
            },
            plan,
            FrontendChanges::Component(changes),
            next_version,
        )?;
        self.planning_dirty
            .retain(|token| !resolved.contains(token));
        Ok(())
    }
}

impl<R: NativeRuntime> Drop for Pump<R> {
    fn drop(&mut self) {
        _ = self.cleanup_component_effects();
        if self.reset_on_drop {
            self.runtime.reset();
        }
    }
}
