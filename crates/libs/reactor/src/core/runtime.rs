use super::scope::ScopeId;
use super::*;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_WINDOW_ID: AtomicU64 = AtomicU64::new(1);

/// Theme-resource selections recorded by the test runtime.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ThemeStyle {
    values: [Option<ThemeBrush>; 4],
}

impl ThemeStyle {
    pub(crate) const fn new(values: [Option<ThemeBrush>; 4]) -> Self {
        Self { values }
    }

    pub(crate) fn is_empty(self) -> bool {
        self.values.iter().all(Option::is_none)
    }

    pub(crate) fn values(self) -> [Option<ThemeBrush>; 4] {
        self.values
    }
}

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

    pub fn next(self) -> Self {
        Self {
            id: self.id,
            epoch: self.epoch.checked_add(1).unwrap(),
        }
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
    WindowOpenCapacity,
}

pub trait NativeRuntime {
    fn apply(&mut self, commands: &[Command]) -> Result<(), NativeApplyError>;
    fn reset(&mut self);

    fn open_windows(&mut self, _roots: Vec<View>) -> Result<(), RuntimeError> {
        Err(RuntimeError::UnsupportedKind)
    }

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

    fn drain_host_events(&mut self) -> Vec<NativeWork<HostEvent>> {
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
pub enum HostEvent {
    WindowSize {
        observation: HostObservationId,
        size: WindowSize,
    },
    ColorScheme {
        observation: HostObservationId,
        scheme: ColorScheme,
    },
    ObservationError {
        observation: HostObservationId,
        error: RuntimeError,
    },
    Error(RuntimeError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostObservationId {
    pub owner: ScopeId,
    pub revision: u32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowObservationFlags {
    pub window_size: Option<HostObservationId>,
    pub color_scheme: Option<HostObservationId>,
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
    SetWindowTitle {
        node: NodeId,
        title: String,
    },
    ClearWindowTitleBar {
        node: NodeId,
    },
    SetWindowTitleBar {
        node: NodeId,
        title_bar: NodeId,
        height: WindowTitleBarHeight,
    },
    SetWindowVisuals {
        node: NodeId,
        visuals: WindowVisuals,
    },
    SetWindowObservations {
        node: NodeId,
        observations: WindowObservationFlags,
    },
    SetThemeStyle {
        node: NodeId,
        style: ThemeStyle,
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
    AcknowledgeRecycle {
        collection: NodeId,
        container: RealizedContainer,
    },
    Destroy {
        node: NodeId,
    },
    RetireSubtree {
        root: NodeId,
        nodes: Vec<NodeId>,
        parent: NodeId,
        slot: Option<SlotId>,
        transition: ExitTransition,
    },
    Focus {
        node: NodeId,
        completion: Callback<Result<bool, RuntimeError>>,
    },
    InitializeWebView2 {
        node: NodeId,
        completion: Callback<Result<windows_core::IUnknown, RuntimeError>>,
    },
    ObserveSwapChainPanel {
        node: NodeId,
        observation: u64,
        callback: Callback<SwapChainPanelEvent>,
    },
    SetSwapChain {
        node: NodeId,
        swap_chain: Option<windows_core::IUnknown>,
        completion: Callback<Result<(), RuntimeError>>,
    },
    SetNativeImageSource {
        node: NodeId,
        source: Option<windows_core::IUnknown>,
        completion: Callback<Result<(), RuntimeError>>,
    },
    ObserveImageScale {
        node: NodeId,
        observation: u64,
        callback: Callback<f64>,
    },
    ObserveCompositionHost {
        node: NodeId,
        observation: u64,
        callback: Callback<CompositionHostEvent>,
    },
    RevokeObservation {
        node: NodeId,
        observation: u64,
    },
    SetCompositionChildVisual {
        node: NodeId,
        visual: Option<windows_core::IUnknown>,
        completion: Callback<Result<(), RuntimeError>>,
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
    SetTooltip {
        target: NodeId,
        tooltip: Option<NodeId>,
        placement: TooltipPlacement,
    },
    SetFlyout {
        target: NodeId,
        content: Option<NodeId>,
        placement: FlyoutPlacement,
    },
    SetOwnedMenu {
        owner: NodeId,
        target: NodeId,
        kind: OwnedMenuKind,
        items: Option<Vec<MenuItem>>,
        revision: u32,
    },
    SetCommandBarFlyout {
        owner: NodeId,
        target: NodeId,
        primary: Option<Vec<CommandBarCommand>>,
        secondary: Vec<CommandBarCommand>,
        revision: u32,
    },
    SetTreeViewNodes {
        target: NodeId,
        nodes: Vec<TreeNode>,
    },
    SetContentDialogOpen {
        node: NodeId,
        owner: NodeId,
        open: bool,
    },
    InsertChild {
        parent: NodeId,
        slot: Option<SlotId>,
        child: NodeId,
        index: usize,
    },
    RemoveChild {
        parent: NodeId,
        slot: Option<SlotId>,
        child: NodeId,
    },
    SynchronizeChildren {
        parent: NodeId,
        slot: Option<SlotId>,
        children: Vec<NodeId>,
    },
    MoveChild {
        parent: NodeId,
        slot: Option<SlotId>,
        child: NodeId,
        index: usize,
    },
}

impl Command {
    pub(crate) fn complete_unavailable(&self) {
        match self {
            Self::Focus { node, completion } => {
                _ = completion.call(Err(RuntimeError::MissingNode(*node)));
            }
            Self::InitializeWebView2 { node, completion } => {
                _ = completion.call(Err(RuntimeError::MissingNode(*node)));
            }
            Self::SetSwapChain {
                node, completion, ..
            }
            | Self::SetNativeImageSource {
                node, completion, ..
            }
            | Self::SetCompositionChildVisual {
                node, completion, ..
            } => {
                _ = completion.call(Err(RuntimeError::MissingNode(*node)));
            }
            _ => {}
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OwnedMenuKind {
    ButtonFlyout,
    DropDownButtonFlyout,
    MenuBarItem,
}
