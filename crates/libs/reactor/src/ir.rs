use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObjectCategory {
    Visual,
    Structural,
    Data,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cardinality {
    One,
    Many,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Identity {
    Positional,
    Keyed,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Realization {
    Owned,
    Structural,
    Container,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PropertyContract {
    pub id: PropertyId,
    pub value: ValueType,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventContract {
    pub id: EventId,
    pub value: ValueType,
}
/// A relation whose children the native control can reorder, reported through `event`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReorderContract {
    pub relation: RelationId,
    pub event: EventId,
}

pub fn reorder_event(kind: ObjectType, relation: RelationId) -> Option<EventId> {
    reorder_contracts(kind)
        .iter()
        .find(|contract| contract.relation == relation)
        .map(|contract| contract.event)
}

/// The collection an index property counts: a child relation or a list property.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IndexItems {
    Relation(RelationId),
    Property(PropertyId),
}

/// A controlled property whose value indexes `items`. Reactor applies it after `items` changes,
/// and the native `event` raised by changing `items` is not reported when the index is applied
/// afterwards. An index out of range is applied as no selection, or as the first item when the
/// control `requires_selection` and has items. A control that applies the index only to realized
/// item containers lists `realize_items`, and the index is written after measuring the control.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IndexContract {
    pub property: PropertyId,
    pub items: IndexItems,
    pub requires_selection: bool,
    pub realize_items: bool,
    pub event: EventId,
}

pub fn index_for_items(kind: ObjectType, items: IndexItems) -> Option<IndexContract> {
    index_contract(kind).filter(|contract| contract.items == items)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueType {
    String,
    Bool,
    Brush,
    ButtonStyle,
    ContentDialogResult,
    CharacterEventInfo,
    Color,
    CornerRadius,
    DateTime,
    Duration,
    DragDropPolicy,
    DragKind,
    DroppedData,
    F64,
    FocusEventInfo,
    FontWeight,
    GridLengths,
    I32,
    Icon,
    ImageSource,
    Key,
    KeyAccelerators,
    KeyEventInfo,
    Keys,
    NavigationViewDisplayMode,
    OptionalBool,
    OptionalDateTime,
    OptionalF64,
    OptionalTimeSpan,
    PointerEventInfo,
    ResourceOverrides,
    RichText,
    Selection,
    ThemeTransitions,
    Thickness,
    Vector2,
    SelectionIndex,
    StringList,
    Enum {
        kind: &'static str,
        variants: &'static [&'static str],
    },
    Unit,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RelationContract {
    pub id: RelationId,
    pub child: ObjectCategory,
    pub allowed_objects: &'static [ObjectType],
    pub cardinality: Cardinality,
    pub identity: Identity,
    pub realization: Realization,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectionContract {
    pub relations: &'static [RelationId],
    pub item: ObjectType,
    pub selected_property: PropertyId,
    pub event: EventId,
    pub payload_property: PropertyId,
}

pub fn selection_for_relation(kind: ObjectType, relation: RelationId) -> Option<SelectionContract> {
    selection_contract(kind).filter(|selection| selection.relations.contains(&relation))
}
pub fn selection_for_item_property(
    owner: ObjectType,
    relation: RelationId,
    item: ObjectType,
    property: PropertyId,
) -> Option<SelectionContract> {
    selection_for_relation(owner, relation)
        .filter(|selection| selection.item == item && selection.selected_property == property)
}
pub fn relation_accepts(contract: &RelationContract, kind: ObjectType) -> bool {
    object_category(kind) == contract.child
        && (contract.allowed_objects.is_empty() || contract.allowed_objects.contains(&kind))
}

#[cfg(test)]
pub fn property_contracts(kind: ObjectType) -> &'static [PropertyContract] {
    static CONTRACTS: std::sync::OnceLock<Vec<&'static [PropertyContract]>> =
        std::sync::OnceLock::new();
    let contracts = CONTRACTS.get_or_init(|| {
        let mut unique = Vec::<&'static [PropertyContract]>::new();
        ALL_OBJECT_TYPES
            .iter()
            .copied()
            .map(|kind| {
                let mut current = ALL_PROPERTY_IDS
                    .iter()
                    .copied()
                    .filter_map(|id| property_contract(kind, id))
                    .collect::<Vec<_>>();
                current.sort_by_key(|contract| property_order(kind, contract.id));
                if let Some(existing) = unique
                    .iter()
                    .copied()
                    .find(|existing| *existing == current.as_slice())
                {
                    existing
                } else {
                    let current: &'static [PropertyContract] =
                        Box::leak(current.into_boxed_slice());
                    unique.push(current);
                    current
                }
            })
            .collect()
    });
    contracts[object_index(kind)]
}

pub(crate) const MAX_DEPTH: usize = 128;
pub(crate) const MAX_OBJECTS: usize = 65_536;

#[derive(Default)]
pub(crate) struct DeclarationValidator {
    references: HashSet<usize>,
    keys: HashSet<Key>,
}

impl DeclarationValidator {
    /// Checks a declaration against the object contracts and returns its object count.
    ///
    /// Panics when the declaration violates a contract.
    pub(crate) fn validate(&mut self, root: &Declaration) -> usize {
        self.references.clear();
        self.keys.clear();
        let mut objects = 0;
        validate_object(
            root,
            0,
            &mut objects,
            &mut self.references,
            &mut self.keys,
            true,
            None,
        );
        objects
    }
}

fn validate_key(keys: &mut HashSet<Key>, key: &Key) {
    assert!(keys.insert(key.clone()), "duplicate key {key:?}");
}

pub(crate) fn validate_menu_items(items: &[MenuItem], keys: &mut HashSet<Key>) {
    for item in items {
        let (key, children) = match item {
            MenuItem::Item { key, .. } | MenuItem::Separator { key } => (key, None),
            MenuItem::Submenu { key, items, .. } => (key, Some(items.as_slice())),
        };
        validate_key(keys, key);
        if let Some(children) = children {
            validate_menu_items(children, keys);
        }
    }
}

fn validate_commands(commands: &[CommandBarCommand], keys: &mut HashSet<Key>) {
    for command in commands {
        let key = match command {
            CommandBarCommand::Button { key, .. } | CommandBarCommand::Separator { key } => key,
        };
        validate_key(keys, key);
    }
}

pub(crate) fn validate_limits(depth: usize, objects: usize) {
    assert!(
        depth <= MAX_DEPTH,
        "declaration exceeds the maximum depth of {MAX_DEPTH}"
    );
    assert!(
        objects < MAX_OBJECTS,
        "declaration exceeds the maximum of {MAX_OBJECTS} objects"
    );
}

fn invalid_attachment(kind: ObjectType) -> ! {
    panic!("invalid attachment on {kind:?}")
}

fn validate_object(
    declaration: &Declaration,
    depth: usize,
    objects: &mut usize,
    references: &mut HashSet<usize>,
    keys: &mut HashSet<Key>,
    content_dialog_allowed: bool,
    internal_kind: Option<ObjectType>,
) {
    validate_limits(depth, *objects);
    *objects += 1;
    if matches!(
        declaration.kind,
        ObjectType::ToolTip | ObjectType::ContentDialog
    ) && internal_kind != Some(declaration.kind)
    {
        invalid_attachment(declaration.kind);
    }
    assert!(
        declaration
            .reference
            .as_ref()
            .is_none_or(|reference| references.insert(reference.identity())),
        "element reference declared more than once"
    );

    if let Some(attachments) = &declaration.attachments {
        if let Some(tooltip) = &attachments.tooltip {
            validate_limits(depth + 1, *objects);
            *objects += 1;
            validate_object(
                child_object(&tooltip.content),
                depth + 2,
                objects,
                references,
                keys,
                false,
                None,
            );
        }
        if let Some(flyout) = &attachments.flyout {
            if !matches!(
                declaration.kind,
                ObjectType::Button | ObjectType::SplitButton
            ) {
                invalid_attachment(declaration.kind);
            }
            validate_limits(depth + 1, *objects);
            *objects += 1;
            validate_object(
                child_object(&flyout.content),
                depth + 2,
                objects,
                references,
                keys,
                false,
                None,
            );
        }
        if let Some(menu) = &attachments.menu {
            if !matches!(
                declaration.kind,
                ObjectType::Button | ObjectType::DropDownButton | ObjectType::MenuBarItem
            ) {
                invalid_attachment(declaration.kind);
            }
            validate_menu_items(&menu.menu.items, &mut HashSet::new());
        }
        if let Some(flyout) = &attachments.command_bar_flyout {
            if declaration.kind != ObjectType::Button {
                invalid_attachment(declaration.kind);
            }
            let mut keys = HashSet::new();
            validate_commands(&flyout.flyout.primary, &mut keys);
            validate_commands(&flyout.flyout.secondary, &mut keys);
        }
        if attachments.flyout.is_some()
            && (attachments.menu.is_some() || attachments.command_bar_flyout.is_some())
            || attachments.menu.is_some() && attachments.command_bar_flyout.is_some()
        {
            invalid_attachment(declaration.kind);
        }
        if let Some(dialog) = &attachments.content_dialog {
            if !content_dialog_allowed {
                invalid_attachment(ObjectType::ContentDialog);
            }
            validate_object(
                &dialog.declaration,
                depth + 1,
                objects,
                references,
                keys,
                false,
                Some(ObjectType::ContentDialog),
            );
        }
    }

    for property in declaration.properties.iter() {
        validate_property(declaration.kind, property);
    }

    for event in declaration.events.iter() {
        let Some(contract) = event_contracts(declaration.kind)
            .iter()
            .find(|contract| contract.id == event.id)
        else {
            panic!("invalid event {:?} for {:?}", event.id, declaration.kind);
        };
        let valid = matches!(
            (contract.value, &event.value),
            (ValueType::Bool, EventValue::Bool(_))
                | (ValueType::Color, EventValue::Color(_))
                | (
                    ValueType::CharacterEventInfo,
                    EventValue::CharacterEventInfo(_)
                )
                | (
                    ValueType::ContentDialogResult,
                    EventValue::ContentDialogResult(_)
                )
                | (ValueType::String, EventValue::String(_))
                | (ValueType::F64, EventValue::F64(_))
                | (ValueType::FocusEventInfo, EventValue::FocusEventInfo(_))
                | (ValueType::DragKind, EventValue::DragKind(_))
                | (ValueType::DroppedData, EventValue::DroppedData(_))
                | (ValueType::OptionalBool, EventValue::OptionalBool(_))
                | (ValueType::OptionalDateTime, EventValue::OptionalDateTime(_))
                | (ValueType::OptionalF64, EventValue::OptionalF64(_))
                | (ValueType::OptionalTimeSpan, EventValue::OptionalTimeSpan(_))
                | (
                    ValueType::NavigationViewDisplayMode,
                    EventValue::NavigationViewDisplayMode(_)
                )
                | (ValueType::PointerEventInfo, EventValue::PointerEventInfo(_))
                | (ValueType::KeyEventInfo, EventValue::KeyEventInfo(_))
                | (ValueType::Selection, EventValue::Selection(_))
                | (ValueType::SelectionIndex, EventValue::SelectionIndex(_))
                | (ValueType::Key, EventValue::Key(_))
                | (ValueType::Keys, EventValue::Keys(_))
                | (ValueType::Unit, EventValue::Unit(_))
        );
        assert!(valid, "invalid callback for event {:?}", event.id);
    }

    if let Some(virtual_items) = &declaration.virtual_items {
        assert!(
            relation_contracts(declaration.kind).iter().any(|contract| {
                contract.id == virtual_items.relation
                    && contract.cardinality == Cardinality::Many
                    && contract.identity == Identity::Keyed
                    && contract.realization == Realization::Container
                    && contract.child == ObjectCategory::Visual
            }),
            "invalid relation {:?} for {:?}",
            virtual_items.relation,
            declaration.kind
        );
    }

    for relation in declaration.relations.iter() {
        let Some(contract) = relation_contracts(declaration.kind)
            .iter()
            .find(|contract| contract.id == relation.id)
        else {
            panic!(
                "invalid relation {:?} for {:?}",
                relation.id, declaration.kind
            );
        };
        let cardinality = match &relation.value {
            RelationValue::One(_) => Cardinality::One,
            RelationValue::Many(_) => Cardinality::Many,
        };
        assert!(
            contract.cardinality == cardinality,
            "invalid cardinality for relation {:?}",
            relation.id
        );
        match &relation.value {
            RelationValue::One(child) => {
                if let Some(child) = child {
                    let child = child_object(child);
                    validate_child(contract, child);
                    validate_object(
                        child,
                        depth + 1,
                        objects,
                        references,
                        keys,
                        content_dialog_allowed,
                        None,
                    );
                }
            }
            RelationValue::Many(children) => {
                if contract.identity == Identity::Keyed {
                    keys.clear();
                    keys.reserve(children.len());
                    for child in children.iter() {
                        let Some(key) = &child_object(child).key else {
                            panic!("children of relation {:?} require keys", relation.id);
                        };
                        validate_key(keys, key);
                    }
                }
                for child in children.iter() {
                    let child = child_object(child);
                    validate_child(contract, child);
                    validate_object(
                        child,
                        depth + 1,
                        objects,
                        references,
                        keys,
                        content_dialog_allowed,
                        None,
                    );
                }
            }
        }
    }
}

fn child_object(child: &DeclaredNode) -> &Declaration {
    child.as_object()
}

pub(crate) fn validate_property(kind: ObjectType, property: &Property) {
    let Some(contract) = property_contract(kind, property.id) else {
        panic!("invalid property {:?} for {kind:?}", property.id);
    };
    let valid = match (contract.value, &property.value) {
        (ValueType::String, PropertyValue::String(_))
        | (ValueType::Bool, PropertyValue::Bool(_))
        | (ValueType::Brush, PropertyValue::Brush(_))
        | (ValueType::ButtonStyle, PropertyValue::ButtonStyle(_))
        | (ValueType::Color, PropertyValue::Color(_))
        | (ValueType::CornerRadius, PropertyValue::CornerRadius(_))
        | (ValueType::DateTime, PropertyValue::DateTime(_))
        | (ValueType::Duration, PropertyValue::Duration(_))
        | (ValueType::DragDropPolicy, PropertyValue::DragDropPolicy(_))
        | (ValueType::F64, PropertyValue::F64(_))
        | (ValueType::FontWeight, PropertyValue::FontWeight(_))
        | (ValueType::GridLengths, PropertyValue::GridLengths(_))
        | (ValueType::I32, PropertyValue::I32(_))
        | (ValueType::Icon, PropertyValue::Icon(_))
        | (ValueType::ImageSource, PropertyValue::ImageSource(_))
        | (ValueType::KeyAccelerators, PropertyValue::KeyAccelerators(_))
        | (ValueType::OptionalF64, PropertyValue::OptionalF64(_))
        | (ValueType::OptionalBool, PropertyValue::OptionalBool(_))
        | (ValueType::OptionalDateTime, PropertyValue::OptionalDateTime(_))
        | (ValueType::ResourceOverrides, PropertyValue::ResourceOverrides(_))
        | (ValueType::RichText, PropertyValue::RichText(_))
        | (ValueType::SelectionIndex, PropertyValue::SelectionIndex(_))
        | (ValueType::StringList, PropertyValue::StringList(_))
        | (ValueType::ThemeTransitions, PropertyValue::ThemeTransitions(_))
        | (ValueType::Thickness, PropertyValue::Thickness(_)) => true,
        (ValueType::Vector2, PropertyValue::Vector2(value)) => {
            value.iter().all(|value| (*value as f32).is_finite())
        }
        (
            ValueType::Enum {
                kind: expected,
                variants,
            },
            PropertyValue::Enum { kind, variant },
        ) => expected == *kind && variants.contains(variant),
        _ => false,
    };
    assert!(valid, "invalid value for property {:?}", property.id);
}

fn validate_child(contract: &RelationContract, child: &Declaration) {
    assert!(
        relation_accepts(contract, child.kind),
        "{:?} is not a valid child of relation {:?}",
        child.kind,
        contract.id
    );
}
