use super::*;

/// The role a WinRT accessor method plays in a property or event.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum AccessorKind {
    Get,
    Set,
    Add,
    Remove,
}

/// Classifies a special-name method as a property or event accessor from its name and
/// signature, returning the accessor kind and the property or event name. The reader uses this
/// to associate explicit accessors with Property/Event rows and the writer uses it to decide
/// whether the shorthand can regenerate the method.
pub(crate) fn accessor_kind<'a>(
    name: &'a str,
    types: &[metadata::Type],
    return_type: &metadata::Type,
) -> Option<(AccessorKind, &'a str)> {
    let input =
        |ty: &metadata::Type| !matches!(ty, metadata::Type::RefMut(_) | metadata::Type::PtrMut(..));

    if let Some(name) = name.strip_prefix("get_") {
        (types.is_empty() && *return_type != metadata::Type::Void)
            .then_some((AccessorKind::Get, name))
    } else if let Some(name) = name.strip_prefix("put_") {
        (types.len() == 1 && input(&types[0]) && *return_type == metadata::Type::Void)
            .then_some((AccessorKind::Set, name))
    } else if let Some(name) = name.strip_prefix("add_") {
        (types.len() == 1
            && matches!(types[0], metadata::Type::ClassName(_))
            && *return_type == event_token())
        .then_some((AccessorKind::Add, name))
    } else if let Some(name) = name.strip_prefix("remove_") {
        (types.len() == 1 && types[0] == event_token() && *return_type == metadata::Type::Void)
            .then_some((AccessorKind::Remove, name))
    } else {
        None
    }
}

pub(crate) fn event_token() -> metadata::Type {
    metadata::Type::value_named("Windows.Foundation", "EventRegistrationToken")
}
