use super::*;
use std::collections::HashSet;

pub(super) fn validate(context: &mut Context<'_>) {
    let mut applied = HashSet::new();

    for attribute in context.index.attributes() {
        let args = validate_constructor(context, attribute);

        let Some(definition) = definition(context, attribute) else {
            continue;
        };
        let parent = parent(attribute.parent());

        if let Some(args) = args {
            validate_named_args(context, attribute, definition, parent, &args);
        }

        if definition.has_attribute("AttributeUsageAttribute")
            && let Some(parent) = parent
        {
            let key = (
                parent,
                definition.namespace().to_string(),
                definition.name().to_string(),
            );
            if !applied.insert(key) && !definition.has_attribute("AllowMultipleAttribute") {
                context.duplicate(
                    attribute.row_id(),
                    parent,
                    format!(
                        "duplicate attribute `{}.{}`",
                        definition.namespace(),
                        definition.name()
                    ),
                );
            }
        }
    }
}

fn validate_constructor(
    context: &mut Context<'_>,
    attribute: reader::Attribute<'_>,
) -> Option<Vec<reader::AttributeArg>> {
    let ctor = attribute.ctor();
    let signature = ctor.signature(&[]);
    let parent = parent(attribute.parent());

    if ctor.name() != ".ctor" {
        context.invalid(
            attribute.row_id(),
            parent,
            format!(
                "attribute `{}.{}` constructor is named `{}` instead of `.ctor`",
                attribute.namespace(),
                attribute.name(),
                ctor.name()
            ),
        );
    }

    if !signature
        .flags
        .contains(crate::MethodCallAttributes::HASTHIS)
    {
        context.invalid(
            attribute.row_id(),
            parent,
            format!(
                "attribute `{}.{}` constructor must be an instance method",
                attribute.namespace(),
                attribute.name()
            ),
        );
    } else if signature.flags != crate::MethodCallAttributes::HASTHIS {
        context.invalid(
            attribute.row_id(),
            parent,
            format!(
                "attribute `{}.{}` constructor must use the default calling convention",
                attribute.namespace(),
                attribute.name()
            ),
        );
    }

    if signature.return_type != crate::Type::Void {
        context.invalid(
            attribute.row_id(),
            parent,
            format!(
                "attribute `{}.{}` constructor must return void",
                attribute.namespace(),
                attribute.name()
            ),
        );
    }

    let mut valid_parameters = true;
    for (position, ty) in signature.types.iter().enumerate() {
        if !valid_parameter_type(ty) {
            valid_parameters = false;
            context.invalid(
                attribute.row_id(),
                parent,
                format!(
                    "attribute `{}.{}` constructor parameter {} has invalid type `{}`",
                    attribute.namespace(),
                    attribute.name(),
                    position + 1,
                    type_name(ty)
                ),
            );
        }
    }

    if !valid_parameters {
        return None;
    }

    let args = match context.references {
        Some(references) => attribute.try_args_with_references(references),
        None => attribute.try_args(),
    };

    match args {
        Ok(args) => Some(args),
        Err(error) if error.is_unsupported() => None,
        Err(error) => {
            context.invalid(
                attribute.row_id(),
                parent,
                format!(
                    "attribute `{}.{}` value is invalid at byte {}: {}",
                    attribute.namespace(),
                    attribute.name(),
                    error.offset(),
                    error.message()
                ),
            );
            None
        }
    }
}

fn validate_named_args(
    context: &mut Context<'_>,
    attribute: reader::Attribute<'_>,
    definition: reader::TypeDef<'_>,
    parent: Option<RowId>,
    args: &[reader::AttributeArg],
) {
    let mut names = HashSet::new();

    for arg in args {
        let reader::AttributeArg::Named { kind, name, value } = arg else {
            continue;
        };

        if !names.insert((*kind, name.as_str())) {
            context.duplicate_optional(
                attribute.row_id(),
                parent,
                format!(
                    "attribute `{}.{}` has duplicate named {} argument `{name}`",
                    attribute.namespace(),
                    attribute.name(),
                    arg_kind(*kind)
                ),
            );
        }

        let member = match kind {
            reader::AttributeArgKind::Field => definition
                .fields()
                .find(|field| field.name() == name)
                .map(|field| {
                    let flags = field.flags();
                    (
                        field.ty(),
                        flags.0 & 0x7 == crate::FieldAttributes::Public.0
                            && !flags.contains(crate::FieldAttributes::Static)
                            && !flags.contains(crate::FieldAttributes::InitOnly)
                            && !flags.contains(crate::FieldAttributes::Literal),
                    )
                }),
            reader::AttributeArgKind::Property => definition
                .properties()
                .find(|property| property.name() == name)
                .map(|property| {
                    let signature = property.signature(&[]);
                    let usable = signature.types.is_empty()
                        && property.semantics().any(|semantics| {
                            if semantics.semantics() != 0x0001 {
                                return false;
                            }
                            let method = semantics.method();
                            let flags = method.flags();
                            let setter = method.signature(&[]);
                            flags.0 & 0x7 == crate::MethodAttributes::Public.0
                                && !flags.contains(crate::MethodAttributes::Static)
                                && setter.flags.contains(crate::MethodCallAttributes::HASTHIS)
                                && setter.return_type == crate::Type::Void
                                && setter.types == [signature.return_type.clone()]
                        });
                    (signature.return_type, usable)
                }),
        };

        let Some((expected, usable)) = member else {
            context.invalid(
                attribute.row_id(),
                parent,
                format!(
                    "attribute `{}.{}` has no named {} `{name}`",
                    attribute.namespace(),
                    attribute.name(),
                    arg_kind(*kind)
                ),
            );
            continue;
        };

        if !usable {
            context.invalid(
                attribute.row_id(),
                parent,
                format!(
                    "attribute `{}.{}` named {} `{name}` is not a public writable instance member",
                    attribute.namespace(),
                    attribute.name(),
                    arg_kind(*kind)
                ),
            );
        }

        let actual = value.ty();
        if actual != expected {
            context.invalid(
                attribute.row_id(),
                parent,
                format!(
                    "attribute `{}.{}` named {} `{name}` expects `{}` but found `{}`",
                    attribute.namespace(),
                    attribute.name(),
                    arg_kind(*kind),
                    type_name(&expected),
                    type_name(&actual)
                ),
            );
        }
    }
}

fn arg_kind(kind: reader::AttributeArgKind) -> &'static str {
    match kind {
        reader::AttributeArgKind::Field => "field",
        reader::AttributeArgKind::Property => "property",
    }
}

fn valid_parameter_type(ty: &crate::Type) -> bool {
    match ty {
        crate::Type::Bool
        | crate::Type::Char
        | crate::Type::I8
        | crate::Type::U8
        | crate::Type::I16
        | crate::Type::U16
        | crate::Type::I32
        | crate::Type::U32
        | crate::Type::I64
        | crate::Type::U64
        | crate::Type::F32
        | crate::Type::F64
        | crate::Type::String
        | crate::Type::Object
        | crate::Type::ValueName(_) => true,
        crate::Type::ClassName(name) => name == ("System", "Type"),
        crate::Type::Array(element) => valid_parameter_type(element),
        _ => false,
    }
}

fn definition<'a>(
    context: &Context<'a>,
    attribute: reader::Attribute<'a>,
) -> Option<reader::TypeDef<'a>> {
    let parent = attribute.ctor().parent();
    let mut definitions = context.index.get(parent.namespace(), parent.name());
    if let Some(definition) = definitions.next() {
        return definitions.next().is_none().then_some(definition);
    }

    let mut definitions = context.references?.get(parent.namespace(), parent.name());
    let definition = definitions.next()?;
    definitions.next().is_none().then_some(definition)
}

fn parent(parent: reader::HasAttribute<'_>) -> Option<RowId> {
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
