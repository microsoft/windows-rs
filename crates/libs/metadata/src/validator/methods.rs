use super::*;

pub(super) fn validate(context: &mut Context<'_>, ty: reader::TypeDef<'_>) {
    let mut methods = HashMap::<&str, Vec<(reader::MethodDef<'_>, crate::Signature)>>::new();
    let mut overloads = HashMap::<String, Vec<(reader::MethodDef<'_>, crate::Signature)>>::new();
    let mut default_overloads = HashMap::<String, Vec<reader::MethodDef<'_>>>::new();
    let generics = generics(ty);
    for method in ty.methods() {
        let signature = method.signature(&generics);
        let is_static = method.flags().contains(crate::MethodAttributes::Static);
        let has_this = signature
            .flags
            .contains(crate::MethodCallAttributes::HASTHIS);
        if is_static && has_this {
            context.invalid(
                method.row_id(),
                Some(ty.row_id()),
                format!(
                    "static method `{}.{}.{}` has an instance calling convention",
                    ty.namespace(),
                    ty.name(),
                    method.name()
                ),
            );
        }
        for (position, parameter) in signature.types.iter().enumerate() {
            if invalid_signature_type(parameter) {
                context.invalid(
                    method.row_id(),
                    Some(ty.row_id()),
                    format!(
                        "method `{}.{}.{}` parameter {} has invalid type `{}`",
                        ty.namespace(),
                        ty.name(),
                        method.name(),
                        position + 1,
                        type_name(parameter)
                    ),
                );
            }
        }
        let previous = methods.entry(method.name()).or_default().iter().find(
            |(previous, previous_signature)| {
                same_identity(previous_signature, &signature)
                    && arches_overlap(previous.arches(), method.arches())
            },
        );
        if let Some((previous, _)) = previous {
            context.duplicate(
                method.row_id(),
                previous.row_id(),
                format!(
                    "duplicate method `{}` on `{}.{}`",
                    method.name(),
                    ty.namespace(),
                    ty.name()
                ),
            );
        }

        methods
            .entry(method.name())
            .or_default()
            .push((method, signature.clone()));

        let overload_name = method
            .attributes()
            .find(|attribute| {
                attribute.namespace() == "Windows.Foundation.Metadata"
                    && attribute.name() == "OverloadAttribute"
            })
            .and_then(|attribute| {
                attribute
                    .value()
                    .into_iter()
                    .find_map(|(_, value)| match value {
                        crate::Value::Utf8(name) => Some(name),
                        _ => None,
                    })
            });
        let is_default_overload = method.attributes().any(|attribute| {
            attribute.namespace() == "Windows.Foundation.Metadata"
                && attribute.name() == "DefaultOverloadAttribute"
        });

        if is_default_overload && overload_name.is_none() {
            context.invalid(
                method.row_id(),
                Some(ty.row_id()),
                format!(
                    "method `{}.{}.{}` has `DefaultOverloadAttribute` without \
                     `OverloadAttribute`",
                    ty.namespace(),
                    ty.name(),
                    method.name()
                ),
            );
        }

        if let Some(overload_name) = overload_name {
            let previous = overloads
                .entry(overload_name.clone())
                .or_default()
                .iter()
                .find(|(previous, previous_signature)| {
                    same_identity(previous_signature, &signature)
                        && arches_overlap(previous.arches(), method.arches())
                });
            if let Some((previous, _)) = previous {
                context.duplicate(
                    method.row_id(),
                    previous.row_id(),
                    format!(
                        "duplicate overload signature `{overload_name}` on `{}.{}`",
                        ty.namespace(),
                        ty.name()
                    ),
                );
            }
            overloads
                .entry(overload_name.clone())
                .or_default()
                .push((method, signature.clone()));

            if is_default_overload {
                let previous = default_overloads
                    .entry(overload_name.clone())
                    .or_default()
                    .iter()
                    .find(|previous| arches_overlap(previous.arches(), method.arches()));
                if let Some(previous) = previous {
                    context.duplicate(
                        method.row_id(),
                        previous.row_id(),
                        format!(
                            "duplicate default overload `{overload_name}` on `{}.{}`",
                            ty.namespace(),
                            ty.name()
                        ),
                    );
                }
                default_overloads
                    .entry(overload_name)
                    .or_default()
                    .push(method);
            }
        }

        if let Err(error) = method.params_by_sequence(signature.types.len()) {
            context.invalid(
                method.row_id(),
                None,
                format!(
                    "invalid parameters for `{}.{}` method `{}`: {error}",
                    ty.namespace(),
                    ty.name(),
                    method.name()
                ),
            );
        }
    }
}
