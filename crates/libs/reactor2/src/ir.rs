use super::*;
use std::collections::HashSet;

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
    pub(crate) fn validate(&mut self, root: &Declaration) -> Result<(), GraphError> {
        self.references.clear();
        self.keys.clear();
        let mut objects = 0;
        validate_object(root, 0, &mut objects, &mut self.references, &mut self.keys)
    }
}

fn validate_object(
    declaration: &Declaration,
    depth: usize,
    objects: &mut usize,
    references: &mut HashSet<usize>,
    keys: &mut HashSet<Key>,
) -> Result<(), GraphError> {
    if depth > MAX_DEPTH {
        return Err(GraphError::DepthExceeded);
    }
    if *objects >= MAX_OBJECTS {
        return Err(GraphError::SizeExceeded);
    }
    *objects += 1;
    if declaration
        .reference
        .as_ref()
        .is_some_and(|reference| !references.insert(reference.identity()))
    {
        return Err(GraphError::DuplicateReference);
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
                | (ValueType::String, EventValue::String(_))
                | (ValueType::F64, EventValue::F64(_))
                | (ValueType::OptionalBool, EventValue::OptionalBool(_))
                | (ValueType::OptionalF64, EventValue::OptionalF64(_))
                | (ValueType::PointerEventInfo, EventValue::PointerEventInfo(_))
                | (ValueType::Selection, EventValue::Selection(_))
                | (ValueType::SelectionIndex, EventValue::SelectionIndex(_))
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
                    validate_object(child, depth + 1, objects, references, keys)?;
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
                    validate_object(child, depth + 1, objects, references, keys)?;
                }
            }
        }
    }
    Ok(())
}

fn child_object(child: &DeclaredNode) -> Result<&Declaration, GraphError> {
    match child {
        DeclaredNode::Object(child) => Ok(child),
        DeclaredNode::Component { .. } => Err(GraphError::UnresolvedComponent),
    }
}

pub(crate) fn validate_property(kind: ObjectType, property: &Property) -> Result<(), GraphError> {
    let contract = property_contract(kind, property.id)
        .ok_or(GraphError::InvalidProperty(kind, property.id))?;
    let valid = match (contract.value, &property.value) {
        (ValueType::String, PropertyValue::String(_))
        | (ValueType::Bool, PropertyValue::Bool(_))
        | (ValueType::Color, PropertyValue::Color(_))
        | (ValueType::CornerRadius, PropertyValue::CornerRadius(_))
        | (ValueType::F64, PropertyValue::F64(_))
        | (ValueType::FontWeight, PropertyValue::FontWeight(_))
        | (ValueType::GridLengths, PropertyValue::GridLengths(_))
        | (ValueType::I32, PropertyValue::I32(_))
        | (ValueType::OptionalF64, PropertyValue::OptionalF64(_))
        | (ValueType::OptionalBool, PropertyValue::OptionalBool(_))
        | (ValueType::SelectionIndex, PropertyValue::SelectionIndex(_))
        | (ValueType::StringList, PropertyValue::StringList(_))
        | (ValueType::ThemeTransitions, PropertyValue::ThemeTransitions(_))
        | (ValueType::Thickness, PropertyValue::Thickness(_)) => true,
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
