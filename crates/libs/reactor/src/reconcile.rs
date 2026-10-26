use super::*;

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

    pub const fn len(&self) -> usize {
        self.mutations
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

#[derive(Clone, Debug, PartialEq)]
pub enum Observation {
    SetProperty {
        object: ObjectId,
        property: Property,
    },
    SetSelection {
        object: ObjectId,
        selected: ObservedSelection,
    },
    /// The native control reordered a relation's children, listed in native order.
    Reorder {
        object: ObjectId,
        relation: RelationId,
        children: Vec<ObjectId>,
    },
}

/// The item a native selector reports as selected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservedSelection {
    Cleared,
    Item(ObjectId),
    /// A selected item outside the owner's selection relations, such as a nested
    /// `NavigationViewItem` or the built-in settings item.
    Untracked,
}

impl ObservedSelection {
    fn item(self) -> Option<ObjectId> {
        match self {
            Self::Item(item) => Some(item),
            Self::Cleared | Self::Untracked => None,
        }
    }
}

impl Observation {
    pub(crate) fn object(&self) -> ObjectId {
        match self {
            Self::SetProperty { object, .. }
            | Self::SetSelection { object, .. }
            | Self::Reorder { object, .. } => *object,
        }
    }
}

/// The controlled event that reports a property, directly or through the property it coerces.
pub(crate) fn reporting_event(kind: ObjectType, property: PropertyId) -> Option<EventId> {
    controlled_event(kind, property).or_else(|| {
        coerced_property(kind, property).and_then(|coerced| controlled_event(kind, coerced))
    })
}

/// Native notifications dropped while Reactor writes the state they report.
#[derive(Default)]
pub(crate) struct FeedbackState {
    /// Depth of the nested writes suppressing each event.
    suppressed: HashMap<(ObjectId, EventId), usize>,
}

impl FeedbackState {
    /// Returns whether an event should be reported rather than dropped.
    pub(crate) fn reports(&self, object: ObjectId, event: EventId) -> bool {
        !self.suppressed.contains_key(&(object, event))
    }

    pub(crate) fn suppress(&mut self, object: ObjectId, event: EventId) {
        *self.suppressed.entry((object, event)).or_default() += 1;
    }

    pub(crate) fn release(&mut self, object: ObjectId, event: EventId) {
        let key = (object, event);
        let depth = self.suppressed.get_mut(&key).unwrap();
        *depth -= 1;
        if *depth == 0 {
            self.suppressed.remove(&key);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EventDispatch {
    pub object: ObjectId,
    pub event: EventId,
    callback: EventValue,
    payload: EventPayload,
}

impl EventDispatch {
    pub fn new(
        object: ObjectId,
        event: EventId,
        callback: EventValue,
        payload: EventPayload,
    ) -> Self {
        Self {
            object,
            event,
            callback,
            payload,
        }
    }

    pub fn invoke(self) {
        match (self.callback, self.payload) {
            (EventValue::Bool(callback), EventPayload::Bool(value)) => callback.call(value),
            (EventValue::Color(callback), EventPayload::Color(value)) => callback.call(value),
            #[cfg(any(test, feature = "test"))]
            (EventValue::CharacterEventInfo(callback), EventPayload::CharacterEventInfo(value)) => {
                callback.call(value);
            }
            (
                EventValue::ContentDialogResult(callback),
                EventPayload::ContentDialogResult(value),
            ) => callback.call(value),
            (EventValue::String(callback), EventPayload::String(value)) => callback.call(value),
            (EventValue::F64(callback), EventPayload::F64(value)) => callback.call(value),
            (EventValue::FocusEventInfo(callback), EventPayload::FocusEventInfo(value)) => {
                callback.call(value);
            }
            (EventValue::DragKind(callback), EventPayload::DragKind(value)) => callback.call(value),
            (EventValue::DroppedData(callback), EventPayload::DroppedData(value)) => {
                callback.call(value);
            }
            (EventValue::OptionalBool(callback), EventPayload::OptionalBool(value)) => {
                callback.call(value);
            }
            (EventValue::OptionalDateTime(callback), EventPayload::OptionalDateTime(value)) => {
                callback.call(value);
            }
            (EventValue::OptionalF64(callback), EventPayload::OptionalF64(value)) => {
                callback.call(value);
            }
            (EventValue::OptionalTimeSpan(callback), EventPayload::OptionalTimeSpan(value)) => {
                callback.call(value);
            }
            (
                EventValue::NavigationViewDisplayMode(callback),
                EventPayload::NavigationViewDisplayMode(value),
            ) => callback.call(value),
            (EventValue::Selection(callback), EventPayload::Selection(value)) => {
                callback.call(value.value);
            }
            (EventValue::PointerEventInfo(callback), EventPayload::PointerEventInfo(value)) => {
                callback.call(value);
            }
            #[cfg(any(test, feature = "test"))]
            (EventValue::KeyEventInfo(callback), EventPayload::KeyEventInfo(value)) => {
                callback.call(value);
            }
            (EventValue::Key(callback), EventPayload::Key(value)) => callback.call(value),
            (EventValue::Keys(callback), EventPayload::Keys(value)) => callback.call(value),
            (EventValue::SelectionIndex(callback), EventPayload::SelectionIndex(value)) => {
                callback.call(value);
            }
            (EventValue::Unit(callback), EventPayload::Unit) => callback.call(()),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NativeEvent {
    observation: Option<Observation>,
    event: Option<EventDispatch>,
    record: bool,
    retirement: Option<RetirementCompletion>,
    realization: Option<RealizationRequest>,
}

impl NativeEvent {
    pub fn new(observation: Option<Observation>, event: Option<EventDispatch>) -> Self {
        Self {
            observation,
            event,
            record: false,
            retirement: None,
            realization: None,
        }
    }

    /// Records native state that Reactor's own write produced. For a property the declaration
    /// omits, the recorded value becomes the baseline that later notifications compare against.
    pub fn record(observation: Observation) -> Self {
        Self {
            record: true,
            ..Self::new(Some(observation), None)
        }
    }

    pub fn retirement(completion: RetirementCompletion) -> Self {
        Self {
            retirement: Some(completion),
            ..Self::new(None, None)
        }
    }

    pub fn realization(request: RealizationRequest) -> Self {
        Self {
            realization: Some(request),
            ..Self::new(None, None)
        }
    }

    pub(crate) fn dispatches(&self) -> bool {
        self.event.is_some()
    }
}

pub struct NativeEventDispatch {
    event: Option<EventDispatch>,
    active: Rc<Cell<bool>>,
}

impl NativeEventDispatch {
    fn new(event: EventDispatch, active: Rc<Cell<bool>>) -> Self {
        Self {
            event: Some(event),
            active,
        }
    }

    pub fn invoke(&mut self) {
        if let Some(event) = self.event.take() {
            event.invoke();
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

fn virtual_relation(kind: ObjectType) -> Option<RelationId> {
    relation_contracts(kind)
        .iter()
        .find(|contract| {
            contract.realization == Realization::Container
                && contract.child == ObjectCategory::Visual
        })
        .map(|contract| contract.id)
}

fn virtual_keys(items: &VirtualItems) -> Vec<Key> {
    let mut keys = Vec::with_capacity(items.len());
    let mut unique = HashSet::with_capacity(items.len());
    for index in 0..items.len() {
        let key = items.key(index).unwrap();
        assert!(unique.insert(key.clone()), "duplicate key {key:?}");
        keys.push(key);
    }
    keys
}

fn retain_virtual_items(declaration: &Declaration) -> Option<Box<RetainedVirtualItems>> {
    let relation = virtual_relation(declaration.kind)?;
    let declared = declaration
        .virtual_items
        .as_deref()
        .cloned()
        .unwrap_or(DeclaredVirtualItems {
            relation,
            owner: declaration.component,
            items: VirtualItems::Eager(Rc::default()),
        });
    assert!(
        declared.relation == relation,
        "invalid relation {:?} for {:?}",
        declared.relation,
        declaration.kind
    );
    let keys = virtual_keys(&declared.items);
    Some(Box::new(RetainedVirtualItems {
        relation,
        owner: declared.owner.or(declaration.component),
        items: declared.items,
        keys: Rc::new(keys),
        source_revision: 0,
        lease_revision: 0,
        active: HashMap::new(),
        containers: HashMap::new(),
    }))
}

#[derive(Clone, Debug, PartialEq)]
struct RetainedRetirement {
    nodes: Vec<ObjectId>,
    parent: ObjectId,
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

impl RetainedGraph {
    pub(crate) fn tooltip(&self, target: ObjectId) -> Option<ObjectId> {
        self.get(target)?
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.tooltip.as_ref())
            .map(|tooltip| tooltip.object)
    }

    pub fn flyout(&self, target: ObjectId) -> Option<(ObjectId, FlyoutPlacement)> {
        self.get(target)?
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.flyout.as_ref())
            .map(|flyout| (flyout.object, flyout.placement))
    }

    pub fn content_dialog(&self, owner: ObjectId) -> Option<(ObjectId, bool)> {
        self.get(owner)?
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.content_dialog.as_ref())
            .map(|dialog| (dialog.object, dialog.open))
    }

    fn clear_references(&self, endpoint: &ReferenceEndpoint) {
        #[cfg(test)]
        self.full_reference_scans
            .0
            .set(self.full_reference_scans.0.get() + 1);
        for (index, slot) in self.objects.iter().enumerate() {
            let Some(object) = slot.object.as_ref() else {
                continue;
            };
            if let Some(reference) = object.reference.as_ref()
                && let Ok(index) = u32::try_from(index)
            {
                reference.clear(
                    endpoint,
                    ObjectId {
                        index,
                        generation: slot.generation,
                    },
                );
            }
        }
    }

    pub fn root(&self) -> Option<ObjectId> {
        self.root
    }

    fn validate_observation(&self, observation: &Observation) -> bool {
        match observation {
            Observation::SetProperty { object, property } => {
                let Some(retained) = self.get(*object) else {
                    return false;
                };
                validate_property(retained.kind, property);
                true
            }
            Observation::SetSelection { object, selected } => {
                let Some(owner) = self.get(*object) else {
                    return false;
                };
                let Some(contract) = selection_contract(owner.kind) else {
                    panic!("{:?} does not support selection", owner.kind);
                };
                selected.item().is_none_or(|selected| {
                    self.kind(selected) == Some(contract.item)
                        && contract.relations.iter().any(|relation| {
                            self.children(*object, *relation)
                                .is_some_and(|children| children.contains(&selected))
                        })
                })
            }
            Observation::Reorder {
                object,
                relation,
                children,
            } => {
                let Some(owner) = self.get(*object) else {
                    return false;
                };
                assert!(
                    reorder_event(owner.kind, *relation).is_some(),
                    "invalid relation {relation:?} for {:?}",
                    owner.kind
                );
                self.reordered_children(*object, *relation, children)
                    .is_some()
            }
        }
    }

    /// Returns the live children in native order when they are a permutation of the retained
    /// children. Native collections may still hold children that are retiring.
    fn reordered_children(
        &self,
        object: ObjectId,
        relation: RelationId,
        native: &[ObjectId],
    ) -> Option<Vec<ObjectId>> {
        let retained = self.children(object, relation).unwrap_or_default();
        let children = native
            .iter()
            .copied()
            .filter(|child| self.get(*child).is_some())
            .collect::<Vec<_>>();
        let unique = children.iter().collect::<HashSet<_>>();
        (children.len() == retained.len()
            && unique.len() == children.len()
            && retained.iter().all(|child| unique.contains(child)))
        .then_some(children)
    }

    /// Applies a native observation. Returns `false` when the paired event must not be dispatched:
    /// a native order that already matches the retained order echoes Reactor's own mutation or
    /// a drop in place, and is not a reorder. Likewise, a selection that matches every child's
    /// retained selection flag reports nothing new, such as a control raising a deferred
    /// notification after Reactor wrote the flags.
    fn apply_observation(&mut self, observation: Observation, record: bool) -> bool {
        if !self.validate_observation(&observation) {
            return true;
        }
        match observation {
            Observation::SetProperty { object, property } => {
                let retained = self.get_mut(object);
                let property_id = property.id;
                let baseline = retained
                    .baselines
                    .iter()
                    .position(|baseline| baseline.id == property_id);
                // Observing a native value does not make the declaration own that property.
                let Some(current) = retained
                    .properties
                    .iter()
                    .find(|current| current.id == property_id)
                    .cloned()
                else {
                    match baseline {
                        Some(index) if retained.baselines[index] == property => return false,
                        Some(index) if record => retained.baselines[index] = property,
                        Some(index) => {
                            retained.baselines.swap_remove(index);
                        }
                        None if record => retained.baselines.push(property),
                        None => {}
                    }
                    return true;
                };
                // Native state that matches the declaration is settled, whichever write or
                // notification produced it, so only a departure from the declaration is reported.
                let settled = match baseline {
                    Some(index) if retained.baselines[index] == property => {
                        retained.baselines.swap_remove(index);
                        true
                    }
                    Some(_) => false,
                    None if current == property => return false,
                    None => {
                        retained.baselines.push(current);
                        false
                    }
                };
                retained
                    .properties
                    .upsert(|current| current.id == property_id, property);
                if settled {
                    return false;
                }
            }
            Observation::SetSelection { object, selected } => {
                let contract = selection_contract(self.get(object).unwrap().kind).unwrap();
                let children = contract
                    .relations
                    .iter()
                    .flat_map(|relation| {
                        self.children(object, *relation)
                            .unwrap_or_default()
                            .iter()
                            .copied()
                    })
                    .collect::<Vec<_>>();
                let item = selected.item();
                let matches_retained = selected != ObservedSelection::Untracked
                    && children.iter().all(|child| {
                        self.get(*child).unwrap().properties.iter().any(|property| {
                            property.id == contract.selected_property
                                && property.value == PropertyValue::Bool(Some(*child) == item)
                        })
                    });
                if matches_retained {
                    return false;
                }
                for child in children {
                    let child_object = self.get_mut(child);
                    if child_object
                        .properties
                        .iter()
                        .any(|property| property.id == contract.selected_property)
                    {
                        child_object.properties.upsert(
                            |property| property.id == contract.selected_property,
                            Property {
                                id: contract.selected_property,
                                value: PropertyValue::Bool(Some(child) == item),
                            },
                        );
                    }
                }
            }
            Observation::Reorder {
                object,
                relation,
                children,
            } => {
                let children = self
                    .reordered_children(object, relation, &children)
                    .unwrap();
                if let Some(retained) = self
                    .get_mut(object)
                    .relations
                    .iter_mut()
                    .find(|retained| retained.id == relation)
                {
                    if matches!(&retained.value, RetainedRelationValue::Many(current) if *current == children)
                    {
                        return false;
                    }
                    retained.value = RetainedRelationValue::Many(children);
                }
            }
        }
        true
    }

    pub(crate) fn virtual_realization(
        &self,
        collection: ObjectId,
        container: RealizedContainer,
    ) -> Option<ObjectId> {
        self.get(collection)?
            .virtual_items
            .as_ref()?
            .containers
            .get(&container)?
            .child
    }

    pub fn kind(&self, object: ObjectId) -> Option<ObjectType> {
        self.get(object).map(|object| object.kind)
    }

    pub fn children(&self, object: ObjectId, relation: RelationId) -> Option<&[ObjectId]> {
        match &self.relation(object, relation)?.value {
            RetainedRelationValue::Many(children) => Some(children),
            _ => None,
        }
    }

    pub fn child(&self, object: ObjectId, relation: RelationId) -> Option<ObjectId> {
        match &self.relation(object, relation)?.value {
            RetainedRelationValue::One(child) => *child,
            _ => None,
        }
    }

    pub(crate) fn owner(&self, child: ObjectId) -> Option<(ObjectId, RelationId)> {
        self.objects.iter().enumerate().find_map(|(index, slot)| {
            let object = (!slot.retiring).then_some(slot.object.as_ref()).flatten()?;
            object.relations.iter().find_map(|relation| {
                let contains = match &relation.value {
                    RetainedRelationValue::One(current) => *current == Some(child),
                    RetainedRelationValue::Many(children) => children.contains(&child),
                };
                if contains {
                    Some((
                        ObjectId {
                            index: index.try_into().ok()?,
                            generation: slot.generation,
                        },
                        relation.id,
                    ))
                } else {
                    None
                }
            })
        })
    }

    /// Validates a native event dispatch and resolves native item payloads to keys.
    fn validate_event_dispatch(&self, dispatch: &mut EventDispatch) -> bool {
        let Some(object) = self.get(dispatch.object) else {
            return false;
        };
        if dispatch.event == EventId::MenuItemInvoked {
            let Some(menu) = object
                .attachments
                .as_ref()
                .and_then(|attachments| attachments.menu.as_ref())
            else {
                return false;
            };
            return matches!(
                (&dispatch.callback, &dispatch.payload),
                (EventValue::Key(callback), EventPayload::Key(_))
                    if callback == &menu.menu.on_click
            );
        }
        if dispatch.event == EventId::CommandInvoked {
            let Some(flyout) = object
                .attachments
                .as_ref()
                .and_then(|attachments| attachments.command_bar_flyout.as_ref())
            else {
                return false;
            };
            return matches!(
                (&dispatch.callback, &dispatch.payload),
                (EventValue::Key(callback), EventPayload::Key(_))
                    if callback == &flyout.flyout.on_click
            );
        }
        let Some(contract) = event_contracts(object.kind)
            .iter()
            .find(|contract| contract.id == dispatch.event)
        else {
            panic!("invalid event {:?} for {:?}", dispatch.event, object.kind);
        };
        let valid = match (contract.value, &dispatch.callback, &dispatch.payload) {
            #[cfg(any(test, feature = "test"))]
            (
                ValueType::CharacterEventInfo,
                EventValue::CharacterEventInfo(_),
                EventPayload::CharacterEventInfo(_),
            )
            | (
                ValueType::KeyEventInfo,
                EventValue::KeyEventInfo(_),
                EventPayload::KeyEventInfo(_),
            ) => true,
            (ValueType::Bool, EventValue::Bool(_), EventPayload::Bool(_))
            | (ValueType::Color, EventValue::Color(_), EventPayload::Color(_))
            | (
                ValueType::ContentDialogResult,
                EventValue::ContentDialogResult(_),
                EventPayload::ContentDialogResult(_),
            )
            | (ValueType::String, EventValue::String(_), EventPayload::String(_))
            | (ValueType::F64, EventValue::F64(_), EventPayload::F64(_))
            | (
                ValueType::FocusEventInfo,
                EventValue::FocusEventInfo(_),
                EventPayload::FocusEventInfo(_),
            )
            | (ValueType::DragKind, EventValue::DragKind(_), EventPayload::DragKind(_))
            | (ValueType::DroppedData, EventValue::DroppedData(_), EventPayload::DroppedData(_))
            | (
                ValueType::OptionalBool,
                EventValue::OptionalBool(_),
                EventPayload::OptionalBool(_),
            )
            | (
                ValueType::OptionalDateTime,
                EventValue::OptionalDateTime(_),
                EventPayload::OptionalDateTime(_),
            )
            | (ValueType::OptionalF64, EventValue::OptionalF64(_), EventPayload::OptionalF64(_))
            | (
                ValueType::OptionalTimeSpan,
                EventValue::OptionalTimeSpan(_),
                EventPayload::OptionalTimeSpan(_),
            )
            | (
                ValueType::NavigationViewDisplayMode,
                EventValue::NavigationViewDisplayMode(_),
                EventPayload::NavigationViewDisplayMode(_),
            )
            | (ValueType::Selection, EventValue::Selection(_), EventPayload::Selection(_))
            | (
                ValueType::PointerEventInfo,
                EventValue::PointerEventInfo(_),
                EventPayload::PointerEventInfo(_),
            )
            | (
                ValueType::SelectionIndex,
                EventValue::SelectionIndex(_),
                EventPayload::SelectionIndex(_),
            )
            | (ValueType::Key, EventValue::Key(_), EventPayload::Item(_))
            | (ValueType::Keys, EventValue::Keys(_), EventPayload::Items(_))
            | (ValueType::Unit, EventValue::Unit(_), EventPayload::Unit) => true,
            _ => false,
        };
        assert!(valid, "invalid payload for event {:?}", dispatch.event);
        if let EventPayload::Selection(selection) = &dispatch.payload
            && let Some(item) = selection.item
        {
            let Some(contract) = selection_contract(object.kind) else {
                panic!("{:?} does not support selection", object.kind);
            };
            if !contract.relations.iter().any(|relation| {
                self.children(dispatch.object, *relation)
                    .is_some_and(|children| children.contains(&item))
            }) {
                return false;
            }
        }
        let resolved = match &dispatch.payload {
            EventPayload::Item(item) => {
                let child = object.relations.iter().any(|relation| {
                    matches!(&relation.value, RetainedRelationValue::Many(children) if children.contains(item))
                });
                let Some(key) = child.then(|| self.get(*item)?.key.clone()).flatten() else {
                    return false;
                };
                Some(EventPayload::Key(key))
            }
            EventPayload::Items(items) => {
                let Some(contract) = reorder_contracts(object.kind)
                    .iter()
                    .find(|contract| contract.event == dispatch.event)
                else {
                    panic!("invalid payload for event {:?}", dispatch.event);
                };
                // A stale payload no longer describes the retained order.
                let retained = self
                    .children(dispatch.object, contract.relation)
                    .unwrap_or_default();
                let Some(keys) = self
                    .reordered_children(dispatch.object, contract.relation, items)
                    .filter(|children| children == retained)
                    .and_then(|children| {
                        children
                            .iter()
                            .map(|child| self.get(*child)?.key.clone())
                            .collect::<Option<Vec<_>>>()
                    })
                else {
                    return false;
                };
                Some(EventPayload::Keys(keys))
            }
            _ => None,
        };
        // An occurrence reaches the callback subscribed when it fired, even if a render has since
        // replaced that callback. Removing the subscription drops occurrences still queued.
        let subscribed = object
            .events
            .as_deref()
            .into_iter()
            .flatten()
            .any(|event| event.id == dispatch.event);
        if subscribed && let Some(resolved) = resolved {
            dispatch.payload = resolved;
        }
        subscribed
    }

    fn get(&self, object: ObjectId) -> Option<&RetainedObject> {
        let slot = self.objects.get(object.index as usize)?;
        (slot.generation == object.generation && !slot.retiring)
            .then_some(slot.object.as_ref())
            .flatten()
    }

    /// Returns whether the adapter still owns the object, including during an exit transition.
    fn is_allocated(&self, object: ObjectId) -> bool {
        self.objects
            .get(object.index as usize)
            .is_some_and(|slot| slot.generation == object.generation && slot.object.is_some())
    }

    fn get_mut(&mut self, object: ObjectId) -> &mut RetainedObject {
        let slot = &mut self.objects[object.index as usize];
        assert_eq!(slot.generation, object.generation);
        assert!(!slot.retiring);
        slot.object.as_mut().unwrap()
    }

    fn relation(&self, object: ObjectId, relation: RelationId) -> Option<&RetainedRelation> {
        self.get(object)?
            .relations
            .iter()
            .find(|state| state.id == relation)
    }

    fn retirement_nodes(&self, root: ObjectId) -> Option<&[ObjectId]> {
        self.retirements
            .get(&root)
            .map(|retirement| retirement.nodes.as_slice())
    }

    fn retirements_for_parent(&self, parent: ObjectId) -> Vec<(ObjectId, Vec<ObjectId>)> {
        self.retirements
            .iter()
            .filter(|(_, retirement)| retirement.parent == parent)
            .map(|(root, retirement)| (*root, retirement.nodes.clone()))
            .collect()
    }

    fn matches_declaration(
        &self,
        object: ObjectId,
        declaration: &Declaration,
        remaining: &mut usize,
    ) -> bool {
        self.matches_declaration_inner(object, declaration, remaining, true)
    }

    fn matches_subtree_declaration(
        &self,
        object: ObjectId,
        declaration: &Declaration,
        remaining: &mut usize,
    ) -> bool {
        self.matches_declaration_inner(object, declaration, remaining, false)
    }

    fn matches_declaration_inner(
        &self,
        object: ObjectId,
        declaration: &Declaration,
        remaining: &mut usize,
        compare_key: bool,
    ) -> bool {
        let Some(next) = remaining.checked_sub(1) else {
            panic!("declaration exceeds the maximum of {MAX_OBJECTS} objects");
        };
        *remaining = next;
        let Some(current) = self.get(object) else {
            return false;
        };
        if current.kind != declaration.kind
            || (compare_key && current.key != declaration.key)
            || current.reference != declaration.reference
            || current.exit_transition != declaration.exit_transition
            || current.window_title_bar != declaration.window_title_bar
            || current.properties.as_slice() != declaration.properties.as_slice()
            || retained_events(&current.events) != declaration.events.as_slice()
            // A baseline for a declared property records an unsettled departure; the update
            // path settles it even when the app accepted the native value.
            || current.baselines.iter().any(|baseline| {
                declaration
                    .properties
                    .iter()
                    .any(|property| property.id == baseline.id)
            })
        {
            return false;
        }
        let current_tooltip = current
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.tooltip.as_ref());
        let declared_tooltip = declaration
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.tooltip.as_ref());
        match (current_tooltip, declared_tooltip) {
            (None, None) => {}
            (Some(current), Some(declared))
                if current.placement == declared.placement
                    && self.matches_declaration_inner(
                        current.object,
                        &declared.declaration(),
                        remaining,
                        true,
                    ) => {}
            _ => return false,
        }
        let current_menu = current
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.menu.as_ref());
        let declared_menu = declaration
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.menu.as_ref());
        match (current_menu, declared_menu) {
            (None, None) => {}
            (Some(current), Some(declared)) if current.menu == declared.menu => {}
            _ => return false,
        }
        let current_command_bar_flyout = current
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.command_bar_flyout.as_ref());
        let declared_command_bar_flyout = declaration
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.command_bar_flyout.as_ref());
        match (current_command_bar_flyout, declared_command_bar_flyout) {
            (None, None) => {}
            (Some(current), Some(declared)) if current.flyout == declared.flyout => {}
            _ => return false,
        }
        let current_flyout = current
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.flyout.as_ref());
        let declared_flyout = declaration
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.flyout.as_ref());
        match (current_flyout, declared_flyout) {
            (None, None) => {}
            (Some(current), Some(declared))
                if current.placement == declared.placement
                    && self.matches_declaration_inner(
                        current.object,
                        &declared.declaration(),
                        remaining,
                        true,
                    ) => {}
            _ => return false,
        }
        let current_dialog = current
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.content_dialog.as_ref());
        let declared_dialog = declaration
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.content_dialog.as_ref());
        match (current_dialog, declared_dialog) {
            (None, None) => {}
            (Some(current), Some(declared))
                if current.open == declared.open
                    && self.matches_declaration_inner(
                        current.object,
                        &declared.declaration,
                        remaining,
                        true,
                    ) => {}
            _ => return false,
        }
        if let Some(current_virtual) = &current.virtual_items {
            let declared = declaration.virtual_items.as_ref();
            if declared.map(|declared| &declared.items) != Some(&current_virtual.items)
                || declared
                    .and_then(|declared| declared.owner)
                    .or(declaration.component)
                    != current_virtual.owner
            {
                return false;
            }
        } else if declaration.virtual_items.is_some() {
            return false;
        }
        let contracts = relation_contracts(current.kind);
        for (index, relation) in declaration.relations.iter().enumerate() {
            if !contracts.iter().any(|contract| contract.id == relation.id)
                || declaration.relations.as_slice()[..index]
                    .iter()
                    .any(|previous| previous.id == relation.id)
            {
                return false;
            }
        }
        for contract in contracts {
            if contract.realization == Realization::Container && current.virtual_items.is_some() {
                continue;
            }
            let declared = declaration
                .relations
                .iter()
                .find(|relation| relation.id == contract.id);
            let retained = current
                .relations
                .iter()
                .find(|relation| relation.id == contract.id)
                .unwrap();
            match (&retained.value, declared.map(|relation| &relation.value)) {
                (RetainedRelationValue::One(None), None) => {}
                (RetainedRelationValue::Many(children), None) if children.is_empty() => {}
                (RetainedRelationValue::One(previous), Some(RelationValue::One(next))) => {
                    match (previous, next) {
                        (None, None) => {}
                        (Some(previous), Some(next))
                            if self.matches_declaration_inner(
                                *previous,
                                next.as_object(),
                                remaining,
                                true,
                            ) => {}
                        _ => return false,
                    }
                }
                (RetainedRelationValue::Many(previous), Some(RelationValue::Many(next))) => {
                    if previous.len() != next.len() {
                        return false;
                    }
                    for (previous, next) in previous.iter().zip(next.iter()) {
                        if !self.matches_declaration_inner(
                            *previous,
                            next.as_object(),
                            remaining,
                            true,
                        ) {
                            return false;
                        }
                    }
                }
                _ => return false,
            }
        }
        true
    }

    fn validate_realization_request(&self, request: RealizationRequest) -> bool {
        let (collection, source_revision, index) = match request {
            RealizationRequest::Realize {
                collection,
                source_revision,
                index,
                ..
            } => (collection, source_revision, Some(index)),
            RealizationRequest::Recycle {
                collection,
                source_revision,
                ..
            }
            | RealizationRequest::Cancel {
                collection,
                source_revision,
                ..
            } => (collection, source_revision, None),
        };
        let Some(object) = self.get(collection) else {
            return false;
        };
        let Some(items) = &object.virtual_items else {
            panic!("{collection:?} does not realize virtual items");
        };
        if items.source_revision != source_revision {
            return false;
        }
        if let Some(index) = index {
            assert!(
                index < items.keys.len(),
                "invalid realization {index} for {collection:?}"
            );
        }
        true
    }

    fn virtual_work(&self, request: RealizationRequest) -> Option<VirtualWork> {
        if !self.validate_realization_request(request) {
            return None;
        }
        match request {
            RealizationRequest::Realize {
                collection,
                container,
                index,
                ..
            } => {
                let items = self
                    .get(collection)
                    .unwrap()
                    .virtual_items
                    .as_ref()
                    .unwrap();
                let key = items.keys[index].clone();
                let revision = match items.active.get(&key) {
                    Some((revision, _)) => *revision,
                    None => items.lease_revision.checked_add(1).unwrap(),
                };
                Some(VirtualWork::Realize {
                    lease: RealizationLease {
                        collection,
                        container,
                        key,
                        revision,
                    },
                    index,
                    view: Box::new(items.items.view(index).unwrap()),
                    owner: items.owner,
                })
            }
            RealizationRequest::Recycle {
                collection,
                container,
                ..
            } => {
                let items = self
                    .get(collection)
                    .unwrap()
                    .virtual_items
                    .as_ref()
                    .unwrap();
                items
                    .containers
                    .get(&container)
                    .map(|realization| VirtualWork::Recycle {
                        lease: RealizationLease {
                            collection,
                            container,
                            key: realization.key.clone(),
                            revision: realization.revision,
                        },
                    })
            }
            RealizationRequest::Cancel {
                collection,
                container,
                ..
            } => {
                let relation = self
                    .get(collection)
                    .unwrap()
                    .virtual_items
                    .as_ref()
                    .unwrap()
                    .relation;
                Some(VirtualWork::Cancel {
                    collection,
                    relation,
                    container,
                })
            }
        }
    }
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

impl RetainedGraph {
    fn relation_mut(&mut self, object: ObjectId, relation: RelationId) -> &mut RetainedRelation {
        self.get_mut(object)
            .relations
            .iter_mut()
            .find(|state| state.id == relation)
            .unwrap()
    }

    fn set_window_title_bar(&mut self, object: ObjectId, height: WindowTitleBarHeight) {
        assert!(
            self.window_title_bar
                .is_none_or(|(current, _)| current == object),
            "window title bar declared more than once"
        );
        self.window_title_bar = Some((object, height));
    }

    fn clear_window_title_bar(&mut self, object: ObjectId) {
        if self
            .window_title_bar
            .is_some_and(|(current, _)| current == object)
        {
            self.window_title_bar = None;
        }
    }

    fn allocate(&mut self, object: RetainedObject) -> ObjectId {
        if let Some(index) = self.free.pop() {
            let slot = &mut self.objects[index as usize];
            slot.object = Some(object);
            slot.retiring = false;
            ObjectId {
                index,
                generation: slot.generation,
            }
        } else {
            let index = self.objects.len().try_into().unwrap();
            self.objects.push(RetainedSlot {
                generation: 0,
                object: Some(object),
                retiring: false,
            });
            ObjectId {
                index,
                generation: 0,
            }
        }
    }

    fn remove(&mut self, object: ObjectId) {
        let slot = &mut self.objects[object.index as usize];
        assert_eq!(slot.generation, object.generation);
        slot.object = None;
        slot.retiring = false;
        slot.generation = slot.generation.wrapping_add(1);
        self.free.push(object.index);
    }

    fn mark_retiring(&mut self, root: ObjectId, nodes: Vec<ObjectId>, parent: ObjectId) {
        for object in &nodes {
            let slot = &mut self.objects[object.index as usize];
            assert_eq!(slot.generation, object.generation);
            assert!(!slot.retiring);
            slot.retiring = true;
        }
        assert!(
            self.retirements
                .insert(root, RetainedRetirement { nodes, parent })
                .is_none()
        );
    }

    fn complete_retirement(&mut self, root: ObjectId) {
        let Some(retirement) = self.retirements.remove(&root) else {
            return;
        };
        for object in retirement.nodes {
            self.remove(object);
        }
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

impl<A: Adapter> Runtime<A> {
    pub fn new(adapter: A) -> Self {
        Self {
            graph: RetainedGraph::default(),
            adapter,
            mutations: Vec::new(),
            virtual_refreshes: VecDeque::new(),
            validator: DeclarationValidator::default(),
            native_event_active: Rc::new(Cell::new(false)),
            references: ReferenceEndpoint::new(),
        }
    }

    pub fn graph(&self) -> &RetainedGraph {
        &self.graph
    }

    pub fn adapter(&self) -> &A {
        &self.adapter
    }

    pub(crate) fn adapter_mut(&mut self) -> &mut A {
        &mut self.adapter
    }

    fn finish_update(&mut self) -> UpdateStats {
        let stats = UpdateStats::capture(&self.mutations);
        if self.mutations.capacity() > 256 {
            self.mutations = Vec::with_capacity(256);
        }
        stats
    }

    /// Applies the planned mutations, then publishes the planned reference changes.
    fn apply_plan(&mut self, references: Vec<ReferenceChange>, before_apply: impl FnOnce()) {
        defer_window_title_bar_sets(&mut self.mutations);
        before_apply();
        self.adapter.apply(&self.mutations);
        apply_reference_changes(&self.references, references);
    }

    pub(crate) fn set_imperative_waker(&self, waker: impl Fn() + 'static) {
        self.references.set_waker(waker);
    }

    fn dispatch_imperative(&mut self) -> bool {
        let Some(queued) = self.references.pop() else {
            return false;
        };
        // Revocations must also reach retiring objects, whose native subscriptions stay live until
        // the exit transition completes.
        let available = if matches!(queued.request, ImperativeRequest::RevokeObservation { .. }) {
            self.graph.is_allocated(queued.object)
        } else {
            self.graph.kind(queued.object).is_some() && queued.is_current()
        };
        if available {
            self.adapter.imperative(queued.request);
        } else {
            queued.request.complete_unavailable();
        }
        true
    }

    fn pop_native_work(&mut self) -> Option<NativeWork> {
        let event = self.adapter.pop_native_event()?;
        // Only a property's controlled event reports its state; an action that also observes the
        // property, such as `Click`, dispatches even when the state is settled.
        let report = match (&event.observation, &event.event) {
            (Some(Observation::SetProperty { object, property }), Some(dispatch)) => {
                Some(self.graph.kind(*object).is_some_and(|kind| {
                    controlled_event(kind, property.id) == Some(dispatch.event)
                }))
            }
            _ => None,
        };
        // A report followed by another report of the same state is skipped, so a control that
        // passes through intermediate states within one interaction reports only where it settled.
        if report == Some(true) {
            let current = event.event.as_ref().unwrap();
            if self.adapter.peek_native_dispatch().is_some_and(|next| {
                next.observation.is_some()
                    && next.event.is_some_and(|next| {
                        next.object == current.object && next.event == current.event
                    })
            }) {
                return Some(NativeWork::Maintenance);
            }
        }
        let action = report == Some(false);
        let dispatch = event
            .observation
            .is_none_or(|observation| self.graph.apply_observation(observation, event.record))
            || action;
        if let Some(completion) = event.retirement {
            self.apply_retirement_completion(completion);
        }
        if let Some(request) = event.realization {
            if let RealizationRequest::Recycle {
                collection,
                container,
                ..
            }
            | RealizationRequest::Cancel {
                collection,
                container,
                ..
            } = request
            {
                self.virtual_refreshes.retain(|refresh| {
                    !matches!(
                        refresh,
                        RealizationRequest::Realize {
                            collection: refresh_collection,
                            container: refresh_container,
                            ..
                        } if *refresh_collection == collection && *refresh_container == container
                    )
                });
            }
            return Some(
                self.graph
                    .virtual_work(request)
                    .map_or(NativeWork::Maintenance, NativeWork::Virtual),
            );
        }
        let Some(mut dispatch) = event.event.filter(|_| dispatch) else {
            return Some(NativeWork::Maintenance);
        };
        if !self.graph.validate_event_dispatch(&mut dispatch) {
            return Some(NativeWork::Maintenance);
        }
        self.native_event_active.set(true);
        Some(NativeWork::Event(NativeEventDispatch::new(
            dispatch,
            Rc::clone(&self.native_event_active),
        )))
    }

    pub(crate) fn next_native_work(&mut self) -> Option<NativeWork> {
        assert!(
            !self.native_event_active.get(),
            "native event dispatch is in progress"
        );
        if let Some(work) = self.pop_native_work() {
            return Some(work);
        }
        if let Some(request) = self.virtual_refreshes.pop_front() {
            return Some(
                self.graph
                    .virtual_work(request)
                    .map_or(NativeWork::Maintenance, NativeWork::Virtual),
            );
        }
        for _ in 0..64 {
            if !self.dispatch_imperative() {
                return None;
            }
        }
        self.references.wake();
        None
    }

    /// Returns the virtual work for a realization at the front of the native queue, after applying
    /// the observations ahead of it. Returns `None` when a native event comes first.
    pub(crate) fn next_realization(&mut self) -> Option<VirtualWork> {
        loop {
            if self.prepare_update() {
                return None;
            }
            let next = self.adapter.peek_native_event()?;
            if next.event.is_some() || next.realization.is_none() {
                return None;
            }
            match self.pop_native_work()? {
                NativeWork::Virtual(work) => return Some(work),
                NativeWork::Maintenance => {}
                NativeWork::Event(_) => unreachable!(),
            }
        }
    }

    /// Applies queued native observations and retirement completions. Returns `false` while a
    /// native callback or virtual realization must run before the next update.
    pub(crate) fn prepare_update(&mut self) -> bool {
        if self.native_event_active.get() {
            return true;
        }
        if !self.virtual_refreshes.is_empty() {
            return false;
        }
        while let Some(next) = self.adapter.peek_native_event() {
            if next.event.is_some() || next.realization.is_some() {
                return false;
            }
            let event = self.adapter.pop_native_event().unwrap();
            if let Some(observation) = event.observation {
                self.graph.apply_observation(observation, event.record);
            }
            if let Some(completion) = event.retirement {
                self.apply_retirement_completion(completion);
            }
        }
        true
    }

    pub(crate) fn assert_ready(&mut self) {
        assert!(self.prepare_update(), "native events are pending");
    }

    pub(crate) fn realize_virtual(
        &mut self,
        lease: &RealizationLease,
        index: usize,
        root: impl Into<View>,
    ) -> UpdateStats {
        self.mutations.clear();
        let mut declaration = root.into().0.object();
        declaration.key = Some(lease.key.clone());
        self.validator.validate(&declaration);
        let items = self
            .graph
            .get(lease.collection)
            .and_then(|object| object.virtual_items.as_ref())
            .unwrap();
        assert!(
            items.keys.get(index) == Some(&lease.key),
            "invalid realization {index} for {:?}",
            lease.collection
        );
        let relation_id = items.relation;
        let mut child = items
            .active
            .get(&lease.key)
            .and_then(|(_, container)| items.containers.get(container))
            .and_then(|active| active.child);
        let replaced_container = items
            .containers
            .get(&lease.container)
            .filter(|current| current.key != lease.key || current.revision != lease.revision)
            .map(|current| {
                (
                    current.child,
                    items
                        .active
                        .get(&current.key)
                        .is_some_and(|(_, container)| *container == lease.container),
                )
            });
        let expected_revision = match items.active.get(&lease.key) {
            Some((revision, _)) => Some(*revision),
            None => items.lease_revision.checked_add(1),
        };
        assert!(
            expected_revision == Some(lease.revision),
            "invalid realization {index} for {:?}",
            lease.collection
        );
        let moved_container = match items.active.get(&lease.key) {
            Some((_, container)) => (*container != lease.container).then(|| {
                (
                    *container,
                    items
                        .containers
                        .get(container)
                        .and_then(|active| active.child),
                )
            }),
            None => None,
        };
        let mut references = Vec::new();
        if let Some((previous, retire)) = replaced_container {
            self.mutations.push(Mutation::Recycle {
                parent: lease.collection,
                relation: relation_id,
                container: lease.container,
                child: previous,
            });
            if retire && let Some(previous) = previous {
                let relation = self.graph.relation_mut(lease.collection, relation_id);
                let RetainedRelationValue::Many(children) = &mut relation.value else {
                    unreachable!();
                };
                children.retain(|current| *current != previous);
                let mut planner = Planner {
                    retained: &mut self.graph,
                    mutations: &mut self.mutations,
                    references: &mut references,
                    virtual_refreshes: None,
                };
                planner.retire(previous);
            }
        }
        if let Some((container, previous_child)) = moved_container {
            self.mutations.push(Mutation::Recycle {
                parent: lease.collection,
                relation: relation_id,
                container,
                child: previous_child,
            });
        }
        if let Some(current) = child {
            if self.graph.get(current).unwrap().kind == declaration.kind {
                let mut planner = Planner {
                    retained: &mut self.graph,
                    mutations: &mut self.mutations,
                    references: &mut references,
                    virtual_refreshes: None,
                };
                planner.reconcile_object(current, &declaration);
            } else {
                let relation = self.graph.relation_mut(lease.collection, relation_id);
                let RetainedRelationValue::Many(children) = &mut relation.value else {
                    unreachable!();
                };
                children.retain(|candidate| *candidate != current);
                let mut planner = Planner {
                    retained: &mut self.graph,
                    mutations: &mut self.mutations,
                    references: &mut references,
                    virtual_refreshes: None,
                };
                planner.retire(current);
                child = None;
            }
        }
        let child = if let Some(child) = child {
            child
        } else {
            let mut planner = Planner {
                retained: &mut self.graph,
                mutations: &mut self.mutations,
                references: &mut references,
                virtual_refreshes: None,
            };
            planner.mount(&declaration)
        };
        let items = self
            .graph
            .get_mut(lease.collection)
            .virtual_items
            .as_mut()
            .unwrap();
        if !items.active.contains_key(&lease.key) {
            items.lease_revision = lease.revision;
        }
        if let Some((_, previous)) = items
            .active
            .insert(lease.key.clone(), (lease.revision, lease.container))
        {
            items.containers.remove(&previous);
        }
        if let Some(previous) = items.containers.insert(
            lease.container,
            RetainedRealization {
                key: lease.key.clone(),
                index,
                revision: lease.revision,
                child: Some(child),
            },
        ) && previous.key != lease.key
        {
            items.active.remove(&previous.key);
        }
        let ordered = {
            let mut active = items
                .containers
                .values()
                .filter_map(|realization| realization.child.map(|child| (realization.index, child)))
                .collect::<Vec<_>>();
            active.sort_unstable_by_key(|(index, _)| *index);
            active.into_iter().map(|(_, child)| child).collect()
        };
        let relation = self.graph.relation_mut(lease.collection, relation_id);
        let RetainedRelationValue::Many(children) = &mut relation.value else {
            unreachable!();
        };
        *children = ordered;
        self.mutations.push(Mutation::Realize {
            parent: lease.collection,
            relation: relation_id,
            container: lease.container,
            index,
            child,
        });
        self.apply_plan(references, || {});
        self.finish_update()
    }

    pub(crate) fn recycle_virtual_before_apply(
        &mut self,
        lease: &RealizationLease,
        before_apply: impl FnOnce(),
    ) -> UpdateStats {
        self.mutations.clear();
        let items = self
            .graph
            .get(lease.collection)
            .and_then(|object| object.virtual_items.as_ref())
            .unwrap();
        let mut references = Vec::new();
        if let Some(realization) = items.containers.get(&lease.container)
            && realization.key == lease.key
            && realization.revision == lease.revision
        {
            let relation_id = items.relation;
            let child = realization.child;
            self.mutations.push(Mutation::Recycle {
                parent: lease.collection,
                relation: relation_id,
                container: lease.container,
                child,
            });
            let items = self
                .graph
                .get_mut(lease.collection)
                .virtual_items
                .as_mut()
                .unwrap();
            items.containers.remove(&lease.container);
            if items
                .active
                .get(&lease.key)
                .is_some_and(|(_, container)| *container == lease.container)
            {
                items.active.remove(&lease.key);
            }
            if let Some(child) = child {
                let relation = self.graph.relation_mut(lease.collection, relation_id);
                let RetainedRelationValue::Many(children) = &mut relation.value else {
                    unreachable!();
                };
                children.retain(|current| *current != child);
                Planner {
                    retained: &mut self.graph,
                    mutations: &mut self.mutations,
                    references: &mut references,
                    virtual_refreshes: None,
                }
                .retire(child);
            }
        }
        self.apply_plan(references, before_apply);
        self.finish_update()
    }

    pub(crate) fn cancel_virtual(
        &mut self,
        collection: ObjectId,
        relation: RelationId,
        container: RealizedContainer,
    ) -> UpdateStats {
        self.mutations.clear();
        self.mutations.push(Mutation::Recycle {
            parent: collection,
            relation,
            container,
            child: None,
        });
        self.apply_plan(Vec::new(), || {});
        self.finish_update()
    }

    pub(crate) fn virtual_lease_active(&self, collection: ObjectId, key: &Key) -> bool {
        self.graph
            .get(collection)
            .and_then(|object| object.virtual_items.as_ref())
            .is_some_and(|items| items.active.contains_key(key))
    }

    pub fn update(&mut self, root: impl Into<View>) -> UpdateStats {
        self.assert_ready();
        self.mutations.clear();
        let declaration = root.into().0.object();
        if let Some(root) = self.graph.root {
            let mut remaining = MAX_OBJECTS;
            if self
                .graph
                .matches_declaration(root, &declaration, &mut remaining)
            {
                return UpdateStats::default();
            }
        }
        let object_count = self.validator.validate(&declaration);
        if self.graph.objects.is_empty() {
            self.graph.objects.reserve_exact(object_count);
        }
        if let Some(current) = self.graph.root {
            let previous = self.graph.get(current).unwrap().kind;
            let next = declaration.kind;
            assert!(
                previous == next,
                "root type changed from {previous:?} to {next:?}"
            );
        }
        let mut references = Vec::new();
        let mut virtual_refreshes = Vec::new();
        let mut planner = Planner {
            retained: &mut self.graph,
            mutations: &mut self.mutations,
            references: &mut references,
            virtual_refreshes: Some(&mut virtual_refreshes),
        };
        let root = match planner.retained.root {
            Some(current) => planner.reconcile_object(current, &declaration),
            None => planner.mount(&declaration),
        };
        planner.retained.root = Some(root);
        self.apply_plan(references, || {});
        self.virtual_refreshes.extend(virtual_refreshes);
        self.finish_update()
    }

    pub(crate) fn update_subtree_before_apply(
        &mut self,
        object: ObjectId,
        root: impl Into<View>,
        before_apply: impl FnOnce(),
    ) -> UpdateStats {
        self.update_subtree_inner(object, root.into(), None, before_apply)
    }

    fn update_subtree_inner(
        &mut self,
        object: ObjectId,
        declaration: View,
        owner: Option<(ObjectId, RelationId)>,
        before_apply: impl FnOnce(),
    ) -> UpdateStats {
        self.assert_ready();
        self.mutations.clear();
        let declaration = declaration.0.object();
        self.validator.validate(&declaration);
        let Some(previous) = self.graph.kind(object) else {
            panic!("stale object {object:?}");
        };
        if previous != declaration.kind {
            let owner = owner.or_else(|| self.graph.owner(object));
            let Some((parent, relation)) = owner else {
                panic!(
                    "root type changed from {previous:?} to {:?}",
                    declaration.kind
                );
            };
            let owns_object =
                self.graph
                    .relation(parent, relation)
                    .is_some_and(|owned| match &owned.value {
                        RetainedRelationValue::One(child) => *child == Some(object),
                        RetainedRelationValue::Many(children) => children.contains(&object),
                    });
            assert!(
                owns_object,
                "{object:?} is not a child of relation {relation:?}"
            );
            let contract = relation_contracts(self.graph.kind(parent).unwrap())
                .iter()
                .find(|contract| contract.id == relation)
                .unwrap();
            assert!(
                contract.realization == Realization::Owned
                    && relation_accepts(contract, declaration.kind),
                "{:?} is not a valid child of relation {relation:?}",
                declaration.kind
            );
        }
        let mut remaining = MAX_OBJECTS;
        if self
            .graph
            .matches_subtree_declaration(object, &declaration, &mut remaining)
        {
            before_apply();
            return UpdateStats::default();
        }
        let mut references = Vec::new();
        let mut virtual_refreshes = Vec::new();
        let mut planner = Planner {
            retained: &mut self.graph,
            mutations: &mut self.mutations,
            references: &mut references,
            virtual_refreshes: Some(&mut virtual_refreshes),
        };
        if previous == declaration.kind {
            planner.reconcile_object(object, &declaration);
        } else {
            planner.replace_object(object, &declaration);
        }
        self.apply_plan(references, before_apply);
        self.virtual_refreshes.extend(virtual_refreshes);
        self.finish_update()
    }

    fn apply_retirement_completion(&mut self, completion: RetirementCompletion) {
        self.mutations.clear();
        let Some(nodes) = self.graph.retirement_nodes(completion.root) else {
            return;
        };
        self.mutations.push(Mutation::CompleteRetirement {
            root: completion.root,
            nodes: nodes.to_vec(),
        });
        self.graph.complete_retirement(completion.root);
        self.apply_plan(Vec::new(), || {});
        self.mutations.clear();
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

impl Planner<'_> {
    fn retained_attachments(&mut self, owner: ObjectId) -> &mut RetainedAttachments {
        self.retained
            .get_mut(owner)
            .attachments
            .get_or_insert_with(Default::default)
    }

    fn mount_tooltip(&mut self, target: ObjectId, tooltip: &DeclaredTooltip) {
        let object = self.mount(&tooltip.declaration());
        self.retained_attachments(target).tooltip = Some(RetainedTooltip {
            object,
            placement: tooltip.placement,
        });
        self.mutations.push(Mutation::SetTooltip {
            target,
            tooltip: Some(object),
            placement: tooltip.placement,
        });
    }

    fn clear_tooltip(&mut self, target: ObjectId) {
        let tooltip = self
            .retained
            .get_mut(target)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.tooltip.take());
        if let Some(tooltip) = tooltip {
            self.mutations.push(Mutation::SetTooltip {
                target,
                tooltip: None,
                placement: tooltip.placement,
            });
            self.retire(tooltip.object);
        }
    }

    fn reconcile_tooltip(
        &mut self,
        target: ObjectId,
        previous: Option<RetainedTooltip>,
        desired: Option<&DeclaredTooltip>,
        target_replaced: bool,
    ) {
        match (previous, desired) {
            (None, None) => {}
            (None, Some(desired)) => self.mount_tooltip(target, desired),
            (Some(previous), None) => {
                if !target_replaced {
                    self.mutations.push(Mutation::SetTooltip {
                        target,
                        tooltip: None,
                        placement: previous.placement,
                    });
                }
                self.retire(previous.object);
            }
            (Some(previous), Some(desired)) => {
                let object = self.reconcile_object(previous.object, &desired.declaration());
                if target_replaced
                    || object != previous.object
                    || desired.placement != previous.placement
                {
                    self.mutations.push(Mutation::SetTooltip {
                        target,
                        tooltip: Some(object),
                        placement: desired.placement,
                    });
                }
                self.retained_attachments(target).tooltip = Some(RetainedTooltip {
                    object,
                    placement: desired.placement,
                });
            }
        }
    }

    fn mount_flyout(&mut self, target: ObjectId, flyout: &DeclaredFlyout) {
        let object = self.mount(&flyout.declaration());
        self.retained_attachments(target).flyout = Some(RetainedFlyout {
            object,
            placement: flyout.placement,
        });
        self.mutations.push(Mutation::SetFlyout {
            target,
            content: Some(object),
            placement: flyout.placement,
        });
    }

    fn clear_flyout(&mut self, target: ObjectId) {
        let flyout = self
            .retained
            .get_mut(target)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.flyout.take());
        if let Some(flyout) = flyout {
            self.mutations.push(Mutation::SetFlyout {
                target,
                content: None,
                placement: flyout.placement,
            });
            self.retire(flyout.object);
        }
    }

    fn reconcile_flyout(
        &mut self,
        target: ObjectId,
        previous: Option<RetainedFlyout>,
        desired: Option<&DeclaredFlyout>,
        target_replaced: bool,
    ) {
        match (previous, desired) {
            (None, None) => {}
            (None, Some(desired)) => self.mount_flyout(target, desired),
            (Some(previous), None) => {
                if !target_replaced {
                    self.mutations.push(Mutation::SetFlyout {
                        target,
                        content: None,
                        placement: previous.placement,
                    });
                }
                self.retire(previous.object);
            }
            (Some(previous), Some(desired)) => {
                let object = self.reconcile_object(previous.object, &desired.declaration());
                if target_replaced
                    || object != previous.object
                    || desired.placement != previous.placement
                {
                    self.mutations.push(Mutation::SetFlyout {
                        target,
                        content: Some(object),
                        placement: desired.placement,
                    });
                }
                self.retained_attachments(target).flyout = Some(RetainedFlyout {
                    object,
                    placement: desired.placement,
                });
            }
        }
    }

    fn mount_menu(&mut self, target: ObjectId, menu: &DeclaredMenu) {
        let revision = 1;
        self.retained_attachments(target).menu = Some(RetainedMenu {
            menu: menu.menu.clone(),
            revision,
        });
        self.mutations.push(Mutation::SetMenu {
            target,
            menu: Some(menu.menu.clone()),
            revision,
        });
    }

    fn clear_menu(&mut self, target: ObjectId) {
        let menu = self
            .retained
            .get_mut(target)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.menu.take());
        if let Some(menu) = menu {
            self.mutations.push(Mutation::SetMenu {
                target,
                menu: None,
                revision: menu.revision.wrapping_add(1),
            });
        }
    }

    fn reconcile_menu(
        &mut self,
        target: ObjectId,
        previous: Option<RetainedMenu>,
        desired: Option<&DeclaredMenu>,
        target_replaced: bool,
    ) {
        match (previous, desired) {
            (None, None) => {}
            (None, Some(desired)) => self.mount_menu(target, desired),
            (Some(previous), None) => {
                if !target_replaced {
                    self.mutations.push(Mutation::SetMenu {
                        target,
                        menu: None,
                        revision: previous.revision.wrapping_add(1),
                    });
                }
            }
            (Some(previous), Some(desired))
                if !target_replaced && previous.menu == desired.menu =>
            {
                self.retained_attachments(target).menu = Some(previous);
            }
            (Some(previous), Some(desired)) => {
                let revision = if !target_replaced && previous.menu.same_content(&desired.menu) {
                    previous.revision
                } else {
                    previous.revision.wrapping_add(1)
                };
                self.retained_attachments(target).menu = Some(RetainedMenu {
                    menu: desired.menu.clone(),
                    revision,
                });
                self.mutations.push(Mutation::SetMenu {
                    target,
                    menu: Some(desired.menu.clone()),
                    revision,
                });
            }
        }
    }

    fn mount_command_bar_flyout(&mut self, target: ObjectId, flyout: &DeclaredCommandBarFlyout) {
        let revision = 1;
        self.retained_attachments(target).command_bar_flyout = Some(RetainedCommandBarFlyout {
            flyout: flyout.flyout.clone(),
            revision,
        });
        self.mutations.push(Mutation::SetCommandBarFlyout {
            target,
            flyout: Some(flyout.flyout.clone()),
            revision,
        });
    }

    fn clear_command_bar_flyout(&mut self, target: ObjectId) {
        let flyout = self
            .retained
            .get_mut(target)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.command_bar_flyout.take());
        if let Some(flyout) = flyout {
            self.mutations.push(Mutation::SetCommandBarFlyout {
                target,
                flyout: None,
                revision: flyout.revision.wrapping_add(1),
            });
        }
    }

    fn reconcile_command_bar_flyout(
        &mut self,
        target: ObjectId,
        previous: Option<RetainedCommandBarFlyout>,
        desired: Option<&DeclaredCommandBarFlyout>,
        target_replaced: bool,
    ) {
        match (previous, desired) {
            (None, None) => {}
            (None, Some(desired)) => self.mount_command_bar_flyout(target, desired),
            (Some(previous), None) => {
                if !target_replaced {
                    self.mutations.push(Mutation::SetCommandBarFlyout {
                        target,
                        flyout: None,
                        revision: previous.revision.wrapping_add(1),
                    });
                }
            }
            (Some(previous), Some(desired))
                if !target_replaced && previous.flyout == desired.flyout =>
            {
                self.retained_attachments(target).command_bar_flyout = Some(previous);
            }
            (Some(previous), Some(desired)) => {
                let revision = if !target_replaced && previous.flyout.same_content(&desired.flyout)
                {
                    previous.revision
                } else {
                    previous.revision.wrapping_add(1)
                };
                self.retained_attachments(target).command_bar_flyout =
                    Some(RetainedCommandBarFlyout {
                        flyout: desired.flyout.clone(),
                        revision,
                    });
                self.mutations.push(Mutation::SetCommandBarFlyout {
                    target,
                    flyout: Some(desired.flyout.clone()),
                    revision,
                });
            }
        }
    }

    fn mount_content_dialog(&mut self, owner: ObjectId, dialog: &DeclaredContentDialog) {
        let object = self.mount(&dialog.declaration);
        self.retained_attachments(owner).content_dialog = Some(RetainedContentDialog {
            object,
            open: dialog.open,
        });
        self.mutations.push(Mutation::SetContentDialog {
            owner,
            dialog: Some(object),
        });
        if dialog.open {
            self.mutations.push(Mutation::SetContentDialogOpen {
                dialog: object,
                open: true,
            });
        }
    }

    fn clear_content_dialog(&mut self, owner: ObjectId) {
        let dialog = self
            .retained
            .get_mut(owner)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.content_dialog.take());
        if let Some(dialog) = dialog {
            if dialog.open {
                self.mutations.push(Mutation::SetContentDialogOpen {
                    dialog: dialog.object,
                    open: false,
                });
            }
            self.mutations.push(Mutation::SetContentDialog {
                owner,
                dialog: None,
            });
            self.retire(dialog.object);
        }
    }

    fn reconcile_content_dialog(
        &mut self,
        owner: ObjectId,
        previous: Option<RetainedContentDialog>,
        desired: Option<&DeclaredContentDialog>,
        owner_replaced: bool,
    ) {
        match (previous, desired) {
            (None, None) => {}
            (None, Some(desired)) => self.mount_content_dialog(owner, desired),
            (Some(previous), None) => {
                if !owner_replaced {
                    if previous.open {
                        self.mutations.push(Mutation::SetContentDialogOpen {
                            dialog: previous.object,
                            open: false,
                        });
                    }
                    self.mutations.push(Mutation::SetContentDialog {
                        owner,
                        dialog: None,
                    });
                }
                self.retire(previous.object);
            }
            (Some(previous), Some(desired)) => {
                let object = self.reconcile_object(previous.object, &desired.declaration);
                if owner_replaced || object != previous.object {
                    self.mutations.push(Mutation::SetContentDialog {
                        owner,
                        dialog: Some(object),
                    });
                }
                if owner_replaced && desired.open || previous.open != desired.open {
                    self.mutations.push(Mutation::SetContentDialogOpen {
                        dialog: object,
                        open: desired.open,
                    });
                }
                self.retained_attachments(owner).content_dialog = Some(RetainedContentDialog {
                    object,
                    open: desired.open,
                });
            }
        }
    }

    fn replace_object(&mut self, object: ObjectId, declaration: &Declaration) {
        if self
            .retained
            .get_mut(object)
            .window_title_bar
            .take()
            .is_some()
        {
            self.retained.clear_window_title_bar(object);
            self.mutations
                .push(Mutation::ClearWindowTitleBar { object });
        }
        let previous_tooltip = self
            .retained
            .get_mut(object)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.tooltip.take());
        if let Some(tooltip) = &previous_tooltip {
            self.mutations.push(Mutation::SetTooltip {
                target: object,
                tooltip: None,
                placement: tooltip.placement,
            });
        }
        let previous_flyout = self
            .retained
            .get_mut(object)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.flyout.take());
        if let Some(flyout) = &previous_flyout {
            self.mutations.push(Mutation::SetFlyout {
                target: object,
                content: None,
                placement: flyout.placement,
            });
        }
        let previous_menu = self
            .retained
            .get_mut(object)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.menu.take());
        if let Some(menu) = &previous_menu {
            self.mutations.push(Mutation::SetMenu {
                target: object,
                menu: None,
                revision: menu.revision.wrapping_add(1),
            });
        }
        let previous_command_bar_flyout = self
            .retained
            .get_mut(object)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.command_bar_flyout.take());
        if let Some(flyout) = &previous_command_bar_flyout {
            self.mutations.push(Mutation::SetCommandBarFlyout {
                target: object,
                flyout: None,
                revision: flyout.revision.wrapping_add(1),
            });
        }
        let previous_content_dialog = self
            .retained
            .get_mut(object)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.content_dialog.take());
        if let Some(dialog) = &previous_content_dialog {
            if dialog.open {
                self.mutations.push(Mutation::SetContentDialogOpen {
                    dialog: dialog.object,
                    open: false,
                });
            }
            self.mutations.push(Mutation::SetContentDialog {
                owner: object,
                dialog: None,
            });
        }
        let previous = std::mem::take(&mut self.retained.get_mut(object).relations);
        for relation in previous {
            match relation.value {
                RetainedRelationValue::One(Some(child)) => {
                    self.mutations.push(Mutation::Detach {
                        parent: object,
                        relation: relation.id,
                        child,
                    });
                    self.retire(child);
                }
                RetainedRelationValue::Many(children) => {
                    for (index, child) in children.into_iter().enumerate().rev() {
                        self.mutations.push(Mutation::Remove {
                            parent: object,
                            relation: relation.id,
                            child,
                            index,
                        });
                        self.retire(child);
                    }
                }
                RetainedRelationValue::One(None) => {}
            }
        }
        let retained = self.retained.get_mut(object);
        let key = retained.key.take();
        if let Some(reference) = retained.reference.take() {
            self.references
                .push(ReferenceChange::Clear { reference, object });
        }
        // Ending the replaced control's subscriptions drops its pending occurrences, which would
        // otherwise reach the replacement through the preserved ID.
        if let Some(events) = retained.events.take() {
            self.mutations.push(Mutation::SetEvents {
                object,
                set: Rc::from([]),
                clear: events.iter().map(|event| event.id).collect(),
            });
        }
        if let Some(reference) = &declaration.reference {
            self.references.push(ReferenceChange::Set {
                reference: reference.clone(),
                object,
            });
        }
        let virtual_items = retain_virtual_items(declaration);
        *self.retained.get_mut(object) = RetainedObject {
            kind: declaration.kind,
            key,
            reference: declaration.reference.clone(),
            exit_transition: declaration.exit_transition,
            window_title_bar: declaration.window_title_bar,
            attachments: None,
            properties: declaration.properties.clone(),
            baselines: Vec::new(),
            events: retain_events(&declaration.events),
            relations: relation_contracts(declaration.kind)
                .iter()
                .map(|contract| RetainedRelation {
                    id: contract.id,
                    value: match contract.cardinality {
                        Cardinality::One => RetainedRelationValue::One(None),
                        Cardinality::Many => RetainedRelationValue::Many(Vec::new()),
                    },
                })
                .collect(),
            virtual_items,
        };
        self.mutations.push(Mutation::Replace {
            object,
            kind: declaration.kind,
        });
        let set = properties_before_relations(declaration.kind, declaration.properties.as_slice());
        if !set.is_empty() {
            self.mutations.push(Mutation::SetProperties {
                object,
                set,
                clear: Rc::from([]),
            });
        }
        if !declaration.events.as_slice().is_empty() {
            self.mutations.push(Mutation::SetEvents {
                object,
                set: Rc::from(declaration.events.as_slice()),
                clear: Rc::from([]),
            });
        }
        if let Some(items) = &self.retained.get(object).unwrap().virtual_items {
            self.mutations.push(Mutation::SetVirtualSource {
                object,
                item_count: items.keys.len(),
                source_revision: items.source_revision,
            });
        }
        for contract in relation_contracts(declaration.kind) {
            self.mount_relation(object, declaration, contract);
        }
        self.apply_index(object, declaration, None, false);
        if let Some(height) = declaration.window_title_bar {
            self.retained.set_window_title_bar(object, height);
            self.mutations
                .push(Mutation::SetWindowTitleBar { object, height });
        }
        self.reconcile_tooltip(
            object,
            previous_tooltip,
            declaration
                .attachments
                .as_ref()
                .and_then(|attachments| attachments.tooltip.as_ref()),
            true,
        );
        self.reconcile_flyout(
            object,
            previous_flyout,
            declaration
                .attachments
                .as_ref()
                .and_then(|attachments| attachments.flyout.as_ref()),
            true,
        );
        self.reconcile_menu(
            object,
            previous_menu,
            declaration
                .attachments
                .as_ref()
                .and_then(|attachments| attachments.menu.as_ref()),
            true,
        );
        self.reconcile_command_bar_flyout(
            object,
            previous_command_bar_flyout,
            declaration
                .attachments
                .as_ref()
                .and_then(|attachments| attachments.command_bar_flyout.as_ref()),
            true,
        );
        self.reconcile_content_dialog(
            object,
            previous_content_dialog,
            declaration
                .attachments
                .as_ref()
                .and_then(|attachments| attachments.content_dialog.as_ref()),
            true,
        );
    }

    fn mount(&mut self, declaration: &Declaration) -> ObjectId {
        let virtual_items = retain_virtual_items(declaration);
        let object = self.retained.allocate(RetainedObject {
            kind: declaration.kind,
            key: declaration.key.clone(),
            reference: declaration.reference.clone(),
            exit_transition: declaration.exit_transition,
            window_title_bar: declaration.window_title_bar,
            attachments: None,
            properties: declaration.properties.clone(),
            baselines: Vec::new(),
            events: retain_events(&declaration.events),
            relations: relation_contracts(declaration.kind)
                .iter()
                .map(|contract| RetainedRelation {
                    id: contract.id,
                    value: match contract.cardinality {
                        Cardinality::One => RetainedRelationValue::One(None),
                        Cardinality::Many => RetainedRelationValue::Many(Vec::new()),
                    },
                })
                .collect(),
            virtual_items,
        });
        if let Some(height) = declaration.window_title_bar {
            self.retained.set_window_title_bar(object, height);
        }
        if let Some(reference) = &declaration.reference {
            self.references.push(ReferenceChange::Set {
                reference: reference.clone(),
                object,
            });
        }
        self.mutations.push(Mutation::Create {
            object,
            kind: declaration.kind,
        });
        let set = properties_before_relations(declaration.kind, declaration.properties.as_slice());
        if !set.is_empty() {
            self.mutations.push(Mutation::SetProperties {
                object,
                set,
                clear: Rc::from([]),
            });
        }
        if self.retained.get(object).unwrap().events.is_some() {
            self.mutations.push(Mutation::SetEvents {
                object,
                set: Rc::from(retained_events(&self.retained.get(object).unwrap().events)),
                clear: Rc::from([]),
            });
        }
        if let Some(items) = &self.retained.get(object).unwrap().virtual_items {
            self.mutations.push(Mutation::SetVirtualSource {
                object,
                item_count: items.keys.len(),
                source_revision: items.source_revision,
            });
        }
        for contract in relation_contracts(declaration.kind) {
            self.mount_relation(object, declaration, contract);
        }
        self.apply_index(object, declaration, None, false);
        if let Some(height) = declaration.window_title_bar {
            self.mutations
                .push(Mutation::SetWindowTitleBar { object, height });
        }
        if let Some(attachments) = &declaration.attachments {
            if let Some(tooltip) = &attachments.tooltip {
                self.mount_tooltip(object, tooltip);
            }
            if let Some(flyout) = &attachments.flyout {
                self.mount_flyout(object, flyout);
            }
            if let Some(menu) = &attachments.menu {
                self.mount_menu(object, menu);
            }
            if let Some(flyout) = &attachments.command_bar_flyout {
                self.mount_command_bar_flyout(object, flyout);
            }
            if let Some(dialog) = &attachments.content_dialog {
                self.mount_content_dialog(object, dialog);
            }
        }
        object
    }

    fn mount_relation(
        &mut self,
        object: ObjectId,
        declaration: &Declaration,
        contract: &RelationContract,
    ) {
        if contract.realization == Realization::Container
            && self.retained.get(object).unwrap().virtual_items.is_some()
        {
            return;
        }
        let relation = declaration
            .relations
            .iter()
            .find(|relation| relation.id == contract.id);
        match contract.cardinality {
            Cardinality::One => {
                let child = relation
                    .and_then(|relation| match &relation.value {
                        RelationValue::One(child) => child.as_deref(),
                        RelationValue::Many(_) => unreachable!(),
                    })
                    .map(|child| self.mount(child.as_object()));
                if let Some(child) = child {
                    self.mutations.push(Mutation::Attach {
                        parent: object,
                        relation: contract.id,
                        child,
                    });
                }
                self.retained.relation_mut(object, contract.id).value =
                    RetainedRelationValue::One(child);
            }
            Cardinality::Many => {
                let children = relation
                    .map(|relation| match &relation.value {
                        RelationValue::Many(children) => children.as_slice(),
                        RelationValue::One(_) => unreachable!(),
                    })
                    .unwrap_or_default();
                let mut retained = Vec::with_capacity(children.len());
                for (index, child) in children.iter().enumerate() {
                    let child = self.mount(child.as_object());
                    self.mutations.push(Mutation::Insert {
                        parent: object,
                        relation: contract.id,
                        child,
                        index,
                    });
                    retained.push(child);
                }
                self.retained.relation_mut(object, contract.id).value =
                    RetainedRelationValue::Many(retained);
            }
        }
    }

    fn reconcile_object(&mut self, object: ObjectId, declaration: &Declaration) -> ObjectId {
        debug_assert_eq!(self.retained.get(object).unwrap().kind, declaration.kind);
        if self.retained.get(object).unwrap().reference != declaration.reference {
            if let Some(reference) = self.retained.get_mut(object).reference.take() {
                self.references
                    .push(ReferenceChange::Clear { reference, object });
            }
            if let Some(reference) = &declaration.reference {
                self.references.push(ReferenceChange::Set {
                    reference: reference.clone(),
                    object,
                });
            }
            self.retained
                .get_mut(object)
                .reference
                .clone_from(&declaration.reference);
        }
        if self.retained.get(object).unwrap().exit_transition != declaration.exit_transition {
            self.retained.get_mut(object).exit_transition = declaration.exit_transition;
        }
        let previous_title_bar = self.retained.get(object).unwrap().window_title_bar;
        if previous_title_bar != declaration.window_title_bar {
            match (previous_title_bar, declaration.window_title_bar) {
                (Some(_), None) => {
                    self.retained.clear_window_title_bar(object);
                    self.mutations
                        .push(Mutation::ClearWindowTitleBar { object });
                }
                (_, Some(height)) => {
                    self.retained.set_window_title_bar(object, height);
                    self.mutations
                        .push(Mutation::SetWindowTitleBar { object, height });
                }
                (None, None) => {}
            }
            self.retained.get_mut(object).window_title_bar = declaration.window_title_bar;
        }
        if self.retained.get(object).unwrap().attachments.is_some()
            || declaration.attachments.is_some()
        {
            let previous_tooltip = self
                .retained
                .get_mut(object)
                .attachments
                .as_mut()
                .and_then(|attachments| attachments.tooltip.take());
            self.reconcile_tooltip(
                object,
                previous_tooltip,
                declaration
                    .attachments
                    .as_ref()
                    .and_then(|attachments| attachments.tooltip.as_ref()),
                false,
            );
            let previous_flyout = self
                .retained
                .get_mut(object)
                .attachments
                .as_mut()
                .and_then(|attachments| attachments.flyout.take());
            self.reconcile_flyout(
                object,
                previous_flyout,
                declaration
                    .attachments
                    .as_ref()
                    .and_then(|attachments| attachments.flyout.as_ref()),
                false,
            );
            let previous_menu = self
                .retained
                .get_mut(object)
                .attachments
                .as_mut()
                .and_then(|attachments| attachments.menu.take());
            self.reconcile_menu(
                object,
                previous_menu,
                declaration
                    .attachments
                    .as_ref()
                    .and_then(|attachments| attachments.menu.as_ref()),
                false,
            );
            let previous_command_bar_flyout = self
                .retained
                .get_mut(object)
                .attachments
                .as_mut()
                .and_then(|attachments| attachments.command_bar_flyout.take());
            self.reconcile_command_bar_flyout(
                object,
                previous_command_bar_flyout,
                declaration
                    .attachments
                    .as_ref()
                    .and_then(|attachments| attachments.command_bar_flyout.as_ref()),
                false,
            );
            let previous_content_dialog = self
                .retained
                .get_mut(object)
                .attachments
                .as_mut()
                .and_then(|attachments| attachments.content_dialog.take());
            self.reconcile_content_dialog(
                object,
                previous_content_dialog,
                declaration
                    .attachments
                    .as_ref()
                    .and_then(|attachments| attachments.content_dialog.as_ref()),
                false,
            );
        }
        {
            let RetainedObject {
                properties: previous,
                baselines,
                ..
            } = self.retained.get_mut(object);
            if !baselines.is_empty() {
                baselines.retain(|baseline| {
                    !previous.iter().any(|property| property.id == baseline.id)
                        && !declaration
                            .properties
                            .iter()
                            .any(|property| property.id == baseline.id)
                });
            }
        }
        let properties = declaration.properties.as_slice();
        let index = index_contract(declaration.kind);
        let (previous_index, items_property_changed) = {
            let previous = &self.retained.get(object).unwrap().properties;
            let find = |properties: &[Property], id| {
                properties
                    .iter()
                    .find(|property| property.id == id)
                    .cloned()
            };
            match index {
                Some(contract) => (
                    find(previous.as_slice(), contract.property),
                    match contract.items {
                        IndexItems::Property(items) => {
                            find(previous.as_slice(), items) != find(properties, items)
                        }
                        IndexItems::Relation(_) => false,
                    },
                ),
                None => (None, false),
            }
        };
        let index = index.map(|contract| contract.property);
        if self.retained.get(object).unwrap().properties.as_slice() != properties {
            let previous = self.retained.get(object).unwrap().properties.clone();
            let changed = |property: &Property| {
                previous.iter().find(|current| current.id == property.id) != Some(property)
            };
            let clear = previous
                .iter()
                .filter(|property| {
                    Some(property.id) != index
                        && !properties.iter().any(|current| current.id == property.id)
                })
                .map(|property| property.id)
                .collect::<Rc<[_]>>();
            // A native control coerces a value to its range without always reporting it, so a
            // declared value is written again after any change to a property that coerces it.
            let coerced = properties
                .iter()
                .filter(|property| changed(property))
                .map(|property| property.id)
                .chain(clear.iter().copied())
                .filter_map(|id| coerced_property(declaration.kind, id))
                .collect::<Vec<_>>();
            let set = properties
                .iter()
                .filter(|property| {
                    Some(property.id) != index
                        && (changed(property) || coerced.contains(&property.id))
                })
                .cloned()
                .collect::<Rc<[_]>>();
            self.retained.get_mut(object).properties = declaration.properties.clone();
            if !set.is_empty() || !clear.is_empty() {
                self.mutations
                    .push(Mutation::SetProperties { object, set, clear });
            }
        }
        let events = declaration.events.as_slice();
        if retained_events(&self.retained.get(object).unwrap().events) != events {
            let previous = self.retained.get(object).unwrap().events.clone();
            let set = events
                .iter()
                .filter(|event| {
                    retained_events(&previous)
                        .iter()
                        .find(|current| current.id == event.id)
                        != Some(*event)
                })
                .cloned()
                .collect::<Rc<[_]>>();
            let clear = retained_events(&previous)
                .iter()
                .filter(|event| !events.iter().any(|current| current.id == event.id))
                .map(|event| event.id)
                .collect::<Rc<[_]>>();
            self.retained.get_mut(object).events = retain_events(&declaration.events);
            self.mutations
                .push(Mutation::SetEvents { object, set, clear });
        }
        self.reconcile_virtual_items(object, declaration);
        let start = self.mutations.len();
        for contract in relation_contracts(declaration.kind) {
            self.reconcile_relation(object, declaration, contract);
        }
        let items_changed = items_property_changed
            || index_contract(declaration.kind).is_some_and(|contract| {
                self.mutations[start..]
                    .iter()
                    .any(|mutation| match mutation {
                        Mutation::Insert {
                            parent, relation, ..
                        }
                        | Mutation::Remove {
                            parent, relation, ..
                        }
                        | Mutation::Reorder {
                            parent, relation, ..
                        } => *parent == object && contract.items == IndexItems::Relation(*relation),
                        _ => false,
                    })
            });
        self.apply_index(object, declaration, previous_index.as_ref(), items_changed);
        object
    }

    /// Applies a declaration's index property after the items it indexes, since native controls
    /// reject an index past their items. An index out of range is applied as no selection. Native
    /// controls also move or drop the selection when the items change, so the index is applied
    /// again after Reactor changes them.
    fn apply_index(
        &mut self,
        object: ObjectId,
        declaration: &Declaration,
        previous: Option<&Property>,
        items_changed: bool,
    ) {
        let Some(contract) = index_contract(declaration.kind) else {
            return;
        };
        let declared = declaration
            .properties
            .as_slice()
            .iter()
            .find(|property| property.id == contract.property);
        let (set, clear): (Rc<[Property]>, Rc<[PropertyId]>) = match declared {
            Some(declared) if items_changed || Some(declared) != previous => {
                let len = match contract.items {
                    IndexItems::Relation(relation) => self
                        .retained
                        .children(object, relation)
                        .map_or(0, <[_]>::len),
                    IndexItems::Property(items) => declaration
                        .properties
                        .as_slice()
                        .iter()
                        .find_map(|property| match &property.value {
                            PropertyValue::StringList(values) if property.id == items => {
                                Some(values.len())
                            }
                            _ => None,
                        })
                        .unwrap_or(0),
                };
                let value = match declared.value {
                    PropertyValue::SelectionIndex(Some(index)) if index < len => {
                        declared.value.clone()
                    }
                    _ if contract.requires_selection && len > 0 => {
                        PropertyValue::SelectionIndex(Some(0))
                    }
                    _ => PropertyValue::SelectionIndex(None),
                };
                let property = Property {
                    id: contract.property,
                    value,
                };
                (Rc::from([property]), Rc::from([]))
            }
            None if previous.is_some() && !contract.requires_selection => {
                (Rc::from([]), Rc::from([contract.property]))
            }
            _ => return,
        };
        self.mutations
            .push(Mutation::SetProperties { object, set, clear });
    }

    fn reconcile_relation(
        &mut self,
        object: ObjectId,
        declaration: &Declaration,
        contract: &RelationContract,
    ) {
        if contract.realization == Realization::Container
            && self.retained.get(object).unwrap().virtual_items.is_some()
        {
            return;
        }
        let relation = declaration
            .relations
            .iter()
            .find(|relation| relation.id == contract.id);
        match contract.cardinality {
            Cardinality::One => {
                let RetainedRelationValue::One(previous) =
                    self.retained.relation(object, contract.id).unwrap().value
                else {
                    unreachable!()
                };
                let desired = relation.and_then(|relation| match &relation.value {
                    RelationValue::One(child) => child.as_deref(),
                    RelationValue::Many(_) => unreachable!(),
                });
                let desired = desired.map(DeclaredNode::as_object);
                let next = match (previous, desired) {
                    (Some(previous), Some(desired))
                        if self.retained.get(previous).unwrap().kind == desired.kind =>
                    {
                        Some(self.reconcile_object(previous, desired))
                    }
                    (previous, desired) => {
                        if let Some(previous) = previous {
                            if self
                                .retained
                                .get(previous)
                                .unwrap()
                                .exit_transition
                                .is_some()
                            {
                                panic!(
                                    "exit transitions are not supported for children of relation {:?}",
                                    contract.id
                                );
                            }
                            self.mutations.push(Mutation::Detach {
                                parent: object,
                                relation: contract.id,
                                child: previous,
                            });
                            self.retire(previous);
                        }
                        let next = desired.map(|desired| self.mount(desired));
                        if let Some(next) = next {
                            self.mutations.push(Mutation::Attach {
                                parent: object,
                                relation: contract.id,
                                child: next,
                            });
                        }
                        next
                    }
                };
                if previous != next {
                    self.retained.relation_mut(object, contract.id).value =
                        RetainedRelationValue::One(next);
                }
            }
            Cardinality::Many => match contract.identity {
                Identity::Keyed => {
                    self.reconcile_keyed_many(object, contract.id, relation);
                }
                Identity::Positional => {
                    self.reconcile_positional_many(object, contract.id, relation);
                }
            },
        }
    }

    fn reconcile_virtual_items(&mut self, object: ObjectId, declaration: &Declaration) {
        let Some(previous) = self.retained.get(object).unwrap().virtual_items.clone() else {
            return;
        };
        let declared =
            declaration
                .virtual_items
                .as_deref()
                .cloned()
                .unwrap_or(DeclaredVirtualItems {
                    relation: previous.relation,
                    owner: declaration.component,
                    items: VirtualItems::Eager(Rc::default()),
                });
        assert!(
            declared.relation == previous.relation,
            "invalid relation {:?} for {:?}",
            declared.relation,
            declaration.kind
        );
        let keys = match (&previous.items, &declared.items) {
            (VirtualItems::Lazy(previous_source), VirtualItems::Lazy(next_source))
                if previous_source.key_revision == next_source.key_revision
                    && previous_source.len == next_source.len =>
            {
                Rc::clone(&previous.keys)
            }
            _ => Rc::new(virtual_keys(&declared.items)),
        };
        let items_changed = previous.items != declared.items;
        let keys_changed = previous.keys.as_ref() != keys.as_ref();
        let source_revision = if keys_changed {
            previous.source_revision.checked_add(1).unwrap()
        } else {
            previous.source_revision
        };
        let old_by_key = previous
            .containers
            .values()
            .map(|realization| (realization.key.clone(), realization.clone()))
            .collect::<HashMap<_, _>>();
        let represented = previous
            .containers
            .values()
            .filter_map(|realization| keys.get(realization.index))
            .cloned()
            .collect::<HashSet<_>>();
        let mut lease_revision = previous.lease_revision;
        let mut containers = HashMap::with_capacity(previous.containers.len());
        let mut active = HashMap::with_capacity(previous.active.len());
        let mut retired = HashSet::new();
        let mut previous_containers = previous.containers.iter().collect::<Vec<_>>();
        previous_containers
            .sort_unstable_by_key(|(container, realization)| (realization.index, container.0));
        for (container, current) in previous_containers {
            let Some(key) = keys.get(current.index).cloned() else {
                self.mutations.push(Mutation::Recycle {
                    parent: object,
                    relation: previous.relation,
                    container: *container,
                    child: current.child,
                });
                if let Some(child) = current.child
                    && !represented.contains(&current.key)
                    && retired.insert(child)
                {
                    self.retire(child);
                }
                continue;
            };
            let preserved = old_by_key.get(&key);
            let child = preserved.and_then(|realization| realization.child);
            let revision = if let Some(realization) = preserved {
                realization.revision
            } else {
                lease_revision = lease_revision.checked_add(1).unwrap();
                lease_revision
            };
            if current.key != key || current.child != child {
                self.mutations.push(Mutation::Recycle {
                    parent: object,
                    relation: previous.relation,
                    container: *container,
                    child: current.child,
                });
                if let Some(current_child) = current.child
                    && !represented.contains(&current.key)
                    && retired.insert(current_child)
                {
                    self.retire(current_child);
                }
            }
            containers.insert(
                *container,
                RetainedRealization {
                    key: key.clone(),
                    revision,
                    index: current.index,
                    child,
                },
            );
            active.insert(key, (revision, *container));
        }
        let children = {
            let mut children = containers
                .values()
                .filter_map(|realization| realization.child.map(|child| (realization.index, child)))
                .collect::<Vec<_>>();
            children.sort_unstable_by_key(|(index, _)| *index);
            children.into_iter().map(|(_, child)| child).collect()
        };
        let relation = self.retained.relation_mut(object, previous.relation);
        let RetainedRelationValue::Many(retained_children) = &mut relation.value else {
            unreachable!();
        };
        *retained_children = children;
        let retained = self
            .retained
            .get_mut(object)
            .virtual_items
            .as_mut()
            .unwrap();
        retained.owner = declared.owner.or(declaration.component);
        retained.items = declared.items;
        retained.keys = keys;
        retained.source_revision = source_revision;
        retained.lease_revision = lease_revision;
        retained.containers = containers;
        retained.active = active;
        if keys_changed {
            self.mutations.push(Mutation::SetVirtualSource {
                object,
                item_count: retained.keys.len(),
                source_revision,
            });
        }
        if (items_changed || keys_changed)
            && let Some(refreshes) = self.virtual_refreshes.as_mut()
        {
            let mut containers = retained.containers.iter().collect::<Vec<_>>();
            containers
                .sort_unstable_by_key(|(container, realization)| (realization.index, container.0));
            refreshes.extend(containers.into_iter().map(|(container, realization)| {
                RealizationRequest::Realize {
                    collection: object,
                    container: *container,
                    index: realization.index,
                    source_revision,
                }
            }));
        }
    }

    fn reconcile_keyed_many(
        &mut self,
        object: ObjectId,
        relation_id: RelationId,
        relation: Option<&DeclaredRelation>,
    ) {
        let desired = relation
            .map(|relation| match &relation.value {
                RelationValue::Many(children) => children.as_slice(),
                RelationValue::One(_) => unreachable!(),
            })
            .unwrap_or_default();
        let same_order = match &self.retained.relation(object, relation_id).unwrap().value {
            RetainedRelationValue::Many(previous) if previous.len() == desired.len() => {
                let mut same_order = true;
                for (previous, desired) in previous.iter().zip(desired) {
                    let previous = self.retained.get(*previous).unwrap();
                    let desired = desired.as_object();
                    if previous.kind != desired.kind || previous.key != desired.key {
                        same_order = false;
                        break;
                    }
                }
                same_order
            }
            _ => false,
        };
        if same_order {
            for (index, desired) in desired.iter().enumerate() {
                let previous = match &self.retained.relation(object, relation_id).unwrap().value {
                    RetainedRelationValue::Many(previous) => previous[index],
                    _ => unreachable!(),
                };
                self.reconcile_object(previous, desired.as_object());
            }
            return;
        }
        let desired = desired
            .iter()
            .map(DeclaredNode::as_object)
            .collect::<Vec<_>>();
        let previous = match &self.retained.relation(object, relation_id).unwrap().value {
            RetainedRelationValue::Many(children) => children.clone(),
            _ => unreachable!(),
        };
        let mut by_key = HashMap::with_capacity(previous.len());
        for (index, child) in previous.iter().enumerate() {
            by_key.insert(
                self.retained.get(*child).unwrap().key.clone().unwrap(),
                (*child, index),
            );
        }

        let mut matched = Vec::with_capacity(desired.len());
        let mut removed = Vec::new();
        for desired in desired.iter().copied() {
            let key = desired.key.clone().unwrap();
            let previous = match by_key.remove(&key) {
                Some((previous, _))
                    if self.retained.get(previous).unwrap().kind == desired.kind =>
                {
                    Some(previous)
                }
                Some(previous) => {
                    removed.push(previous);
                    None
                }
                None => None,
            };
            matched.push((previous, desired));
        }
        removed.extend(by_key.into_values());
        removed.sort_unstable_by_key(|(_, index)| std::cmp::Reverse(*index));
        let mut current = previous.clone();
        for (child, index) in removed {
            if !self.retire_with_transition(object, relation_id, child) {
                self.mutations.push(Mutation::Remove {
                    parent: object,
                    relation: relation_id,
                    child,
                    index,
                });
                self.retire(child);
            }
            current.remove(index);
        }

        let mut next = Vec::with_capacity(desired.len());
        for (index, (previous, desired)) in matched.into_iter().enumerate() {
            let child = if let Some(previous) = previous {
                self.reconcile_object(previous, desired)
            } else {
                let child = self.mount(desired);
                self.mutations.push(Mutation::Insert {
                    parent: object,
                    relation: relation_id,
                    child,
                    index,
                });
                current.insert(index, child);
                child
            };
            next.push(child);
        }
        let moves = reorder_moves(&current, &next);
        if !moves.is_empty() {
            self.mutations.push(Mutation::Reorder {
                parent: object,
                relation: relation_id,
                moves,
                children: next.clone(),
            });
        }
        if previous != next {
            self.retained.relation_mut(object, relation_id).value =
                RetainedRelationValue::Many(next);
        }

        fn reorder_moves(previous: &[ObjectId], next: &[ObjectId]) -> Vec<Move> {
            if previous == next {
                return Vec::new();
            }
            let previous_indices = previous
                .iter()
                .copied()
                .enumerate()
                .map(|(index, child)| (child, index))
                .collect::<HashMap<_, _>>();
            let retained = next
                .iter()
                .enumerate()
                .filter_map(|(next_index, child)| {
                    previous_indices
                        .get(child)
                        .map(|previous_index| (next_index, *previous_index))
                })
                .collect::<Vec<_>>();
            let stable = longest_increasing_positions(&retained, next.len());
            let mut moves = Vec::new();

            for next_index in (0..next.len()).rev() {
                if stable[next_index] {
                    continue;
                }
                let child = next[next_index];
                moves.push(Move {
                    child,
                    before: next.get(next_index + 1).copied(),
                });
            }
            moves
        }

        fn longest_increasing_positions(sequence: &[(usize, usize)], len: usize) -> Vec<bool> {
            let mut tails = Vec::<usize>::new();
            let mut predecessors = vec![None; sequence.len()];

            for (sequence_index, &(_, value)) in sequence.iter().enumerate() {
                let position = tails.partition_point(|tail| sequence[*tail].1 < value);
                if position > 0 {
                    predecessors[sequence_index] = Some(tails[position - 1]);
                }
                if position == tails.len() {
                    tails.push(sequence_index);
                } else {
                    tails[position] = sequence_index;
                }
            }

            let mut positions = vec![false; len];
            let mut cursor = tails.last().copied();
            while let Some(sequence_index) = cursor {
                positions[sequence[sequence_index].0] = true;
                cursor = predecessors[sequence_index];
            }
            positions
        }
    }

    fn reconcile_positional_many(
        &mut self,
        object: ObjectId,
        relation_id: RelationId,
        relation: Option<&DeclaredRelation>,
    ) {
        let desired = relation
            .map(|relation| match &relation.value {
                RelationValue::Many(children) => children.as_slice(),
                RelationValue::One(_) => unreachable!(),
            })
            .unwrap_or_default();
        let same_order = match &self.retained.relation(object, relation_id).unwrap().value {
            RetainedRelationValue::Many(previous) if previous.len() == desired.len() => {
                let mut same_order = true;
                for (previous, desired) in previous.iter().zip(desired) {
                    if self.retained.get(*previous).unwrap().kind != desired.as_object().kind {
                        same_order = false;
                        break;
                    }
                }
                same_order
            }
            _ => false,
        };
        if same_order {
            for (index, desired) in desired.iter().enumerate() {
                let previous = match &self.retained.relation(object, relation_id).unwrap().value {
                    RetainedRelationValue::Many(previous) => previous[index],
                    _ => unreachable!(),
                };
                self.reconcile_object(previous, desired.as_object());
            }
            return;
        }
        let desired = desired
            .iter()
            .map(DeclaredNode::as_object)
            .collect::<Vec<_>>();
        let previous = match &self.retained.relation(object, relation_id).unwrap().value {
            RetainedRelationValue::Many(children) => children.clone(),
            _ => unreachable!(),
        };
        let mut next = Vec::with_capacity(desired.len());
        for (index, desired) in desired.iter().copied().enumerate() {
            let child = if let Some(previous) = previous.get(index).copied() {
                if self.retained.get(previous).unwrap().kind == desired.kind {
                    self.reconcile_object(previous, desired)
                } else {
                    if !self.retire_with_transition(object, relation_id, previous) {
                        self.mutations.push(Mutation::Remove {
                            parent: object,
                            relation: relation_id,
                            child: previous,
                            index,
                        });
                        self.retire(previous);
                    }
                    let child = self.mount(desired);
                    self.mutations.push(Mutation::Insert {
                        parent: object,
                        relation: relation_id,
                        child,
                        index,
                    });
                    child
                }
            } else {
                let child = self.mount(desired);
                self.mutations.push(Mutation::Insert {
                    parent: object,
                    relation: relation_id,
                    child,
                    index,
                });
                child
            };
            next.push(child);
        }
        for (index, child) in previous
            .iter()
            .enumerate()
            .skip(desired.len())
            .rev()
            .map(|(index, child)| (index, *child))
        {
            if !self.retire_with_transition(object, relation_id, child) {
                self.mutations.push(Mutation::Remove {
                    parent: object,
                    relation: relation_id,
                    child,
                    index,
                });
                self.retire(child);
            }
        }
        if previous != next {
            self.retained.relation_mut(object, relation_id).value =
                RetainedRelationValue::Many(next);
        }
    }

    fn retire(&mut self, object: ObjectId) {
        for (root, nodes) in self.retained.retirements_for_parent(object) {
            self.mutations
                .push(Mutation::CompleteRetirement { root, nodes });
            self.retained.complete_retirement(root);
        }
        if let Some(reference) = self.retained.get_mut(object).reference.take() {
            self.references
                .push(ReferenceChange::Clear { reference, object });
        }
        if self
            .retained
            .get_mut(object)
            .window_title_bar
            .take()
            .is_some()
        {
            self.retained.clear_window_title_bar(object);
            self.mutations
                .push(Mutation::ClearWindowTitleBar { object });
        }
        self.clear_content_dialog(object);
        self.clear_command_bar_flyout(object);
        self.clear_menu(object);
        self.clear_flyout(object);
        self.clear_tooltip(object);
        let virtual_items = self.retained.get_mut(object).virtual_items.take();
        let relations = std::mem::take(&mut self.retained.get_mut(object).relations);
        for relation in relations {
            match relation.value {
                RetainedRelationValue::One(Some(child)) => {
                    self.mutations.push(Mutation::Detach {
                        parent: object,
                        relation: relation.id,
                        child,
                    });
                    self.retire(child);
                }
                RetainedRelationValue::Many(children) => {
                    if let Some(items) = virtual_items
                        .as_ref()
                        .filter(|items| items.relation == relation.id)
                    {
                        let mut containers = items.containers.iter().collect::<Vec<_>>();
                        containers.sort_unstable_by_key(|(container, realization)| {
                            (realization.index, container.0)
                        });
                        for (container, realization) in containers {
                            self.mutations.push(Mutation::Recycle {
                                parent: object,
                                relation: relation.id,
                                container: *container,
                                child: realization.child,
                            });
                        }
                        for child in children.into_iter().rev() {
                            self.retire(child);
                        }
                    } else {
                        for (index, child) in children.into_iter().enumerate().rev() {
                            self.mutations.push(Mutation::Remove {
                                parent: object,
                                relation: relation.id,
                                child,
                                index,
                            });
                            self.retire(child);
                        }
                    }
                }
                RetainedRelationValue::One(None) => {}
            }
        }
        self.mutations.push(Mutation::Destroy { object });
        self.retained.remove(object);
    }

    fn retire_with_transition(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        root: ObjectId,
    ) -> bool {
        let Some(transition) = self.retained.get(root).unwrap().exit_transition else {
            return false;
        };
        let contract = relation_contracts(self.retained.get(parent).unwrap().kind)
            .iter()
            .find(|contract| contract.id == relation)
            .unwrap();
        // A retiring child keeps its native slot and stays selectable, so it cannot fade out of a
        // selector's items.
        let kind = self.retained.get(parent).unwrap().kind;
        if contract.cardinality != Cardinality::Many
            || contract.realization != Realization::Owned
            || index_for_items(kind, IndexItems::Relation(relation)).is_some()
            || selection_for_relation(kind, relation).is_some()
        {
            panic!("exit transitions are not supported for children of relation {relation:?}");
        }
        let mut nodes = Vec::new();
        self.collect_subtree_postorder(root, &mut nodes);
        for object in nodes.iter().copied() {
            self.clear_content_dialog(object);
            self.clear_command_bar_flyout(object);
            self.clear_menu(object);
            self.clear_flyout(object);
            self.clear_tooltip(object);
        }
        for object in &nodes {
            let retained = self.retained.get_mut(*object);
            if let Some(reference) = retained.reference.take() {
                self.references.push(ReferenceChange::Clear {
                    reference,
                    object: *object,
                });
            }
            if let Some(events) = retained.events.take() {
                self.mutations.push(Mutation::SetEvents {
                    object: *object,
                    set: Rc::from([]),
                    clear: events.iter().map(|event| event.id).collect(),
                });
            }
            if retained.window_title_bar.take().is_some() {
                self.retained.clear_window_title_bar(*object);
                self.mutations
                    .push(Mutation::ClearWindowTitleBar { object: *object });
            }
        }
        self.mutations.push(Mutation::Retire {
            root,
            nodes: nodes.clone(),
            parent,
            relation,
            duration: transition.duration(),
        });
        self.retained.mark_retiring(root, nodes, parent);
        true
    }

    fn collect_subtree_postorder(&self, object: ObjectId, nodes: &mut Vec<ObjectId>) {
        for relation in &self.retained.get(object).unwrap().relations {
            match &relation.value {
                RetainedRelationValue::One(Some(child)) => {
                    self.collect_subtree_postorder(*child, nodes);
                }
                RetainedRelationValue::Many(children) => {
                    for child in children {
                        self.collect_subtree_postorder(*child, nodes);
                    }
                }
                RetainedRelationValue::One(None) => {}
            }
        }
        nodes.push(object);
    }
}

#[cfg(any(test, feature = "test"))]
#[path = "test_support/reconcile.rs"]
mod test_support;
#[cfg(any(test, feature = "test"))]
pub use test_support::*;
