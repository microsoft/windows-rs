use super::*;

pub(super) fn validate_maps(context: &mut Context<'_>) {
    validate_map_rows(
        context,
        context.index.property_maps(),
        context.index.properties(),
        "property",
        |map| map.parent(),
        |map| map.properties(),
        |property| property.name(),
    );
    validate_map_rows(
        context,
        context.index.event_maps(),
        context.index.events(),
        "event",
        |map| map.parent(),
        |map| map.events(),
        |event| event.name(),
    );
}

pub(super) fn validate_type<'a>(context: &mut Context<'a>, ty: reader::TypeDef<'a>) {
    validate_properties(context, ty);
    validate_events(context, ty);
}

fn validate_map_rows<'a, M, R, I>(
    context: &mut Context<'a>,
    maps: impl Iterator<Item = M>,
    rows: impl Iterator<Item = R>,
    kind: &str,
    parent: impl Fn(M) -> reader::TypeDef<'a>,
    members: impl Fn(M) -> I,
    name: impl Fn(R) -> &'a str,
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
            context.duplicate(
                map.row_id(),
                previous.row_id(),
                format!(
                    "duplicate {kind} map for `{}.{}`",
                    parent.namespace(),
                    parent.name()
                ),
            );
        }
        for row in members(map) {
            if let Some(previous) = owners.insert(row.row_id(), map) {
                context.duplicate(
                    map.row_id(),
                    previous.row_id(),
                    format!("{kind} `{}` has multiple owners", name(row)),
                );
            }
        }
    }

    for row in rows {
        if !owners.contains_key(&row.row_id()) {
            context.invalid(
                row.row_id(),
                None,
                format!("{kind} `{}` has no owner", name(row)),
            );
        }
    }
}

fn validate_properties<'a>(context: &mut Context<'a>, ty: reader::TypeDef<'a>) {
    let generics = generics(ty);
    let mut properties = HashMap::<&str, Vec<(reader::Property<'_>, crate::Signature)>>::new();
    for property in ty.properties() {
        let signature = property.signature(&generics);
        if invalid_signature_type(&signature.return_type) {
            context.invalid(
                property.row_id(),
                Some(ty.row_id()),
                format!(
                    "property `{}.{}.{}` has invalid value type `{}`",
                    ty.namespace(),
                    ty.name(),
                    property.name(),
                    type_name(&signature.return_type)
                ),
            );
        }
        for (position, parameter) in signature.types.iter().enumerate() {
            if invalid_signature_type(parameter) {
                context.invalid(
                    property.row_id(),
                    Some(ty.row_id()),
                    format!(
                        "property `{}.{}.{}` index parameter {} has invalid type `{}`",
                        ty.namespace(),
                        ty.name(),
                        property.name(),
                        position + 1,
                        type_name(parameter)
                    ),
                );
            }
        }
        let arches = association_arches(property.arches(), property.semantics());
        let previous = properties.entry(property.name()).or_default().iter().find(
            |(previous, previous_signature)| {
                same_identity(previous_signature, &signature)
                    && arches_overlap(
                        association_arches(previous.arches(), previous.semantics()),
                        arches,
                    )
                    && (previous_signature.return_type != signature.return_type
                        || semantics_conflict(previous.semantics(), property.semantics(), 0x0003))
            },
        );
        if let Some((previous, _)) = previous {
            context.duplicate(
                property.row_id(),
                previous.row_id(),
                format!(
                    "duplicate property `{}` on `{}.{}`",
                    property.name(),
                    ty.namespace(),
                    ty.name()
                ),
            );
        }
        properties
            .entry(property.name())
            .or_default()
            .push((property, signature));
        validate_semantics(
            context,
            property.row_id(),
            property.name(),
            "property",
            property.semantics(),
            &[0x0001, 0x0002, 0x0004],
        );
    }
}

fn validate_events<'a>(context: &mut Context<'a>, ty: reader::TypeDef<'a>) {
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
            context.duplicate(
                event.row_id(),
                previous.row_id(),
                format!(
                    "duplicate event `{}` on `{}.{}`",
                    event.name(),
                    ty.namespace(),
                    ty.name()
                ),
            );
        }
        events
            .entry(event.name())
            .or_default()
            .push((event, event_type));
        validate_semantics(
            context,
            event.row_id(),
            event.name(),
            "event",
            event.semantics(),
            &[0x0004, 0x0008, 0x0010, 0x0020],
        );
    }
}

fn validate_semantics<'a>(
    context: &mut Context<'a>,
    association: RowId,
    name: &str,
    kind: &str,
    semantics: impl Iterator<Item = reader::MethodSemantics<'a>>,
    allowed: &[u16],
) {
    let mut seen = HashMap::<u16, Vec<(reader::MethodSemantics<'_>, i32)>>::new();
    for semantics in semantics {
        let value = semantics.semantics();
        if !allowed.contains(&value) {
            context.invalid(
                semantics.row_id(),
                Some(association),
                format!("{kind} `{name}` has invalid method semantics {value:#06x}"),
            );
        } else if value != 0x0004 {
            let arches = semantics.method().arches();
            let previous = seen
                .entry(value)
                .or_default()
                .iter()
                .find(|(_, previous_arches)| arches_overlap(*previous_arches, arches));
            if let Some((previous, _)) = previous {
                context.duplicate(
                    semantics.row_id(),
                    previous.row_id(),
                    format!("{kind} `{name}` has duplicate method semantics {value:#06x}"),
                );
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
