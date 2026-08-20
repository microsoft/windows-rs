use super::*;

/// Maximum number of native commands applied in one recovery continuation.
pub(super) const RECOVERY_COMMAND_BUDGET: usize = 64;

pub(super) struct PropertyCommit {
    pub(super) command: usize,
    pub(super) node: NodeId,
    pub(super) property: PropertyId,
    pub(super) value: Option<PropertyValue>,
}

pub(super) struct UpdatePlan {
    pub(super) commands: Vec<Command>,
    pub(super) commits: Vec<PropertyCommit>,
    pub(super) identity: NativeIdentity,
    pub(super) retry_properties: bool,
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
    pub(super) fn new(identity: NativeIdentity) -> Self {
        Self {
            commands: Vec::new(),
            commits: Vec::new(),
            identity,
            retry_properties: false,
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

/// In-flight recovery continuation owned by [`Pump`].
///
/// When a structural recovery plan exceeds [`RECOVERY_COMMAND_BUDGET`], the
/// pump stores this continuation and resumes it on subsequent scheduler turns
/// before processing ordinary events, messages, or reconciliation.
pub(super) struct PendingRecovery {
    pub(super) candidate: CandidateState,
    pub(super) changes: FrontendChanges,
    pub(super) plan: UpdatePlan,
    pub(super) failure: CommitReceipt,
    pub(super) next_version: u64,
    pub(super) commands_applied: usize,
    pub(super) outcomes: Vec<CommandOutcome>,
    pub(super) recovery_root: NodeId,
}

impl PendingRecovery {
    pub(super) fn remaining_commands(&self) -> &[Command] {
        &self.plan.commands[self.commands_applied..]
    }

    pub(super) fn is_complete(&self) -> bool {
        self.commands_applied >= self.plan.commands.len()
    }
}
