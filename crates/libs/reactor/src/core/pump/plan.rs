use super::*;

pub(super) struct PropertyCommit {
    pub(super) node: NodeId,
    pub(super) property: PropertyId,
    pub(super) value: Option<PropertyValue>,
}

pub(super) struct ReferenceCommit {
    pub(super) node: NodeId,
    pub(super) old: Option<NativeElementRef>,
    pub(super) new: Option<NativeElementRef>,
}

pub(super) struct UpdatePlan {
    pub(super) commands: Vec<Command>,
    pub(super) commits: Vec<PropertyCommit>,
    pub(super) diagnostics: Vec<PumpDiagnostic>,
    pub(super) reference_commits: Vec<ReferenceCommit>,
    pub(super) identity: WindowToken,
    pub(super) reconcile_observations: bool,
    pub(super) post_publish_commands: Vec<Command>,
    pub(super) post_publish_windows: Vec<View>,
}

#[derive(Default)]
pub(super) struct ComponentChanges {
    pub(super) composed: IdSet<ComponentToken>,
    pub(super) context_reads: IdMap<ComponentToken, ContextDependencies>,
    pub(super) deferred: IdSet<ComponentToken>,
    pub(super) recompose: IdSet<ComponentToken>,
    pub(super) reserved: Vec<ComponentToken>,
    pub(super) retired: Vec<ComponentToken>,
    pub(super) touched: IdSet<ComponentToken>,
    pub(super) host_requests: Vec<HostRequest>,
}

#[derive(Clone, Copy)]
pub(super) enum PlanningFailure {
    Discard,
    Rearm,
}

pub(super) enum LocalComponentUpdate {
    Plan(LocalCandidate),
    Fallback(ComponentRender),
    Unavailable,
}

pub(super) struct LocalCandidate {
    pub(super) context_reads: ContextDependencies,
    pub(super) node: NodeId,
    pub(super) desired: MountedProps,
    pub(super) exit_transition: Option<ExitTransition>,
    pub(super) reference: Option<NativeElementRef>,
    pub(super) plan: UpdatePlan,
}

pub(super) struct ComponentCandidate {
    pub(super) activate_window: bool,
    pub(super) changes: ComponentChanges,
    pub(super) next_version: u64,
    pub(super) plan: UpdatePlan,
    pub(super) planning_failure: PlanningFailure,
    pub(super) root: NodeId,
    pub(super) tree: Tree,
    pub(super) window: NodeId,
}

pub(super) enum CandidateState {
    Tree {
        tree: Tree,
        root: NodeId,
    },
    Native {
        node: NodeId,
        desired: MountedProps,
        exit_transition: Option<ExitTransition>,
        reference: Option<NativeElementRef>,
    },
}

#[allow(
    clippy::large_enum_variant,
    reason = "boxing component changes would allocate on every component publication"
)]
pub(super) enum FrontendChanges {
    #[cfg(test)]
    Element(Element),
    Component(ComponentChanges),
    Local {
        context_reads: ContextDependencies,
        token: ComponentToken,
    },
}

impl UpdatePlan {
    pub(super) fn new(identity: WindowToken) -> Self {
        Self {
            commands: Vec::new(),
            commits: Vec::new(),
            diagnostics: Vec::new(),
            reference_commits: Vec::new(),
            identity,
            reconcile_observations: false,
            post_publish_commands: Vec::new(),
            post_publish_windows: Vec::new(),
        }
    }

    pub(super) fn push(&mut self, command: Command) -> usize {
        let index = self.commands.len();
        self.commands.push(command);
        index
    }

    pub(super) fn prepend(&mut self, command: Command) {
        self.commands.insert(0, command);
    }

    pub(super) fn synchronize_children(
        &mut self,
        parent: NodeId,
        slot: Option<SlotId>,
        children: Vec<NodeId>,
    ) {
        if let Some(index) = self.commands.iter().position(|command| {
            matches!(
                command,
                Command::SynchronizeChildren {
                    parent: current_parent,
                    slot: current_slot,
                    ..
                } if *current_parent == parent && *current_slot == slot
            )
        }) {
            self.commands.remove(index);
        }
        self.push(Command::SynchronizeChildren {
            parent,
            slot,
            children,
        });
    }
}
