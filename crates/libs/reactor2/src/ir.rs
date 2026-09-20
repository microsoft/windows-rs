use super::*;
use std::collections::HashSet;

const MAX_DEPTH: usize = 128;
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
    InvalidRelation(ObjectType, RelationId),
    MissingChild(RelationId, ObjectId),
    InvalidChildCategory(RelationId),
    InvalidCardinality(RelationId),
    MissingKey(RelationId),
    DepthExceeded,
    SizeExceeded,
}

pub(crate) fn validate_declaration(root: &Declaration) -> Result<(), GraphError> {
    let mut objects = 0;
    validate_object(root, 0, &mut objects)
}

fn validate_object(
    declaration: &Declaration,
    depth: usize,
    objects: &mut usize,
) -> Result<(), GraphError> {
    if depth > MAX_DEPTH {
        return Err(GraphError::DepthExceeded);
    }
    if *objects >= MAX_OBJECTS {
        return Err(GraphError::SizeExceeded);
    }
    *objects += 1;

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
            (ValueType::String, EventValue::String(_))
                | (ValueType::F64, EventValue::F64(_))
                | (ValueType::Unit, EventValue::Unit(_))
        );
        if !valid {
            return Err(GraphError::InvalidEventValue(event.id));
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
                    validate_child(contract, child)?;
                    validate_object(child, depth + 1, objects)?;
                }
            }
            RelationValue::Many(children) => {
                if contract.cardinality != Cardinality::Many {
                    return Err(GraphError::InvalidCardinality(relation.id));
                }
                if contract.identity == Identity::Keyed {
                    let mut keys = HashSet::with_capacity(children.len());
                    for child in children.iter() {
                        let key = child
                            .key
                            .as_ref()
                            .ok_or(GraphError::MissingKey(relation.id))?;
                        if !keys.insert(key.clone()) {
                            return Err(GraphError::DuplicateKey(key.clone()));
                        }
                    }
                }
                for child in children.iter() {
                    validate_child(contract, child)?;
                    validate_object(child, depth + 1, objects)?;
                }
            }
        }
    }
    Ok(())
}

pub(crate) fn validate_property(kind: ObjectType, property: &Property) -> Result<(), GraphError> {
    let contract = property_contracts(kind)
        .iter()
        .find(|contract| contract.id == property.id)
        .ok_or(GraphError::InvalidProperty(kind, property.id))?;
    let valid = match (contract.value, &property.value) {
        (ValueType::String, PropertyValue::String(_))
        | (ValueType::Bool, PropertyValue::Bool(_))
        | (ValueType::F64, PropertyValue::F64(_))
        | (ValueType::OptionalBool, PropertyValue::OptionalBool(_)) => true,
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
    if object_category(child.kind) == contract.child {
        Ok(())
    } else {
        Err(GraphError::InvalidChildCategory(contract.id))
    }
}
