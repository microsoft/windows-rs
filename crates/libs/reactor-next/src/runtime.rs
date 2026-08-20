use crate::arena::NodeId;
use crate::generated::{MountedKind, PropertyId, PropertyValue};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CommitReceipt {
    pub(crate) outcomes: Vec<CommandOutcome>,
}

impl CommitReceipt {
    pub(crate) fn applied(&self, index: usize) -> bool {
        self.outcomes.get(index) == Some(&CommandOutcome::Applied)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CommandOutcome {
    Applied,
    Failed(RuntimeError),
    Skipped,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RuntimeError {
    AlreadyParented(NodeId),
    ChildNotFound(NodeId),
    DuplicateNode(NodeId),
    HasChildren(NodeId),
    IndexOutOfBounds,
    Injected,
    MissingNode(NodeId),
    SelfParent(NodeId),
    StillParented(NodeId),
}

pub(crate) trait NativeRuntime {
    fn apply(&mut self, commands: &[Command]) -> CommitReceipt;
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Command {
    Create {
        node: NodeId,
        kind: MountedKind,
    },
    Destroy {
        node: NodeId,
    },
    SetProperty {
        node: NodeId,
        property: PropertyId,
        value: PropertyValue,
    },
    ClearProperty {
        node: NodeId,
        property: PropertyId,
    },
    InsertChild {
        parent: NodeId,
        child: NodeId,
        index: usize,
    },
    RemoveChild {
        parent: NodeId,
        child: NodeId,
    },
    MoveChild {
        parent: NodeId,
        child: NodeId,
        index: usize,
    },
}

impl Command {
    pub(crate) fn structural(&self) -> bool {
        !matches!(self, Self::SetProperty { .. } | Self::ClearProperty { .. })
    }
}
