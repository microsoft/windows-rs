use super::*;
use std::rc::Rc;
use std::sync::Arc;
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeWork<T> {
    pub identity: WindowToken,
    pub work: T,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeApplyError {
    pub command: usize,
    pub error: RuntimeError,
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
    fn apply(&mut self, commands: &[Command]) -> Result<(), NativeApplyError>;
    fn reset(&mut self);

    fn native_window_closed(&mut self) {}

    fn set_identity(&mut self, _identity: WindowToken) {}

    fn component_waker(&self) -> Option<Rc<dyn Fn()>> {
        None
    }

    fn component_background_waker(&self) -> Option<Arc<dyn Fn() -> bool + Send + Sync>> {
        None
    }

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
        source_revision: u64,
    },
    Recycle {
        collection: NodeId,
        container: RealizedContainer,
        source_revision: u64,
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
    invoke_callback: bool,
}

impl QueuedEvent {
    pub fn new(node: NodeId, event: EventId, revision: u32, payload: EventPayload) -> Self {
        Self {
            node,
            event,
            revision,
            payload,
            invoke_callback: true,
        }
    }

    pub(crate) fn observation(
        node: NodeId,
        event: EventId,
        revision: u32,
        payload: EventPayload,
    ) -> Self {
        Self {
            node,
            event,
            revision,
            payload,
            invoke_callback: false,
        }
    }

    pub(crate) fn invokes_callback(&self) -> bool {
        self.invoke_callback
    }
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
    CloseWindow {
        node: NodeId,
    },
    Create {
        node: NodeId,
        kind: MountedKind,
    },
    CreateVirtualCollection {
        node: NodeId,
        item_count: usize,
        source_revision: u64,
    },
    ResetVirtualCollection {
        node: NodeId,
        item_count: usize,
        source_revision: u64,
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
    Focus {
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
    SetSlot {
        parent: NodeId,
        slot: SlotId,
        child: Option<NodeId>,
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
    ResetChildren {
        parent: NodeId,
    },
    SynchronizeChildren {
        parent: NodeId,
        children: Vec<NodeId>,
    },
    MoveChild {
        parent: NodeId,
        child: NodeId,
        index: usize,
    },
}
