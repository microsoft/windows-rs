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
pub enum SelectionEventItemSource {
    Owner,
    EventArgs,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectionContract {
    pub relations: &'static [RelationId],
    pub item: ObjectType,
    pub selected_property: PropertyId,
    pub event: EventId,
    pub event_item_source: SelectionEventItemSource,
    pub event_args: Option<&'static str>,
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

#[cfg(any(test, feature = "test"))]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GraphError {
    DuplicateKey(Key),
    StaleObject(ObjectId),
    RootTypeChanged {
        previous: ObjectType,
        next: ObjectType,
    },
    InvalidProperty(ObjectType, PropertyId),
    InvalidPropertyValue(PropertyId),
    InvalidEvent(ObjectType, EventId),
    InvalidEventValue(EventId),
    InvalidSelection(ObjectType),
    InvalidRealization(ObjectId, usize),
    InvalidFocus(ObjectType),
    ReferenceUnavailable,
    DuplicateReference,
    DuplicateWindowTitleBar,
    InvalidAttachment(ObjectType),
    ExitTransitionUnsupported,
    InvalidRelation(ObjectType, RelationId),
    MissingChild(RelationId, ObjectId),
    InvalidChildCategory(RelationId),
    InvalidCardinality(RelationId),
    MissingKey(RelationId),
    UnresolvedComponent,
    DepthExceeded,
    SizeExceeded,
}

#[derive(Default)]
pub(crate) struct DeclarationValidator {
    references: HashSet<usize>,
    keys: HashSet<Key>,
}

impl DeclarationValidator {
    pub(crate) fn validate(&mut self, root: &Declaration) -> Result<usize, GraphError> {
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
        )?;
        Ok(objects)
    }
}

fn validate_menu_items(items: &[MenuItem], keys: &mut HashSet<Key>) -> Result<(), GraphError> {
    for item in items {
        let (key, children) = match item {
            MenuItem::Item { key, .. } | MenuItem::Separator { key } => (key, None),
            MenuItem::Submenu { key, items, .. } => (key, Some(items.as_slice())),
        };
        if !keys.insert(key.clone()) {
            return Err(GraphError::DuplicateKey(key.clone()));
        }
        if let Some(children) = children {
            validate_menu_items(children, keys)?;
        }
    }
    Ok(())
}

fn validate_commands(
    commands: &[CommandBarCommand],
    keys: &mut HashSet<Key>,
) -> Result<(), GraphError> {
    for command in commands {
        let key = match command {
            CommandBarCommand::Button { key, .. } | CommandBarCommand::Separator { key } => key,
        };
        if !keys.insert(key.clone()) {
            return Err(GraphError::DuplicateKey(key.clone()));
        }
    }
    Ok(())
}

fn validate_object(
    declaration: &Declaration,
    depth: usize,
    objects: &mut usize,
    references: &mut HashSet<usize>,
    keys: &mut HashSet<Key>,
    content_dialog_allowed: bool,
    internal_kind: Option<ObjectType>,
) -> Result<(), GraphError> {
    if depth > MAX_DEPTH {
        return Err(GraphError::DepthExceeded);
    }
    if *objects >= MAX_OBJECTS {
        return Err(GraphError::SizeExceeded);
    }
    *objects += 1;
    if matches!(
        declaration.kind,
        ObjectType::ToolTip | ObjectType::ContentDialog
    ) && internal_kind != Some(declaration.kind)
    {
        return Err(GraphError::InvalidAttachment(declaration.kind));
    }
    if declaration
        .reference
        .as_ref()
        .is_some_and(|reference| !references.insert(reference.identity()))
    {
        return Err(GraphError::DuplicateReference);
    }

    if let Some(attachments) = &declaration.attachments {
        if let Some(tooltip) = &attachments.tooltip {
            if depth >= MAX_DEPTH || *objects >= MAX_OBJECTS {
                return Err(if depth >= MAX_DEPTH {
                    GraphError::DepthExceeded
                } else {
                    GraphError::SizeExceeded
                });
            }
            *objects += 1;
            validate_object(
                child_object(&tooltip.content)?,
                depth + 2,
                objects,
                references,
                keys,
                false,
                None,
            )?;
        }
        if let Some(flyout) = &attachments.flyout {
            if !matches!(
                declaration.kind,
                ObjectType::Button | ObjectType::SplitButton
            ) {
                return Err(GraphError::InvalidAttachment(declaration.kind));
            }
            if depth >= MAX_DEPTH || *objects >= MAX_OBJECTS {
                return Err(if depth >= MAX_DEPTH {
                    GraphError::DepthExceeded
                } else {
                    GraphError::SizeExceeded
                });
            }
            *objects += 1;
            validate_object(
                child_object(&flyout.content)?,
                depth + 2,
                objects,
                references,
                keys,
                false,
                None,
            )?;
        }
        if let Some(menu) = &attachments.menu {
            if !matches!(
                declaration.kind,
                ObjectType::Button | ObjectType::DropDownButton | ObjectType::MenuBarItem
            ) {
                return Err(GraphError::InvalidAttachment(declaration.kind));
            }
            validate_menu_items(&menu.menu.items, &mut HashSet::new())?;
        }
        if let Some(flyout) = &attachments.command_bar_flyout {
            if declaration.kind != ObjectType::Button {
                return Err(GraphError::InvalidAttachment(declaration.kind));
            }
            let mut keys = HashSet::new();
            validate_commands(&flyout.flyout.primary, &mut keys)?;
            validate_commands(&flyout.flyout.secondary, &mut keys)?;
        }
        if attachments.flyout.is_some()
            && (attachments.menu.is_some() || attachments.command_bar_flyout.is_some())
            || attachments.menu.is_some() && attachments.command_bar_flyout.is_some()
        {
            return Err(GraphError::InvalidAttachment(declaration.kind));
        }
        if let Some(dialog) = &attachments.content_dialog {
            if !content_dialog_allowed {
                return Err(GraphError::InvalidAttachment(ObjectType::ContentDialog));
            }
            validate_object(
                &dialog.declaration,
                depth + 1,
                objects,
                references,
                keys,
                false,
                Some(ObjectType::ContentDialog),
            )?;
        }
    }

    for property in declaration.properties.iter() {
        validate_property(declaration.kind, property)?;
    }

    for event in declaration.events.iter() {
        let contract = event_contracts(declaration.kind)
            .iter()
            .find(|contract| contract.id == event.id)
            .ok_or(GraphError::InvalidEvent(declaration.kind, event.id))?;
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
        if !valid {
            return Err(GraphError::InvalidEventValue(event.id));
        }
    }

    if let Some(virtual_items) = &declaration.virtual_items {
        let Some(contract) = relation_contracts(declaration.kind)
            .iter()
            .find(|contract| contract.id == virtual_items.relation)
        else {
            return Err(GraphError::InvalidRelation(
                declaration.kind,
                virtual_items.relation,
            ));
        };
        if contract.cardinality != Cardinality::Many
            || contract.identity != Identity::Keyed
            || contract.realization != Realization::Container
            || contract.child != ObjectCategory::Visual
        {
            return Err(GraphError::InvalidRelation(
                declaration.kind,
                virtual_items.relation,
            ));
        }
    }

    for relation in declaration.relations.iter() {
        let contract = relation_contracts(declaration.kind)
            .iter()
            .find(|contract| contract.id == relation.id)
            .ok_or(GraphError::InvalidRelation(declaration.kind, relation.id))?;
        match &relation.value {
            RelationValue::One(child) => {
                if contract.cardinality != Cardinality::One {
                    return Err(GraphError::InvalidCardinality(relation.id));
                }
                if let Some(child) = child {
                    let child = child_object(child)?;
                    validate_child(contract, child)?;
                    validate_object(
                        child,
                        depth + 1,
                        objects,
                        references,
                        keys,
                        content_dialog_allowed,
                        None,
                    )?;
                }
            }
            RelationValue::Many(children) => {
                if contract.cardinality != Cardinality::Many {
                    return Err(GraphError::InvalidCardinality(relation.id));
                }
                if contract.identity == Identity::Keyed {
                    keys.clear();
                    keys.reserve(children.len());
                    for child in children.iter() {
                        let key = child_object(child)?
                            .key
                            .as_ref()
                            .ok_or(GraphError::MissingKey(relation.id))?;
                        if !keys.insert(key.clone()) {
                            return Err(GraphError::DuplicateKey(key.clone()));
                        }
                    }
                }
                for child in children.iter() {
                    let child = child_object(child)?;
                    validate_child(contract, child)?;
                    validate_object(
                        child,
                        depth + 1,
                        objects,
                        references,
                        keys,
                        content_dialog_allowed,
                        None,
                    )?;
                }
            }
        }
    }
    Ok(())
}

fn child_object(child: &DeclaredNode) -> Result<&Declaration, GraphError> {
    match child {
        DeclaredNode::Object(child) => Ok(child),
        DeclaredNode::Component { .. } | DeclaredNode::Provider(_) => {
            Err(GraphError::UnresolvedComponent)
        }
    }
}

pub(crate) fn validate_property(kind: ObjectType, property: &Property) -> Result<(), GraphError> {
    let contract = property_contract(kind, property.id)
        .ok_or(GraphError::InvalidProperty(kind, property.id))?;
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
    if valid {
        Ok(())
    } else {
        Err(GraphError::InvalidPropertyValue(property.id))
    }
}

fn validate_child(contract: &RelationContract, child: &Declaration) -> Result<(), GraphError> {
    if relation_accepts(contract, child.kind) {
        Ok(())
    } else {
        Err(GraphError::InvalidChildCategory(contract.id))
    }
}
