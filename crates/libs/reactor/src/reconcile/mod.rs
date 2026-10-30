use super::*;

mod attachments;
mod feedback;
mod graph;
mod plan;
mod relations;
mod retirement;
mod runtime;
mod virtualization;

pub use feedback::*;
use retirement::NativeChildren;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObjectId {
    index: u32,
    generation: u32,
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedMenu {
    menu: Menu,
    revision: u64,
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedCommandBarFlyout {
    flyout: CommandBarFlyout,
    revision: u64,
}

impl ObjectId {
    pub const fn index(self) -> u32 {
        self.index
    }

    pub const fn generation(self) -> u32 {
        self.generation
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Move {
    pub child: ObjectId,
    pub before: Option<ObjectId>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mutation {
    Create {
        object: ObjectId,
        kind: ObjectType,
    },
    Replace {
        object: ObjectId,
        kind: ObjectType,
    },
    SetProperties {
        object: ObjectId,
        set: Rc<[Property]>,
        clear: Rc<[PropertyId]>,
    },
    SetEvents {
        object: ObjectId,
        set: Rc<[Event]>,
        clear: Rc<[EventId]>,
    },
    ClearWindowTitleBar {
        object: ObjectId,
    },
    SetWindowTitleBar {
        object: ObjectId,
        height: WindowTitleBarHeight,
    },
    SetTooltip {
        target: ObjectId,
        tooltip: Option<ObjectId>,
        placement: TooltipPlacement,
    },
    SetFlyout {
        target: ObjectId,
        content: Option<ObjectId>,
        placement: FlyoutPlacement,
    },
    SetMenu {
        target: ObjectId,
        menu: Option<Menu>,
        revision: u64,
    },
    SetCommandBarFlyout {
        target: ObjectId,
        flyout: Option<CommandBarFlyout>,
        revision: u64,
    },
    SetContentDialog {
        owner: ObjectId,
        dialog: Option<ObjectId>,
    },
    SetContentDialogOpen {
        dialog: ObjectId,
        open: bool,
    },
    SetVirtualSource {
        object: ObjectId,
        item_count: usize,
        source_revision: u64,
    },
    Realize {
        parent: ObjectId,
        relation: RelationId,
        container: RealizedContainer,
        index: usize,
        child: ObjectId,
    },
    Recycle {
        parent: ObjectId,
        relation: RelationId,
        container: RealizedContainer,
        child: Option<ObjectId>,
    },
    Attach {
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
    },
    Detach {
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
    },
    Insert {
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
        index: usize,
    },
    Remove {
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
        index: usize,
    },
    Reorder {
        parent: ObjectId,
        relation: RelationId,
        moves: Vec<Move>,
        children: Vec<ObjectId>,
    },
    Retire {
        root: ObjectId,
        nodes: Vec<ObjectId>,
        parent: ObjectId,
        relation: RelationId,
        duration: Duration,
    },
    CompleteRetirement {
        root: ObjectId,
        nodes: Vec<ObjectId>,
    },
    Destroy {
        object: ObjectId,
    },
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UpdateStats {
    mutations: usize,
    #[cfg(test)]
    details: Vec<Mutation>,
}

impl UpdateStats {
    fn capture(mutations: &[Mutation]) -> Self {
        Self {
            mutations: mutations.len(),
            #[cfg(test)]
            details: mutations.to_vec(),
        }
    }
}

/// Mutations planned by one update and applied to the adapter as a unit.
#[derive(Default)]
#[must_use]
pub(crate) struct NativeBatch {
    mutations: Vec<Mutation>,
    references: Vec<ReferenceChange>,
}

impl NativeBatch {
    pub(crate) fn mutations(&self) -> &[Mutation] {
        &self.mutations
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetirementCompletion {
    pub root: ObjectId,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RealizedContainer(pub u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealizationLease {
    pub collection: ObjectId,
    pub container: RealizedContainer,
    pub key: Key,
    pub revision: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RealizationRequest {
    Realize {
        collection: ObjectId,
        container: RealizedContainer,
        index: usize,
        source_revision: u64,
    },
    Recycle {
        collection: ObjectId,
        container: RealizedContainer,
        source_revision: u64,
    },
    Cancel {
        collection: ObjectId,
        container: RealizedContainer,
        source_revision: u64,
    },
}

#[derive(Clone)]
pub(crate) enum VirtualWork {
    Realize {
        lease: RealizationLease,
        index: usize,
        view: Box<View>,
        owner: Option<ComponentId>,
    },
    Recycle {
        lease: RealizationLease,
    },
    Cancel {
        collection: ObjectId,
        relation: RelationId,
        container: RealizedContainer,
    },
}

pub(crate) enum NativeWork {
    Event(NativeEventDispatch),
    Virtual(VirtualWork),
    Maintenance,
}

/// Work reported by the native backend, in the order it occurred. `E` is the event occurrence:
/// an `EventDispatch` for the reconciler, or the backend's queued form of it.
#[derive(Clone, Debug, PartialEq)]
pub enum NativeEvent<E = EventDispatch> {
    /// Native state the user changed.
    Observation(Observation),
    /// Native state that Reactor's own write produced. For a property the declaration omits, the
    /// recorded value becomes the baseline that later notifications compare against.
    Record(Observation),
    /// An event occurrence and the native state it reports, if any.
    Event {
        observation: Option<Observation>,
        event: E,
    },
    Retirement(RetirementCompletion),
    Realization(RealizationRequest),
}

impl<E> NativeEvent<E> {
    /// An event occurrence with the state it reports, or the state alone.
    pub fn report(observation: Option<Observation>, event: Option<E>) -> Option<Self> {
        match (observation, event) {
            (observation, Some(event)) => Some(Self::Event { observation, event }),
            (Some(observation), None) => Some(Self::Observation(observation)),
            (None, None) => None,
        }
    }

    pub(crate) fn dispatches(&self) -> bool {
        matches!(self, Self::Event { .. })
    }

    pub(crate) fn observation(&self) -> Option<&Observation> {
        match self {
            Self::Observation(observation) | Self::Record(observation) => Some(observation),
            Self::Event { observation, .. } => observation.as_ref(),
            Self::Retirement(_) | Self::Realization(_) => None,
        }
    }

    pub(crate) fn event(&self) -> Option<&E> {
        match self {
            Self::Event { event, .. } => Some(event),
            _ => None,
        }
    }
}

pub struct NativeEventDispatch {
    event: Option<EventCall>,
    active: Rc<Cell<bool>>,
}

impl NativeEventDispatch {
    fn new(event: EventCall, active: Rc<Cell<bool>>) -> Self {
        Self {
            event: Some(event),
            active,
        }
    }

    pub fn invoke(&mut self) {
        if let Some(event) = self.event.take() {
            event.call();
        }
    }
}

impl Drop for NativeEventDispatch {
    fn drop(&mut self) {
        self.active.set(false);
    }
}

#[derive(Clone, Debug, PartialEq)]
enum RetainedRelationValue {
    One(Option<ObjectId>),
    Many(Vec<ObjectId>),
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedRelation {
    id: RelationId,
    value: RetainedRelationValue,
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedObject {
    kind: ObjectType,
    key: Option<Key>,
    reference: Option<ElementRef>,
    exit_transition: Option<ExitTransition>,
    window_title_bar: Option<WindowTitleBarHeight>,
    attachments: Option<Box<RetainedAttachments>>,
    properties: SharedList<Property>,
    /// The values observations are compared against where `properties` does not hold them: the
    /// declared value of a property whose native state has departed from it, and the native value
    /// recorded for a property the declaration omits.
    baselines: Vec<Property>,
    events: Option<Rc<Vec<Event>>>,
    relations: Vec<RetainedRelation>,
    virtual_items: Option<Box<RetainedVirtualItems>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct RetainedAttachments {
    tooltip: Option<RetainedTooltip>,
    flyout: Option<RetainedFlyout>,
    menu: Option<RetainedMenu>,
    command_bar_flyout: Option<RetainedCommandBarFlyout>,
    content_dialog: Option<RetainedContentDialog>,
}

impl RetainedAttachments {
    const EMPTY: Self = Self {
        tooltip: None,
        flyout: None,
        menu: None,
        command_bar_flyout: None,
        content_dialog: None,
    };
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedTooltip {
    object: ObjectId,
    placement: TooltipPlacement,
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedFlyout {
    object: ObjectId,
    placement: FlyoutPlacement,
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedContentDialog {
    object: ObjectId,
    open: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedVirtualItems {
    relation: RelationId,
    owner: Option<ComponentId>,
    items: VirtualItems,
    keys: Rc<Vec<Key>>,
    source_revision: u64,
    lease_revision: u64,
    active: HashMap<Key, (u64, RealizedContainer)>,
    containers: HashMap<RealizedContainer, RetainedRealization>,
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedRealization {
    key: Key,
    revision: u64,
    index: usize,
    child: Option<ObjectId>,
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedRetirement {
    nodes: Vec<ObjectId>,
    parent: ObjectId,
    relation: RelationId,
}

fn retained_events(events: &Option<Rc<Vec<Event>>>) -> &[Event] {
    events.as_deref().map_or(&[], Vec::as_slice)
}

fn retain_events(events: &SharedList<Event>) -> Option<Rc<Vec<Event>>> {
    (!events.as_slice().is_empty()).then(|| Rc::new(events.as_slice().to_vec()))
}

/// A new object's properties, except an index property that waits for the children it indexes.
fn properties_before_relations(kind: ObjectType, properties: &[Property]) -> Rc<[Property]> {
    let index = index_contract(kind).map(|contract| contract.property);
    properties
        .iter()
        .filter(|property| Some(property.id) != index)
        .cloned()
        .collect()
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RetainedGraph {
    root: Option<ObjectId>,
    objects: Vec<RetainedSlot>,
    free: Vec<u32>,
    retirements: HashMap<ObjectId, RetainedRetirement>,
    /// The native order of each relation with retiring children, which keep their native slots
    /// until their retirement completes. Other relations' native order is their active order.
    native_orders: HashMap<(ObjectId, RelationId), Vec<ObjectId>>,
    window_title_bar: Option<(ObjectId, WindowTitleBarHeight)>,
    #[cfg(test)]
    full_reference_scans: ReferenceScanCounter,
}

#[cfg(test)]
#[derive(Clone, Debug, Default)]
struct ReferenceScanCounter(Cell<usize>);

#[derive(Clone, Debug, Default, PartialEq)]
struct RetainedSlot {
    generation: u32,
    object: Option<RetainedObject>,
    retiring: bool,
}

/// Backend contract for applying retained mutations and publishing ordered native work.
///
/// `peek_native_event` returns the event that the next `pop_native_event` call removes, without
/// consuming it. A backend that cannot apply a mutation batch panics: native mutations cannot be
/// rolled back, so a partially applied batch leaves no state worth continuing from.
pub trait Adapter {
    /// Returns the next native event without consuming it.
    fn peek_native_event(&self) -> Option<NativeEvent> {
        None
    }

    /// Removes the next native event.
    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        None
    }

    /// Returns the first queued native event that carries a dispatch, without consuming any.
    /// Reactor uses it to coalesce repeated state reports across the observations and
    /// realizations queued between them; the default sees only the next event.
    fn peek_native_dispatch(&self) -> Option<NativeEvent> {
        self.peek_native_event().filter(NativeEvent::dispatches)
    }

    /// Applies a mutation batch in order.
    fn apply(&mut self, mutations: &[Mutation]);

    /// Applies an imperative request or completes it as unavailable.
    fn imperative(&mut self, request: ImperativeRequest) {
        request.complete_unavailable();
    }
}

/// Owns retained state and the adapter protocol.
///
/// Mutable adapter access is intentionally unavailable so callers cannot bypass reconciliation.
///
/// ```compile_fail
/// use windows_reactor::{RecordingAdapter, Runtime};
///
/// let mut runtime = Runtime::new(RecordingAdapter::new());
/// let _: &mut RecordingAdapter = runtime.adapter_mut();
/// ```
pub struct Runtime<A> {
    graph: RetainedGraph,
    adapter: A,
    mutations: Vec<Mutation>,
    virtual_refreshes: VecDeque<RealizationRequest>,
    validator: DeclarationValidator,
    native_event_active: Rc<Cell<bool>>,
    references: ReferenceEndpoint,
}

impl<A> Drop for Runtime<A> {
    fn drop(&mut self) {
        self.graph.clear_references(&self.references);
        self.references.clear();
    }
}

struct Planner<'a> {
    retained: &'a mut RetainedGraph,
    mutations: &'a mut Vec<Mutation>,
    references: &'a mut Vec<ReferenceChange>,
    virtual_refreshes: Option<&'a mut Vec<RealizationRequest>>,
}

enum ReferenceChange {
    Clear {
        reference: ElementRef,
        object: ObjectId,
    },
    Set {
        reference: ElementRef,
        object: ObjectId,
    },
}

fn apply_reference_changes(endpoint: &ReferenceEndpoint, changes: Vec<ReferenceChange>) {
    for change in changes {
        match change {
            ReferenceChange::Clear { reference, object } => reference.clear(endpoint, object),
            ReferenceChange::Set { reference, object } => reference.bind(endpoint.clone(), object),
        }
    }
}

fn defer_window_title_bar_sets(mutations: &mut Vec<Mutation>) {
    let (deferred, immediate): (Vec<_>, Vec<_>) = std::mem::take(mutations)
        .into_iter()
        .partition(|mutation| matches!(mutation, Mutation::SetWindowTitleBar { .. }));
    *mutations = immediate;
    mutations.extend(deferred);
}

#[cfg(any(test, feature = "test"))]
#[path = "../test_support/reconcile.rs"]
mod test_support;
#[cfg(any(test, feature = "test"))]
pub use test_support::*;
