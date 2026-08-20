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
    DuplicateEvent(NodeId, EventId),
    DuplicateNode(NodeId),
    HasChildren(NodeId),
    IndexOutOfBounds,
    Injected,
    MissingNode(NodeId),
    MissingSubscription(NodeId, EventId),
    Native(i32),
    SelfParent(NodeId),
    StillParented(NodeId),
    UnsupportedKind,
}

pub trait NativeRuntime {
    fn apply(&mut self, commands: &[Command]) -> CommitReceipt;
    fn reset(&mut self);

    fn drain_events(&mut self) -> Vec<QueuedEvent> {
        Vec::new()
    }

    fn drain_event_errors(&mut self) -> Vec<RuntimeError> {
        Vec::new()
    }
}

pub struct QueuedEvent {
    pub node: NodeId,
    pub event: EventId,
    pub revision: u32,
    pub payload: EventPayload,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    CreateApplication {
        node: NodeId,
    },
    CreateWindow {
        node: NodeId,
    },
    ActivateWindow {
        node: NodeId,
    },
    ResetWindowContent {
        window: NodeId,
    },
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
    SubscribeEvent {
        node: NodeId,
        event: EventId,
        revision: u32,
    },
    UnsubscribeEvent {
        node: NodeId,
        event: EventId,
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
