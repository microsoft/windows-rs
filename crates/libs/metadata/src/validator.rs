use crate::reader::{self, AsRow, HasAttributes, RowId};
use std::collections::HashMap;

/// A metadata validation failure associated with one row and, when applicable, an earlier row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationError {
    category: ValidationCategory,
    message: String,
    row: RowId,
    related: Option<RowId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationCategory {
    Duplicate,
    Invalid,
}

impl ValidationError {
    pub fn category(&self) -> ValidationCategory {
        self.category
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn row(&self) -> RowId {
        self.row
    }

    pub fn related(&self) -> Option<RowId> {
        self.related
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for ValidationError {}

/// Validates metadata identities and associations exposed by [`reader::Index`].
pub fn validate(index: &reader::Index) -> Vec<ValidationError> {
    let mut errors = vec![];
    validate_property_maps(index, &mut errors);
    validate_event_maps(index, &mut errors);
    validate_layouts(index, &mut errors);

    let mut types: Vec<_> = index.types().collect();
    types.sort_by(|a, b| {
        (a.namespace(), a.name(), a.row_id()).cmp(&(b.namespace(), b.name(), b.row_id()))
    });

    let mut names = HashMap::<(&str, &str), Vec<reader::TypeDef<'_>>>::new();
    for ty in types {
        let previous = names
            .entry((ty.namespace(), ty.name()))
            .or_default()
            .iter()
            .find(|previous| arches_overlap(previous.arches(), ty.arches()));
        if let Some(previous) = previous {
            errors.push(duplicate(
                ty.row_id(),
                previous.row_id(),
                format!("duplicate type `{}.{}`", ty.namespace(), ty.name()),
            ));
        }
        names
            .entry((ty.namespace(), ty.name()))
            .or_default()
            .push(ty);

        validate_fields(ty, &mut errors);
        validate_properties(ty, &mut errors);
        validate_events(ty, &mut errors);
        validate_methods(ty, &mut errors);
    }

    errors
}

fn validate_layouts(index: &reader::Index, errors: &mut Vec<ValidationError>) {
    let mut classes = HashMap::new();
    for layout in index.class_layouts() {
        let parent = layout.parent();
        if let Some(previous) = classes.insert(parent.row_id(), layout) {
            errors.push(duplicate(
                layout.row_id(),
                previous.row_id(),
                format!(
                    "duplicate class layout for `{}.{}`",
                    parent.namespace(),
                    parent.name()
                ),
            ));
        }

        let packing = layout.packing_size();
        if packing != 0 && (packing > 128 || !packing.is_power_of_two()) {
            errors.push(ValidationError {
                category: ValidationCategory::Invalid,
                message: format!(
                    "class layout for `{}.{}` has invalid packing size {packing}",
                    parent.namespace(),
                    parent.name()
                ),
                row: layout.row_id(),
                related: Some(parent.row_id()),
            });
        }

        let flags = parent.flags();
        if !flags.contains(crate::TypeAttributes::SequentialLayout)
            && !flags.contains(crate::TypeAttributes::ExplicitLayout)
        {
            errors.push(ValidationError {
                category: ValidationCategory::Invalid,
                message: format!(
                    "class layout for `{}.{}` requires sequential or explicit layout",
                    parent.namespace(),
                    parent.name()
                ),
                row: layout.row_id(),
                related: Some(parent.row_id()),
            });
        }
    }

    let mut fields = HashMap::new();
    for layout in index.field_layouts() {
        let field = layout.field();
        if let Some(previous) = fields.insert(field.row_id(), layout) {
            errors.push(duplicate(
                layout.row_id(),
                previous.row_id(),
                format!("duplicate field layout for `{}`", field.name()),
            ));
        }

        let parent = field.parent();
        if !parent
            .flags()
            .contains(crate::TypeAttributes::ExplicitLayout)
        {
            errors.push(ValidationError {
                category: ValidationCategory::Invalid,
                message: format!(
                    "field layout for `{}.{}.{}` requires explicit layout",
                    parent.namespace(),
                    parent.name(),
                    field.name()
                ),
                row: layout.row_id(),
                related: Some(field.row_id()),
            });
        }
    }
}

fn validate_property_maps(index: &reader::Index, errors: &mut Vec<ValidationError>) {
    validate_maps(
        index.property_maps(),
        index.properties(),
        "property",
        |map| map.parent(),
        |map| map.properties(),
        |property| property.name(),
        errors,
    );
}

fn validate_event_maps(index: &reader::Index, errors: &mut Vec<ValidationError>) {
    validate_maps(
        index.event_maps(),
        index.events(),
        "event",
        |map| map.parent(),
        |map| map.events(),
        |event| event.name(),
        errors,
    );
}

fn validate_maps<'a, M, R, I>(
    maps: impl Iterator<Item = M>,
    rows: impl Iterator<Item = R>,
    kind: &str,
    parent: impl Fn(M) -> reader::TypeDef<'a>,
    members: impl Fn(M) -> I,
    name: impl Fn(R) -> &'a str,
    errors: &mut Vec<ValidationError>,
) where
    M: AsRow<'a>,
    R: AsRow<'a>,
    I: Iterator<Item = R>,
{
    let mut parents = HashMap::new();
    let mut owners = HashMap::new();

    for map in maps {
        let parent = parent(map);
        if let Some(previous) = parents.insert(parent.row_id(), map) {
            errors.push(duplicate(
                map.row_id(),
                previous.row_id(),
                format!(
                    "duplicate {kind} map for `{}.{}`",
                    parent.namespace(),
                    parent.name()
                ),
            ));
        }
        for row in members(map) {
            if let Some(previous) = owners.insert(row.row_id(), map) {
                errors.push(duplicate(
                    map.row_id(),
                    previous.row_id(),
                    format!("{kind} `{}` has multiple owners", name(row)),
                ));
            }
        }
    }

    for row in rows {
        if !owners.contains_key(&row.row_id()) {
            errors.push(ValidationError {
                category: ValidationCategory::Invalid,
                message: format!("{kind} `{}` has no owner", name(row)),
                row: row.row_id(),
                related: None,
            });
        }
    }
}

fn validate_fields(ty: reader::TypeDef, errors: &mut Vec<ValidationError>) {
    let mut names = HashMap::<&str, Vec<reader::Field<'_>>>::new();
    for field in ty.fields() {
        let previous = names
            .entry(field.name())
            .or_default()
            .iter()
            .find(|previous| arches_overlap(previous.arches(), field.arches()));
        if let Some(previous) = previous {
            errors.push(duplicate(
                field.row_id(),
                previous.row_id(),
                format!(
                    "duplicate field `{}` on `{}.{}`",
                    field.name(),
                    ty.namespace(),
                    ty.name()
                ),
            ));
        }
        names.entry(field.name()).or_default().push(field);
    }
}

fn validate_properties(ty: reader::TypeDef, errors: &mut Vec<ValidationError>) {
    let generics = generics(ty);
    let mut properties = HashMap::<&str, Vec<(reader::Property<'_>, crate::Signature)>>::new();
    for property in ty.properties() {
        let signature = property.signature(&generics);
        let arches = association_arches(property.arches(), property.semantics());
        let previous = properties.entry(property.name()).or_default().iter().find(
            |(previous, previous_signature)| {
                same_property_identity(previous_signature, &signature)
                    && arches_overlap(
                        association_arches(previous.arches(), previous.semantics()),
                        arches,
                    )
                    && (previous_signature.return_type != signature.return_type
                        || semantics_conflict(previous.semantics(), property.semantics(), 0x0003))
            },
        );
        if let Some((previous, _)) = previous {
            errors.push(duplicate(
                property.row_id(),
                previous.row_id(),
                format!(
                    "duplicate property `{}` on `{}.{}`",
                    property.name(),
                    ty.namespace(),
                    ty.name()
                ),
            ));
        }
        properties
            .entry(property.name())
            .or_default()
            .push((property, signature));
        validate_semantics(
            property.row_id(),
            property.name(),
            "property",
            property.semantics(),
            &[0x0001, 0x0002, 0x0004],
            errors,
        );
    }
}

fn validate_events(ty: reader::TypeDef, errors: &mut Vec<ValidationError>) {
    let generics = generics(ty);
    let mut events = HashMap::<&str, Vec<(reader::Event<'_>, crate::Type)>>::new();
    for event in ty.events() {
        let event_type = event.ty(&generics);
        let arches = association_arches(event.arches(), event.semantics());
        let previous =
            events
                .entry(event.name())
                .or_default()
                .iter()
                .find(|(previous, previous_type)| {
                    arches_overlap(
                        association_arches(previous.arches(), previous.semantics()),
                        arches,
                    ) && (previous_type != &event_type
                        || semantics_conflict(previous.semantics(), event.semantics(), 0x0038))
                });
        if let Some((previous, _)) = previous {
            errors.push(duplicate(
                event.row_id(),
                previous.row_id(),
                format!(
                    "duplicate event `{}` on `{}.{}`",
                    event.name(),
                    ty.namespace(),
                    ty.name()
                ),
            ));
        }
        events
            .entry(event.name())
            .or_default()
            .push((event, event_type));
        validate_semantics(
            event.row_id(),
            event.name(),
            "event",
            event.semantics(),
            &[0x0004, 0x0008, 0x0010, 0x0020],
            errors,
        );
    }
}

fn validate_semantics<'a>(
    association: RowId,
    name: &str,
    kind: &str,
    semantics: impl Iterator<Item = reader::MethodSemantics<'a>>,
    allowed: &[u16],
    errors: &mut Vec<ValidationError>,
) {
    let mut seen = HashMap::<u16, Vec<(reader::MethodSemantics<'_>, i32)>>::new();
    for semantics in semantics {
        let value = semantics.semantics();
        if !allowed.contains(&value) {
            errors.push(ValidationError {
                category: ValidationCategory::Invalid,
                message: format!("{kind} `{name}` has invalid method semantics {value:#06x}"),
                row: semantics.row_id(),
                related: Some(association),
            });
        } else if value != 0x0004 {
            let arches = semantics.method().arches();
            let previous = seen
                .entry(value)
                .or_default()
                .iter()
                .find(|(_, previous_arches)| arches_overlap(*previous_arches, arches));
            if let Some((previous, _)) = previous {
                errors.push(duplicate(
                    semantics.row_id(),
                    previous.row_id(),
                    format!("{kind} `{name}` has duplicate method semantics {value:#06x}"),
                ));
            }
            seen.entry(value).or_default().push((semantics, arches));
        }
    }
}

fn association_arches<'a>(
    association: i32,
    semantics: impl Iterator<Item = reader::MethodSemantics<'a>>,
) -> i32 {
    let mut methods = None;
    for semantics in semantics {
        let arches = semantics.method().arches();
        methods = Some(match methods {
            None => arches,
            Some(0) => 0,
            Some(_) if arches == 0 => 0,
            Some(current) => current | arches,
        });
    }

    match (association, methods) {
        (0, Some(methods)) => methods,
        (association, None | Some(0)) => association,
        (association, Some(methods)) => {
            let intersection = association & methods;
            if intersection == 0 {
                association
            } else {
                intersection
            }
        }
    }
}

fn semantics_conflict<'a>(
    left: impl Iterator<Item = reader::MethodSemantics<'a>>,
    right: impl Iterator<Item = reader::MethodSemantics<'a>>,
    singleton: u16,
) -> bool {
    let left = left.fold(0, |mask, semantics| mask | semantics.semantics()) & singleton;
    let right = right.fold(0, |mask, semantics| mask | semantics.semantics()) & singleton;
    left == 0 || right == 0 || left & right != 0
}

fn validate_methods(ty: reader::TypeDef, errors: &mut Vec<ValidationError>) {
    let mut methods = HashMap::<&str, Vec<(reader::MethodDef<'_>, crate::Signature)>>::new();
    let generics = generics(ty);
    for method in ty.methods() {
        let signature = method.signature(&generics);
        let previous = methods.entry(method.name()).or_default().iter().find(
            |(previous, previous_signature)| {
                same_method_identity(previous_signature, &signature)
                    && arches_overlap(previous.arches(), method.arches())
            },
        );
        if let Some((previous, _)) = previous {
            errors.push(duplicate(
                method.row_id(),
                previous.row_id(),
                format!(
                    "duplicate method `{}` on `{}.{}`",
                    method.name(),
                    ty.namespace(),
                    ty.name()
                ),
            ));
        }
        methods
            .entry(method.name())
            .or_default()
            .push((method, signature.clone()));

        if let Err(error) = method.params_by_sequence(signature.types.len()) {
            errors.push(ValidationError {
                category: ValidationCategory::Invalid,
                message: format!(
                    "invalid parameters for `{}.{}` method `{}`: {error}",
                    ty.namespace(),
                    ty.name(),
                    method.name()
                ),
                row: method.row_id(),
                related: None,
            });
        }
    }
}

fn generics(ty: reader::TypeDef) -> Vec<crate::Type> {
    ty.generic_params()
        .map(|param| crate::Type::Generic(param.name().to_string(), param.sequence()))
        .collect()
}

fn duplicate(row: RowId, related: RowId, message: String) -> ValidationError {
    ValidationError {
        category: ValidationCategory::Duplicate,
        message,
        row,
        related: Some(related),
    }
}

fn same_method_identity(left: &crate::Signature, right: &crate::Signature) -> bool {
    left.flags == right.flags && left.types == right.types
}

fn same_property_identity(left: &crate::Signature, right: &crate::Signature) -> bool {
    left.flags == right.flags && left.types == right.types
}

fn arches_overlap(left: i32, right: i32) -> bool {
    left == 0 || right == 0 || left & right != 0
}
