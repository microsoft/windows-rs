use super::*;

pub(super) struct PropertyCommit {
    pub(super) node: NodeId,
    pub(super) property: PropertyId,
    pub(super) value: Option<PropertyValue>,
}

pub(super) struct UpdatePlan {
    pub(super) commands: Vec<Command>,
    pub(super) commits: Vec<PropertyCommit>,
    pub(super) identity: WindowToken,
    pub(super) reconcile_observations: bool,
}

#[derive(Default)]
pub(super) struct ComponentChanges {
    pub(super) composed: HashSet<ComponentToken>,
    pub(super) deferred: HashSet<ComponentToken>,
    pub(super) reserved: Vec<ComponentToken>,
    pub(super) retired: Vec<ComponentToken>,
}

pub(super) enum LocalComponentUpdate {
    Plan(LocalCandidate),
    Fallback(View),
    Unavailable,
}

pub(super) struct LocalCandidate {
    pub(super) node: NodeId,
    pub(super) desired: MountedProps,
    pub(super) plan: UpdatePlan,
}

pub(super) enum CandidateState {
    Tree { tree: Tree, root: NodeId },
    Native { node: NodeId, desired: MountedProps },
}

pub(super) enum FrontendChanges {
    Element(Element),
    Component(ComponentChanges),
    Local(ComponentToken),
}

impl UpdatePlan {
    pub(super) fn new(identity: WindowToken) -> Self {
        Self {
            commands: Vec::new(),
            commits: Vec::new(),
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
