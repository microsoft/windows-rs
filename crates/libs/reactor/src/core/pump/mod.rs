use crate::reference::{HostRequest, ImperativeEndpoint, ImperativeRequest, NativeElementRef};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

use super::*;

type IdMap<K, V> = FxHashMap<K, V>;
type IdSet<T> = FxHashSet<T>;

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
    NotMounted,
    DuplicateEffectKey(EffectKey),
    DuplicateElementRef,
    DuplicateKey(Key),
    DuplicateColorSchemeObservation,
    DuplicateWindowSizeObservation,
    DuplicateWindowTitle,
    DuplicateWindowTitleBar,
    DuplicateWindowVisuals,
    ExitTransitionUnsupported,
    EventReadFailed(RuntimeError),
    NativeApplyFailed(NativeApplyError),
    Poisoned,
    StructureUnsupported,
}

impl PumpError {
    pub(crate) fn is_declaration_rejection(&self) -> bool {
        matches!(
            self,
            Self::DuplicateEffectKey(_)
                | Self::DuplicateElementRef
                | Self::DuplicateKey(_)
                | Self::DuplicateColorSchemeObservation
                | Self::DuplicateWindowSizeObservation
                | Self::DuplicateWindowTitle
                | Self::DuplicateWindowTitleBar
                | Self::DuplicateWindowVisuals
                | Self::ExitTransitionUnsupported
                | Self::StructureUnsupported
        )
    }
}

impl From<ComponentDeclarationError> for PumpError {
    fn from(value: ComponentDeclarationError) -> Self {
        match value {
            ComponentDeclarationError::EffectKey(key) => Self::DuplicateEffectKey(key),
            ComponentDeclarationError::ColorSchemeObservation => {
                Self::DuplicateColorSchemeObservation
            }
            ComponentDeclarationError::WindowSizeObservation => {
                Self::DuplicateWindowSizeObservation
            }
            ComponentDeclarationError::WindowTitle => Self::DuplicateWindowTitle,
            ComponentDeclarationError::WindowVisuals => Self::DuplicateWindowVisuals,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PumpDiagnostic {
    WindowOpenRejected {
        error: RuntimeError,
    },
    VirtualRowRootCount {
        collection: NodeId,
        key: Key,
        actual: usize,
    },
}

pub struct Pump<R: NativeRuntime> {
    application: Option<NodeId>,
    components: ComponentStore,
    diagnostics: VecDeque<PumpDiagnostic>,
    dirty_components: IdSet<ComponentToken>,
    #[cfg(test)]
    element: Option<Element>,
    tree: Tree,
    runtime: R,
    root: Option<NodeId>,
    events: VecDeque<NativeWork<QueuedEvent>>,
    host_events: VecDeque<NativeWork<HostEvent>>,
    imperative: ImperativeEndpoint,
    identity: WindowToken,
    native_observation_pending: bool,
    last_native_observation: Option<(NodeId, EventId)>,
    planning_dirty: IdSet<ComponentToken>,
    poisoned: bool,
    realizations: VecDeque<NativeWork<RealizationRequest>>,
    reset_on_drop: bool,
    trace_component_plans: bool,
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
        if let Some(wake) = runtime.component_background_waker() {
            components.set_background_waker(wake);
        }
        let imperative = ImperativeEndpoint::new(runtime.component_waker());
        Self {
            application: None,
            components,
            diagnostics: VecDeque::new(),
            dirty_components: IdSet::default(),
            #[cfg(test)]
            element: None,
            tree: Tree::new(),
            runtime,
            root: None,
            events: VecDeque::new(),
            host_events: VecDeque::new(),
            imperative,
            identity,
            native_observation_pending: false,
            last_native_observation: None,
            planning_dirty: IdSet::default(),
            poisoned: false,
            realizations: VecDeque::new(),
            reset_on_drop: true,
            trace_component_plans: cfg!(debug_assertions)
                && std::env::var("WINDOWS_REACTOR_TRACE")
                    .is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true")),
            version: 0,
            window: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn mount(&mut self, element: Element) -> Result<(), PumpError> {
        if self.poisoned {
            return Err(PumpError::Poisoned);
        }

        if self.root.is_some() {
            return Err(PumpError::AlreadyMounted);
        }
        let next_version = self.next_version();
        let desired = element.clone();
        let mut candidate = Tree::new();
        let mut plan = UpdatePlan::new(self.identity);
        let application = candidate.insert(None, NodeKind::Application);
        plan.push(Command::CreateApplication { node: application });
        let window = candidate.insert(Some(application), NodeKind::Window);
        plan.push(Command::CreateWindow { node: window });
        let node =
            Self::mount_planned_element(&mut candidate, Some(window), None, element, &mut plan)?;
        plan.push(Command::InsertChild {
            parent: window,
            slot: None,
            child: node,
            index: 0,
        });
        Self::plan_window_title_bar(window, &self.tree, &candidate, &mut plan)?;
        plan.push(Command::ActivateWindow { node: window });

        self.publish_candidate(
            CandidateState::Tree {
                tree: candidate,
                root: node,
            },
            plan,
            FrontendChanges::Element(desired),
            next_version,
            PlanningFailure::Discard,
        )?;
        self.application = Some(application);
        self.window = Some(window);
        Ok(())
    }

    pub fn mount_view(&mut self, view: View) -> Result<(), PumpError> {
        if self.poisoned {
            return Err(PumpError::Poisoned);
        }
        if self.root.is_some() {
            return Err(PumpError::AlreadyMounted);
        }
        let next_version = self.next_version();
        let mut candidate = Tree::new();
        let mut plan = UpdatePlan::new(self.identity);
        let mut changes = ComponentChanges::default();
        let application = candidate.insert(None, NodeKind::Application);
        plan.push(Command::CreateApplication { node: application });
        let window = candidate.insert(Some(application), NodeKind::Window);
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
                self.fail_component_candidate(&changes, PlanningFailure::Discard);
                return Err(error);
            }
        };
        match native_roots.as_slice() {
            [] => {}
            [native_root] => {
                plan.push(Command::InsertChild {
                    parent: window,
                    slot: None,
                    child: *native_root,
                    index: 0,
                });
            }
            _ => {
                self.fail_component_candidate(&changes, PlanningFailure::Discard);
                return Err(PumpError::StructureUnsupported);
            }
        }
        self.finalize_component_candidate(ComponentCandidate {
            activate_window: true,
            changes,
            next_version,
            plan,
            planning_failure: PlanningFailure::Discard,
            root,
            tree: candidate,
            window,
        })?;
        self.application = Some(application);
        self.window = Some(window);
        Ok(())
    }

    #[cfg(any(test, feature = "test"))]
    pub fn update_view(&mut self, view: View) -> Result<(), PumpError> {
        if self.poisoned {
            return Err(PumpError::Poisoned);
        }
        let next_version = self.next_version();
        let root = self.root.ok_or(PumpError::NotMounted)?;
        let mut candidate = self.tree.clone();
        let mut plan = UpdatePlan {
            reconcile_observations: self.native_observation_pending,
            ..UpdatePlan::new(self.identity)
        };
        let mut changes = ComponentChanges {
            recompose: self.planning_dirty.clone(),
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
            self.fail_component_candidate(&changes, PlanningFailure::Rearm);
            return Err(error);
        }
        let window = self.window.ok_or(PumpError::NotMounted)?;
        let candidate_root = if let [candidate_root] = candidate.children(window) {
            *candidate_root
        } else {
            self.fail_component_candidate(&changes, PlanningFailure::Rearm);
            return Err(PumpError::StructureUnsupported);
        };
        self.apply_component_candidate(candidate, candidate_root, plan, changes, next_version)
    }

    #[cfg(test)]
    pub(crate) fn update(&mut self, element: Element) -> Result<(), PumpError> {
        if matches!(element.structure(), ElementStructureRef::Virtual(_)) {
            let desired_element = element.clone();
            self.update_view(View::native(element))?;
            self.element = Some(desired_element);
            return Ok(());
        }
        if self.poisoned {
            return Err(PumpError::Poisoned);
        }
        let next_version = self.next_version();
        if !self.native_observation_pending
            && self.element.as_ref() == Some(&element)
            && Self::node_matches_element(
                &self.tree,
                self.root.ok_or(PumpError::NotMounted)?,
                &element,
            )?
        {
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
        let window = self.window.ok_or(PumpError::NotMounted)?;
        Self::plan_window_title_bar(window, &self.tree, &candidate, &mut plan)?;
        self.publish_candidate(
            CandidateState::Tree {
                tree: candidate,
                root: candidate_root,
            },
            plan,
            FrontendChanges::Element(desired_element),
            next_version,
            PlanningFailure::Discard,
        )
    }

    pub fn runtime(&self) -> &R {
        &self.runtime
    }

    pub fn drain_diagnostics(&mut self) -> Vec<PumpDiagnostic> {
        self.diagnostics.drain(..).collect()
    }

    #[cfg(any(test, feature = "test"))]
    pub fn application(&self) -> Option<NodeId> {
        self.application
    }

    #[cfg(any(test, feature = "test"))]
    pub fn runtime_mut(&mut self) -> &mut R {
        &mut self.runtime
    }

    pub fn shutdown(&mut self) {
        let identity = self.identity.next();
        self.cleanup_component_effects();
        self.clear_published_references();
        self.imperative.complete_unavailable();
        self.runtime.reset();
        self.application = None;
        #[cfg(test)]
        {
            self.element = None;
        }
        self.dirty_components.clear();
        self.diagnostics.clear();
        self.events.clear();
        self.host_events.clear();
        self.realizations.clear();
        self.native_observation_pending = false;
        self.last_native_observation = None;
        self.planning_dirty.clear();
        self.root = None;
        self.tree = Tree::new();
        self.version = 0;
        self.window = None;
        self.identity = identity;
        self.runtime.set_identity(identity);
        self.imperative = ImperativeEndpoint::new(self.runtime.component_waker());
        let mut components = self.components.restarted(identity);
        if let Some(wake) = self.runtime.component_waker() {
            components.set_waker(wake);
        }
        if let Some(wake) = self.runtime.component_background_waker() {
            components.set_background_waker(wake);
        }
        self.components = components;
        self.poisoned = false;
    }

    fn cleanup_component_effects(&mut self) {
        let Some(root) = self.root else {
            return;
        };
        for node in self.tree.subtree_postorder(root) {
            if self.tree.kind(node) == NodeKind::Component {
                let scope = self.tree.component_scope(node);
                let token = self.components.token(scope);
                self.components.cleanup_effects(token);
            }
        }
    }

    pub(crate) fn native_window_closed(&mut self) {
        self.cleanup_component_effects();
        self.clear_published_references();
        self.imperative.complete_unavailable();
        self.components.close();
        self.runtime.native_window_closed();
        self.reset_on_drop = false;
    }

    #[cfg(any(test, feature = "test"))]
    pub fn root(&self) -> Option<NodeId> {
        self.root
    }

    #[cfg(feature = "test")]
    pub(crate) fn root_native(&self) -> Option<NodeId> {
        Self::native_root(&self.tree, self.root?).ok()
    }

    #[cfg(feature = "test")]
    pub(crate) fn live_native_children(&self, node: NodeId) -> &[NodeId] {
        self.tree.children(node)
    }

    #[cfg(any(test, feature = "test"))]
    pub fn version(&self) -> u64 {
        self.version
    }

    #[cfg(any(test, feature = "test"))]
    pub fn window(&self) -> Option<NodeId> {
        self.window
    }

    #[cfg(any(test, feature = "test"))]
    pub fn poisoned(&self) -> bool {
        self.poisoned
    }

    #[cfg(test)]
    pub(crate) fn components(&self) -> &ComponentStore {
        &self.components
    }

    #[cfg(test)]
    pub(crate) fn components_mut(&mut self) -> &mut ComponentStore {
        &mut self.components
    }

    fn commit_tree_properties(tree: &mut Tree, commits: &[PropertyCommit]) {
        for commit in commits {
            tree.native_mut(commit.node)
                .properties
                .insert(commit.property, commit.value.clone());
        }
    }

    fn commit_tree_references(tree: &mut Tree, commits: &[ReferenceCommit]) {
        for commit in commits {
            if let Some(native) = tree.try_native_mut(commit.node) {
                native.reference.clone_from(&commit.new);
            }
        }
    }

    fn commit_candidate_properties(
        &mut self,
        candidate: &mut CandidateState,
        commits: &[PropertyCommit],
    ) {
        match candidate {
            CandidateState::Tree { tree, .. } => Self::commit_tree_properties(tree, commits),
            CandidateState::Native { node, .. } => {
                let native = self.tree.native_mut(*node);
                for commit in commits {
                    assert_eq!(commit.node, *node);
                    native
                        .properties
                        .insert(commit.property, commit.value.clone());
                }
            }
        }
    }

    fn commit_candidate_references(
        &mut self,
        candidate: &mut CandidateState,
        commits: &[ReferenceCommit],
    ) {
        match candidate {
            CandidateState::Tree { tree, .. } => Self::commit_tree_references(tree, commits),
            CandidateState::Native {
                node, reference, ..
            } => {
                for commit in commits {
                    assert_eq!(commit.node, *node);
                }
                if let Some(commit) = commits.last() {
                    reference.clone_from(&commit.new);
                }
            }
        }
    }

    fn apply_reference_bindings(&self, commits: &[ReferenceCommit]) {
        for commit in commits {
            if let Some(old) = &commit.old {
                old.unbind(self.identity, commit.node);
            }
            if let Some(new) = &commit.new {
                new.bind(self.imperative.clone(), self.identity, commit.node);
            }
        }
    }

    fn validate_tree_references(
        &self,
        tree: &Tree,
        root: NodeId,
        commits: &[ReferenceCommit],
    ) -> Result<(), PumpError> {
        if !commits.iter().any(|commit| commit.new.is_some()) {
            return Ok(());
        }
        let mut references = HashSet::new();
        for node in tree.subtree_postorder(root) {
            let Some(native) = tree.try_native(node) else {
                continue;
            };
            let desired = commits
                .iter()
                .rev()
                .find(|commit| commit.node == node)
                .map_or(&native.reference, |commit| &commit.new);
            let Some(reference) = desired else {
                continue;
            };
            if !references.insert(reference.identity())
                || reference
                    .binding_target()
                    .is_some_and(|(identity, _)| identity != self.identity)
            {
                return Err(PumpError::DuplicateElementRef);
            }
        }
        Ok(())
    }

    fn validate_candidate_references(
        &self,
        candidate: &CandidateState,
        commits: &[ReferenceCommit],
    ) -> Result<(), PumpError> {
        if !commits.iter().any(|commit| commit.new.is_some()) {
            return Ok(());
        }
        match candidate {
            CandidateState::Tree { tree, root } => {
                self.validate_tree_references(tree, *root, commits)
            }
            CandidateState::Native {
                node, reference, ..
            } => {
                let Some(reference) = reference else {
                    return Ok(());
                };
                if reference
                    .binding_target()
                    .is_some_and(|(identity, _)| identity != self.identity)
                {
                    return Err(PumpError::DuplicateElementRef);
                }
                let Some(root) = self.root else {
                    return Ok(());
                };
                for current in self.tree.subtree_postorder(root) {
                    if current != *node
                        && self
                            .tree
                            .try_native(current)
                            .and_then(|native| native.reference.as_ref())
                            == Some(reference)
                    {
                        return Err(PumpError::DuplicateElementRef);
                    }
                }
                Ok(())
            }
        }
    }

    fn clear_published_references(&self) {
        let Some(root) = self.root else {
            return;
        };
        for node in self.tree.subtree_postorder(root) {
            if let Some(native) = self.tree.try_native(node)
                && let Some(reference) = &native.reference
            {
                reference.unbind(self.identity, node);
            }
        }
    }

    fn next_version(&self) -> u64 {
        self.version.checked_add(1).unwrap()
    }

    fn apply_component_candidate(
        &mut self,
        candidate: Tree,
        candidate_root: NodeId,
        plan: UpdatePlan,
        changes: ComponentChanges,
        next_version: u64,
    ) -> Result<(), PumpError> {
        let window = self.window.ok_or(PumpError::NotMounted)?;
        let resolved = changes
            .composed
            .iter()
            .chain(changes.retired.iter())
            .copied()
            .collect::<HashSet<_>>();
        self.finalize_component_candidate(ComponentCandidate {
            activate_window: false,
            changes,
            next_version,
            plan,
            planning_failure: PlanningFailure::Rearm,
            root: candidate_root,
            tree: candidate,
            window,
        })?;
        self.planning_dirty
            .retain(|token| !resolved.contains(token));
        Ok(())
    }

    fn finalize_component_candidate(
        &mut self,
        mut candidate: ComponentCandidate,
    ) -> Result<(), PumpError> {
        let planning = (|| {
            Self::plan_window_title_bar(
                candidate.window,
                &self.tree,
                &candidate.tree,
                &mut candidate.plan,
            )?;
            Self::plan_window_title(
                candidate.window,
                &self.tree,
                &candidate.tree,
                &mut candidate.plan,
            )?;
            Self::plan_window_visuals(
                candidate.window,
                &self.tree,
                &candidate.tree,
                &mut candidate.plan,
            )?;
            Self::plan_window_observations(
                candidate.window,
                &self.tree,
                &candidate.tree,
                &mut candidate.plan,
            )?;
            if candidate.activate_window {
                candidate.plan.push(Command::ActivateWindow {
                    node: candidate.window,
                });
            }
            Self::plan_host_requests(
                candidate.window,
                &mut candidate.changes.host_requests,
                &mut candidate.plan,
            );
            Ok::<(), PumpError>(())
        })();
        if let Err(error) = planning {
            self.fail_component_candidate(&candidate.changes, candidate.planning_failure);
            return Err(error);
        }
        self.publish_candidate(
            CandidateState::Tree {
                tree: candidate.tree,
                root: candidate.root,
            },
            candidate.plan,
            FrontendChanges::Component(candidate.changes),
            candidate.next_version,
            candidate.planning_failure,
        )
    }

    fn plan_host_requests(window: NodeId, requests: &mut Vec<HostRequest>, plan: &mut UpdatePlan) {
        let mut close = false;
        for request in requests.drain(..) {
            match request {
                HostRequest::CloseWindow { identity } if identity == plan.identity => close = true,
                HostRequest::OpenWindow { identity, root } if identity == plan.identity => {
                    plan.post_publish_windows.push(root);
                }
                HostRequest::CloseWindow { .. } | HostRequest::OpenWindow { .. } => {}
            }
        }
        if close {
            plan.post_publish_commands
                .push(Command::CloseWindow { node: window });
        }
    }

    fn plan_window_title(
        window: NodeId,
        current: &Tree,
        candidate: &Tree,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let current = current
            .validate_window_title()
            .map_err(|()| PumpError::DuplicateWindowTitle)?;
        let candidate = candidate
            .validate_window_title()
            .map_err(|()| PumpError::DuplicateWindowTitle)?;
        let current = current.as_ref().map(|title| title.title.as_ref());
        let candidate = candidate.as_ref().map(|title| title.title.as_ref());
        if current != candidate {
            plan.push(Command::SetWindowTitle {
                node: window,
                title: candidate.unwrap_or_default().to_string(),
            });
        }
        Ok(())
    }

    fn plan_window_visuals(
        window: NodeId,
        current: &Tree,
        candidate: &Tree,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let current = current
            .validate_window_visuals()
            .map_err(|()| PumpError::DuplicateWindowVisuals)?;
        let candidate = candidate
            .validate_window_visuals()
            .map_err(|()| PumpError::DuplicateWindowVisuals)?;
        let current = current.map(|state| state.visuals).unwrap_or_default();
        let candidate = candidate.map(|state| state.visuals).unwrap_or_default();
        if current != candidate {
            plan.push(Command::SetWindowVisuals {
                node: window,
                visuals: candidate,
            });
        }
        Ok(())
    }

    fn plan_window_observations(
        window: NodeId,
        current: &Tree,
        candidate: &Tree,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let current = WindowObservationFlags {
            window_size: current
                .validate_window_size_observation()
                .map_err(|()| PumpError::DuplicateWindowSizeObservation)?
                .map(|(observation, _)| observation),
            color_scheme: current
                .validate_color_scheme_observation()
                .map_err(|()| PumpError::DuplicateColorSchemeObservation)?
                .map(|(observation, _)| observation),
        };
        let candidate = WindowObservationFlags {
            window_size: candidate
                .validate_window_size_observation()
                .map_err(|()| PumpError::DuplicateWindowSizeObservation)?
                .map(|(observation, _)| observation),
            color_scheme: candidate
                .validate_color_scheme_observation()
                .map_err(|()| PumpError::DuplicateColorSchemeObservation)?
                .map(|(observation, _)| observation),
        };
        if current != candidate {
            plan.push(Command::SetWindowObservations {
                node: window,
                observations: candidate,
            });
        }
        Ok(())
    }

    fn plan_window_title_bar(
        window: NodeId,
        current: &Tree,
        candidate: &Tree,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let current_state = current
            .validate_window_title_bar()
            .map_err(|()| PumpError::DuplicateWindowTitleBar)?;
        let candidate_state = candidate
            .validate_window_title_bar()
            .map_err(|()| PumpError::DuplicateWindowTitleBar)?;

        if current_state == candidate_state {
            return Ok(());
        }

        if current_state.is_some_and(|current| {
            candidate_state.is_none_or(|candidate| candidate.title_bar != current.title_bar)
        }) {
            plan.prepend(Command::ClearWindowTitleBar { node: window });
        }
        if let Some(state) = candidate_state {
            plan.push(Command::SetWindowTitleBar {
                node: window,
                title_bar: state.title_bar,
                height: state.height,
            });
        }
        Ok(())
    }
}

impl<R: NativeRuntime> Drop for Pump<R> {
    fn drop(&mut self) {
        self.cleanup_component_effects();
        self.clear_published_references();
        self.imperative.clear();
        if self.reset_on_drop {
            self.runtime.reset();
        }
    }
}
