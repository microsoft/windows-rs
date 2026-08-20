use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommitReceipt {
    pub outcomes: Vec<CommandOutcome>,
}

impl CommitReceipt {
    pub fn applied(&self, index: usize) -> bool {
        self.outcomes.get(index) == Some(&CommandOutcome::Applied)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandOutcome {
    Applied,
    Failed(RuntimeError),
    Skipped,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeError {
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

pub trait NativeRuntime {
    fn apply(&mut self, commands: &[Command]) -> CommitReceipt;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
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
    pub fn structural(&self) -> bool {
        !matches!(self, Self::SetProperty { .. } | Self::ClearProperty { .. })
    }
}
