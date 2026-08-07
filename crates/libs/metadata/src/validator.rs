use crate::reader::{self, AsRow, HasAttributes, RowId};
use std::collections::{HashMap, HashSet};

/// A metadata validation failure associated with one row and, when applicable, a related row.
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

/// Configures validation for authored metadata and its external definitions.
pub struct Validator<'a> {
    index: &'a reader::Index,
    references: Option<&'a reader::Index>,
}

impl<'a> Validator<'a> {
    pub fn new(index: &'a reader::Index) -> Self {
        Self {
            index,
            references: None,
        }
    }

    pub fn references(mut self, references: &'a reader::Index) -> Self {
        self.references = Some(references);
        self
    }

    pub fn validate(self) -> Vec<ValidationError> {
        validate_impl(self.index, self.references)
    }
}

/// Validates metadata identities and associations exposed by [`reader::Index`].
pub fn validate(index: &reader::Index) -> Vec<ValidationError> {
    Validator::new(index).validate()
}

fn validate_impl(
    index: &reader::Index,
    references: Option<&reader::Index>,
) -> Vec<ValidationError> {
    let mut errors = vec![];
    validate_attributes(index, references, &mut errors);
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

        validate_type_members(ty, &mut errors);
        for nested in index.nested_recursive(ty) {
            validate_type_members(nested, &mut errors);
        }
    }

    errors
}

fn validate_attributes(
    index: &reader::Index,
    references: Option<&reader::Index>,
    errors: &mut Vec<ValidationError>,
) {
    let mut applied = HashSet::new();

    for attribute in index.attributes() {
        validate_attribute_constructor(attribute, references, errors);

        let Some(definition) = attribute_definition(index, references, attribute) else {
            continue;
        };
        if !definition.has_attribute("AttributeUsageAttribute") {
            continue;
        }
        let Some(parent) = attribute_parent(attribute.parent()) else {
            continue;
        };

        let key = (
            parent,
            definition.namespace().to_string(),
            definition.name().to_string(),
        );
        if !applied.insert(key) && !definition.has_attribute("AllowMultipleAttribute") {
            errors.push(ValidationError {
                category: ValidationCategory::Duplicate,
                message: format!(
                    "duplicate attribute `{}.{}`",
                    definition.namespace(),
                    definition.name()
                ),
                row: attribute.row_id(),
                related: Some(parent),
            });
        }
    }
}

fn validate_attribute_constructor(
    attribute: reader::Attribute<'_>,
    references: Option<&reader::Index>,
    errors: &mut Vec<ValidationError>,
) {
    let ctor = attribute.ctor();
    let signature = ctor.signature(&[]);
    let parent = attribute_parent(attribute.parent());

    if ctor.name() != ".ctor" {
        errors.push(ValidationError {
            category: ValidationCategory::Invalid,
            message: format!(
                "attribute `{}.{}` constructor is named `{}` instead of `.ctor`",
                attribute.namespace(),
                attribute.name(),
                ctor.name()
            ),
            row: attribute.row_id(),
            related: parent,
        });
    }

    if !signature
        .flags
        .contains(crate::MethodCallAttributes::HASTHIS)
    {
        errors.push(ValidationError {
            category: ValidationCategory::Invalid,
            message: format!(
                "attribute `{}.{}` constructor must be an instance method",
                attribute.namespace(),
                attribute.name()
            ),
            row: attribute.row_id(),
            related: parent,
        });
    } else if signature.flags != crate::MethodCallAttributes::HASTHIS {
        errors.push(ValidationError {
            category: ValidationCategory::Invalid,
            message: format!(
                "attribute `{}.{}` constructor must use the default calling convention",
                attribute.namespace(),
                attribute.name()
            ),
            row: attribute.row_id(),
            related: parent,
        });
    }

    if signature.return_type != crate::Type::Void {
        errors.push(ValidationError {
            category: ValidationCategory::Invalid,
            message: format!(
                "attribute `{}.{}` constructor must return void",
                attribute.namespace(),
                attribute.name()
            ),
            row: attribute.row_id(),
            related: parent,
        });
    }

    let value = match references {
        Some(references) => attribute.try_value_with_references(references),
        None => attribute.try_value(),
    };

    if let Err(error) = value
        && !error.is_unsupported()
    {
        errors.push(ValidationError {
            category: ValidationCategory::Invalid,
            message: format!(
                "attribute `{}.{}` value is invalid at byte {}: {}",
                attribute.namespace(),
                attribute.name(),
                error.offset(),
                error.message()
            ),
            row: attribute.row_id(),
            related: parent,
        });
    }
}

fn attribute_definition<'a>(
    index: &'a reader::Index,
    references: Option<&'a reader::Index>,
    attribute: reader::Attribute<'a>,
) -> Option<reader::TypeDef<'a>> {
    let parent = attribute.ctor().parent();
    let mut definitions = index.get(parent.namespace(), parent.name());
    if let Some(definition) = definitions.next() {
        return definitions.next().is_none().then_some(definition);
    }

    let mut definitions = references?.get(parent.namespace(), parent.name());
    let definition = definitions.next()?;
    definitions.next().is_none().then_some(definition)
}

fn attribute_parent(parent: reader::HasAttribute<'_>) -> Option<RowId> {
    Some(match parent {
        reader::HasAttribute::TypeDef(row) => row.row_id(),
        reader::HasAttribute::Event(row) => row.row_id(),
        reader::HasAttribute::Field(row) => row.row_id(),
        reader::HasAttribute::MethodDef(row) => row.row_id(),
        reader::HasAttribute::MethodParam(row) => row.row_id(),
        reader::HasAttribute::Property(row) => row.row_id(),
        reader::HasAttribute::InterfaceImpl(row) => row.row_id(),
        reader::HasAttribute::TypeRef(_)
        | reader::HasAttribute::MemberRef(_)
        | reader::HasAttribute::TypeSpec(_)
        | reader::HasAttribute::GenericParam(_) => return None,
    })
}

fn validate_type_members(ty: reader::TypeDef, errors: &mut Vec<ValidationError>) {
    validate_fields(ty, errors);
    validate_interfaces(ty, errors);
    validate_properties(ty, errors);
    validate_events(ty, errors);
    validate_methods(ty, errors);
}

fn validate_interfaces(ty: reader::TypeDef, errors: &mut Vec<ValidationError>) {
    let generics = generics(ty);
    let mut interfaces = Vec::<(reader::InterfaceImpl<'_>, crate::Type)>::new();
    for implementation in ty.interface_impls() {
        let interface = implementation.interface(&generics);
        if let Some((previous, _)) = interfaces.iter().find(|(previous, previous_type)| {
            previous_type == &interface
                && arches_overlap(previous.arches(), implementation.arches())
        }) {
            errors.push(duplicate(
                implementation.row_id(),
                previous.row_id(),
                format!(
                    "duplicate interface `{}` on `{}.{}`",
                    display_type(&interface),
                    ty.namespace(),
                    ty.name()
                ),
            ));
        }

        fn display_type(ty: &crate::Type) -> String {
            let (crate::Type::ClassName(name) | crate::Type::ValueName(name)) = ty else {
                return format!("{ty:?}");
            };
            let mut result = if name.namespace.is_empty() {
                name.name.clone()
            } else {
                format!("{}.{}", name.namespace, name.name)
            };
            if !name.generics.is_empty() {
                result.push('<');
                for (index, generic) in name.generics.iter().enumerate() {
                    if index != 0 {
                        result.push_str(", ");
                    }
                    result.push_str(&display_type(generic));
                }
                result.push('>');
            }
            result
        }
        interfaces.push((implementation, interface));
    }
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
                format!("duplicate field `{}`", field.name()),
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
