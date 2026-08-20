use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_WINDOW_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WindowId(u64);

impl WindowId {
    pub fn allocate() -> Self {
        let id = NEXT_WINDOW_ID.fetch_add(1, Ordering::Relaxed);
        assert_ne!(id, u64::MAX, "window id exhausted");
        Self(id)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WindowToken {
    id: WindowId,
    epoch: u64,
}

impl WindowToken {
    pub fn new(id: WindowId) -> Self {
        Self { id, epoch: 1 }
    }

    pub fn next(self) -> Option<Self> {
        Some(Self {
            id: self.id,
            epoch: self.epoch.checked_add(1)?,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeIdentity {
    window: WindowToken,
    realization_epoch: u64,
}

impl NativeIdentity {
    pub fn new(window: WindowToken) -> Self {
        Self {
            window,
            realization_epoch: 1,
        }
    }

    pub fn next_window(self) -> Option<Self> {
        Some(Self::new(self.window.next()?))
    }

    pub fn next_realization(self) -> Option<Self> {
        Some(Self {
            window: self.window,
            realization_epoch: self.realization_epoch.checked_add(1)?,
        })
    }

    pub fn window(self) -> WindowToken {
        self.window
    }

    pub fn realization_epoch(self) -> u64 {
        self.realization_epoch
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeWork<T> {
    pub identity: NativeIdentity,
    pub work: T,
}

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
    DispatcherRejected,
    HasChildren(NodeId),
    IndexOutOfBounds,
    Injected,
    MissingApplication,
    MissingNode(NodeId),
    MissingSubscription(NodeId, EventId),
    Native(i32),
    SelfParent(NodeId),
    SchedulerClosed,
    StillParented(NodeId),
    UnsupportedKind,
}

pub trait NativeRuntime {
    fn apply(&mut self, commands: &[Command]) -> CommitReceipt;
    fn reset(&mut self);

    fn set_identity(&mut self, _identity: NativeIdentity) {}

    fn drain_events(&mut self) -> Vec<NativeWork<QueuedEvent>> {
        Vec::new()
    }

    fn drain_event_errors(&mut self) -> Vec<NativeWork<QueuedEventError>> {
        Vec::new()
    }

    fn drain_realizations(&mut self) -> Vec<NativeWork<RealizationRequest>> {
        Vec::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RealizationRequest {
    Realize {
        collection: NodeId,
        container: RealizedContainer,
        index: usize,
    },
    Recycle {
        collection: NodeId,
        container: RealizedContainer,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RealizationOutcome {
    Realized(RealizationLease),
    Recycled(RealizationLease),
    Rejected(RealizationRequest),
}

pub struct QueuedEvent {
    pub node: NodeId,
    pub event: EventId,
    pub revision: u32,
    pub payload: EventPayload,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueuedEventError {
    pub node: NodeId,
    pub event: EventId,
    pub revision: u32,
    pub error: RuntimeError,
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
    CreateVirtualCollection {
        node: NodeId,
        item_count: usize,
    },
    ResetVirtualCollection {
        node: NodeId,
        item_count: usize,
    },
    AttachRealized {
        collection: NodeId,
        container: RealizedContainer,
        child: NodeId,
    },
    DetachRealized {
        collection: NodeId,
        container: RealizedContainer,
        child: NodeId,
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
