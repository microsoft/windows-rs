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
}

#[derive(Default)]
pub(super) struct ComponentChanges {
    pub(super) composed: HashSet<ComponentToken>,
    pub(super) context_reads: HashMap<ComponentToken, HashSet<ContextDependency>>,
    pub(super) deferred: HashSet<ComponentToken>,
    pub(super) retry: HashSet<ComponentToken>,
    pub(super) reserved: Vec<ComponentToken>,
    pub(super) retired: Vec<ComponentToken>,
    pub(super) touched: HashSet<ComponentToken>,
}

pub(super) enum LocalComponentUpdate {
    Plan(LocalCandidate),
    Fallback(ComponentRender),
    Unavailable,
}

pub(super) struct LocalCandidate {
    pub(super) context_reads: HashSet<ContextDependency>,
    pub(super) node: NodeId,
    pub(super) desired: MountedProps,
    pub(super) reference: Option<NativeElementRef>,
    pub(super) plan: UpdatePlan,
}

pub(super) enum CandidateState {
    Tree {
        tree: Tree,
        root: NodeId,
    },
    Native {
        node: NodeId,
        desired: MountedProps,
        reference: Option<NativeElementRef>,
    },
}

#[allow(
    clippy::large_enum_variant,
    reason = "boxing component changes would allocate on every component publication"
)]
pub(super) enum FrontendChanges {
    #[cfg(any(test, feature = "test"))]
    Element(Element),
    Component(ComponentChanges),
    Local {
        context_reads: HashSet<ContextDependency>,
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
        }
    }

    pub(super) fn push(&mut self, command: Command) -> usize {
        let index = self.commands.len();
        self.commands.push(command);
        index
    }

    pub(super) fn synchronize_children(&mut self, parent: NodeId, children: Vec<NodeId>) {
        if let Some(index) = self.commands.iter().position(
            |command| matches!(command, Command::SynchronizeChildren { parent: current, .. } if *current == parent),
        ) {
            self.commands.remove(index);
        }
        self.push(Command::SynchronizeChildren { parent, children });
    }
}
