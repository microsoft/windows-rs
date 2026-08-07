use super::*;

pub(super) fn validate(context: &mut Context<'_>, ty: reader::TypeDef<'_>) {
    validate_fields(context, ty);
    validate_interfaces(context, ty);
}

fn validate_interfaces(context: &mut Context<'_>, ty: reader::TypeDef<'_>) {
    let generics = generics(ty);
    let mut interfaces = Vec::<(reader::InterfaceImpl<'_>, crate::Type)>::new();
    for implementation in ty.interface_impls() {
        let interface = implementation.interface(&generics);
        if let Some((previous, _)) = interfaces.iter().find(|(previous, previous_type)| {
            previous_type == &interface
                && arches_overlap(previous.arches(), implementation.arches())
        }) {
            context.duplicate(
                implementation.row_id(),
                previous.row_id(),
                format!(
                    "duplicate interface `{}` on `{}.{}`",
                    display_type(&interface),
                    ty.namespace(),
                    ty.name()
                ),
            );
        }
        interfaces.push((implementation, interface));
    }
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

fn validate_fields(context: &mut Context<'_>, ty: reader::TypeDef<'_>) {
    let mut names = HashMap::<&str, Vec<reader::Field<'_>>>::new();
    for field in ty.fields() {
        let field_type = field.ty();
        let void_typedef = field_type == crate::Type::Void
            && field.name() == "Value"
            && ty.has_attribute("NativeTypedefAttribute");
        if invalid_signature_type(&field_type) && !void_typedef {
            context.invalid(
                field.row_id(),
                Some(ty.row_id()),
                format!(
                    "field `{}.{}.{}` has invalid type `{}`",
                    ty.namespace(),
                    ty.name(),
                    field.name(),
                    type_name(&field_type)
                ),
            );
        }

        let previous = names
            .entry(field.name())
            .or_default()
            .iter()
            .find(|previous| arches_overlap(previous.arches(), field.arches()));
        if let Some(previous) = previous {
            context.duplicate(
                field.row_id(),
                previous.row_id(),
                format!("duplicate field `{}`", field.name()),
            );
        }
        names.entry(field.name()).or_default().push(field);
    }
}
