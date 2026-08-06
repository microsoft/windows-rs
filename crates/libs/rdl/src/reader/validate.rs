use super::*;

const DUPLICATE_CODE: &str = "RDL0001";
const UNREPRESENTABLE_CODE: &str = "RDL0002";
const CONTROL_CODE: &str = "RDL0007";

enum Arch {
    None,
    Valid(i32),
    Invalid,
}

pub fn validate_symbols(index: &Index) -> Vec<Error> {
    let mut diagnostics = vec![];

    for members in index.namespaces.values() {
        if let Err(error) = validate_namespace_symbols(members) {
            diagnostics.push(error);
        }

        for (name, variants) in &members.types {
            if let Err(error) = validate_variants("type", name, variants) {
                diagnostics.push(error);
            }
        }
        for (name, variants) in &members.functions {
            if let Err(error) = validate_variants("function", name, variants) {
                diagnostics.push(error);
            }
        }
        for (name, variants) in &members.constants {
            if let Err(error) = validate_variants("constant", name, variants) {
                diagnostics.push(error);
            }
        }

        for variants in members
            .types
            .values()
            .chain(members.functions.values())
            .chain(members.constants.values())
        {
            for (file, item) in variants {
                if let Err(error) = validate_item(file, item) {
                    diagnostics.push(error);
                }
            }
        }
    }

    diagnostics
}

pub fn validate_resolved_symbols(
    model: &ResolvedModel,
    reference: &metadata::reader::Index,
) -> Vec<Error> {
    let mut diagnostics = vec![];

    for (id, item) in model.items.iter().enumerate() {
        debug_assert_eq!(item.id, id);
        match (&item.kind, item.item) {
            (ResolvedItemKind::Attribute { constructors }, Item::Attribute(attribute)) => {
                let mut signatures = vec![];
                for constructor in constructors {
                    if let Some((_, previous)) = signatures
                        .iter()
                        .find(|(types, _)| types == &&constructor.types)
                    {
                        diagnostics.push(duplicate_error(
                            "attribute constructor",
                            &attribute.name.to_string(),
                            item.file,
                            constructor.span,
                            item.file,
                            *previous,
                        ));
                    } else {
                        signatures.push((&constructor.types, constructor.span));
                    }
                }
            }
            (ResolvedItemKind::Class { interfaces }, Item::Class(_)) => {
                let mut resolved = vec![];
                for interface in interfaces {
                    if let Some((_, previous)) =
                        resolved.iter().find(|(ty, _)| ty == &&interface.ty)
                    {
                        diagnostics.push(duplicate_error(
                            "class interface",
                            &display_named_type(&interface.ty),
                            item.file,
                            interface.span,
                            item.file,
                            *previous,
                        ));
                    } else {
                        resolved.push((&interface.ty, interface.span));
                    }
                }
            }
            (
                ResolvedItemKind::Interface {
                    requires,
                    methods,
                    properties,
                },
                Item::Interface(_),
            ) => {
                let mut resolved = vec![];
                for require in requires {
                    if let Some((_, previous)) = resolved.iter().find(|(ty, _)| ty == &&require.ty)
                    {
                        diagnostics.push(duplicate_error(
                            "required interface",
                            &display_named_type(&require.ty),
                            item.file,
                            require.span,
                            item.file,
                            *previous,
                        ));
                    } else {
                        resolved.push((&require.ty, require.span));
                    }
                }

                let mut overloads = HashMap::<&str, Vec<(&ResolvedSignature, Span)>>::new();
                for method in methods {
                    let signatures = overloads.entry(&method.name).or_default();
                    if let Some((_, previous)) = signatures
                        .iter()
                        .find(|(signature, _)| *signature == &method.signature)
                    {
                        diagnostics.push(duplicate_error(
                            "method",
                            &method.name,
                            item.file,
                            method.span,
                            item.file,
                            *previous,
                        ));
                    } else {
                        signatures.push((&method.signature, method.span));
                    }
                }

                let mut resolved = HashMap::<&str, ResolvedPropertyState>::new();
                for property in properties {
                    if let Some(previous) = resolved.get_mut(property.name.as_str()) {
                        if previous.ty != property.ty
                            || previous.get && property.get
                            || previous.set && property.set
                        {
                            diagnostics.push(duplicate_error(
                                "property",
                                &property.name,
                                item.file,
                                property.span,
                                item.file,
                                previous.span,
                            ));
                        } else {
                            previous.get |= property.get;
                            previous.set |= property.set;
                        }
                    } else {
                        resolved.insert(
                            &property.name,
                            ResolvedPropertyState {
                                get: property.get,
                                set: property.set,
                                ty: property.ty.clone(),
                                span: property.span,
                            },
                        );
                    }
                }
            }
            _ => {}
        }
    }

    diagnostics.extend(validate_attribute_targets(model, reference));
    diagnostics
}

fn validate_attribute_targets(
    model: &ResolvedModel,
    reference: &metadata::reader::Index,
) -> Vec<Error> {
    let mut diagnostics = vec![];
    let mut applied = HashMap::<(usize, &str, &str), &ResolvedAttribute>::new();

    for attribute in &model.attributes {
        let Some(usage) = attribute_usage(model, reference, &attribute.value.type_name) else {
            continue;
        };
        let target = attribute_target_bit(attribute.target);
        if usage.valid_on & target == 0 {
            let start = attribute.span.start();
            let end = attribute.span.end();
            diagnostics.push(
                Error::new(
                    &format!(
                        "attribute `{}.{}` cannot be applied to {}",
                        attribute.value.type_name.namespace,
                        attribute.value.type_name.name,
                        attribute_target_name(attribute.target)
                    ),
                    &model.items[attribute.owner].file.source,
                    start.line,
                    start.column,
                )
                .with_code("RDL0006")
                .with_primary_label(
                    Label::primary(
                        &model.items[attribute.owner].file.source,
                        start.line,
                        start.column,
                    )
                    .with_end(end.line, end.column)
                    .with_message("attribute is not valid on this target"),
                ),
            );
        }

        if !usage.allow_multiple {
            let key = (
                attribute.site,
                attribute.value.type_name.namespace.as_str(),
                attribute.value.type_name.name.as_str(),
            );
            if let Some(previous) = applied.insert(key, attribute) {
                diagnostics.push(repeated_attribute_error(model, attribute, previous));
            }
        }
    }

    diagnostics
}

struct AttributeUsage {
    valid_on: u32,
    allow_multiple: bool,
}

fn attribute_usage(
    model: &ResolvedModel,
    reference: &metadata::reader::Index,
    type_name: &metadata::TypeName,
) -> Option<AttributeUsage> {
    if let Some(item) = model.items.iter().find(|item| {
        item.namespace == type_name.namespace
            && item.item.to_string() == type_name.name
            && matches!(item.item, Item::Attribute(_))
    }) {
        let valid_on = model
            .attributes
            .iter()
            .filter(|attribute| attribute.owner == item.id)
            .find_map(|attribute| attribute_usage_value(&attribute.value))?;
        let allow_multiple = model.attributes.iter().any(|attribute| {
            attribute.owner == item.id && is_allow_multiple_attribute(&attribute.value)
        });
        return Some(AttributeUsage {
            valid_on,
            allow_multiple,
        });
    }

    reference
        .get(&type_name.namespace, &type_name.name)
        .find_map(|def| {
            let valid_on = metadata::HasAttributes::attributes(&def)
                .find(|attribute| {
                    attribute.namespace() == "Windows.Foundation.Metadata"
                        && attribute.name() == "AttributeUsageAttribute"
                })
                .and_then(|attribute| {
                    attribute
                        .value()
                        .first()
                        .and_then(|(_, value)| attribute_target_value(value))
                })?;
            let allow_multiple = metadata::HasAttributes::attributes(&def).any(|attribute| {
                attribute.namespace() == "Windows.Foundation.Metadata"
                    && attribute.name() == "AllowMultipleAttribute"
            });
            Some(AttributeUsage {
                valid_on,
                allow_multiple,
            })
        })
}

fn attribute_usage_value(attribute: &AttributeRef) -> Option<u32> {
    if attribute.type_name.namespace != "Windows.Foundation.Metadata"
        || attribute.type_name.name != "AttributeUsageAttribute"
    {
        return None;
    }
    attribute
        .args
        .first()
        .and_then(|(_, value)| attribute_target_value(value))
}

fn is_allow_multiple_attribute(attribute: &AttributeRef) -> bool {
    attribute.type_name.namespace == "Windows.Foundation.Metadata"
        && attribute.type_name.name == "AllowMultipleAttribute"
}

fn repeated_attribute_error(
    model: &ResolvedModel,
    attribute: &ResolvedAttribute,
    previous: &ResolvedAttribute,
) -> Error {
    let file = model.items[attribute.owner].file;
    let start = attribute.span.start();
    let end = attribute.span.end();
    let previous_start = previous.span.start();
    let previous_end = previous.span.end();
    let name = format!(
        "{}.{}",
        attribute.value.type_name.namespace, attribute.value.type_name.name
    );

    Error::new(
        &format!("attribute `{name}` cannot be applied more than once"),
        &file.source,
        start.line,
        start.column,
    )
    .with_code("RDL0006")
    .with_primary_label(
        Label::primary(&file.source, start.line, start.column)
            .with_end(end.line, end.column)
            .with_message("repeated attribute"),
    )
    .with_label(
        Label::secondary(
            &file.source,
            previous_start.line,
            previous_start.column,
            "first applied here",
        )
        .with_end(previous_end.line, previous_end.column),
    )
}

fn attribute_target_value(value: &metadata::Value) -> Option<u32> {
    match value {
        metadata::Value::EnumValue(_, value) => attribute_target_value(value),
        metadata::Value::I32(value) => Some(*value as u32),
        metadata::Value::U32(value) => Some(*value),
        _ => None,
    }
}

fn attribute_target_bit(target: AttributeTarget) -> u32 {
    match target {
        AttributeTarget::Delegate => 1,
        AttributeTarget::Enum => 2,
        AttributeTarget::Field => 8,
        AttributeTarget::Interface => 16,
        AttributeTarget::Method => 64,
        AttributeTarget::Parameter => 128,
        AttributeTarget::RuntimeClass => 512,
        AttributeTarget::Struct => 1024,
        AttributeTarget::InterfaceImpl => 2048,
    }
}

fn attribute_target_name(target: AttributeTarget) -> &'static str {
    match target {
        AttributeTarget::Delegate => "a delegate",
        AttributeTarget::Enum => "an enum",
        AttributeTarget::Field => "a field",
        AttributeTarget::Interface => "an interface",
        AttributeTarget::Method => "a method",
        AttributeTarget::Parameter => "a parameter",
        AttributeTarget::RuntimeClass => "a runtime class",
        AttributeTarget::Struct => "a struct",
        AttributeTarget::InterfaceImpl => "an interface implementation",
    }
}

struct ResolvedPropertyState {
    get: bool,
    set: bool,
    ty: metadata::Type,
    span: Span,
}

fn validate_namespace_symbols(members: &Namespace<'_>) -> Result<(), Error> {
    let mut symbols = HashMap::<String, (&str, &File, Span)>::new();

    for (kind, entries) in [
        ("type", &members.types),
        ("function", &members.functions),
        ("constant", &members.constants),
    ] {
        for (name, variants) in entries {
            let Some((file, item)) = variants.first() else {
                continue;
            };

            if let Some((previous_kind, previous_file, previous_span)) = symbols.get(name) {
                if *previous_kind != kind {
                    return duplicate(
                        "symbol",
                        name,
                        file,
                        item.name_span(),
                        previous_file,
                        *previous_span,
                    );
                }
            } else {
                symbols.insert(name.clone(), (kind, file, item.name_span()));
            }
        }
    }

    Ok(())
}

fn validate_variants(kind: &str, name: &str, variants: &[(&File, &Item)]) -> Result<(), Error> {
    for current in 1..variants.len() {
        let current_arch = item_arch(variants[current].1);
        if matches!(current_arch, Arch::Invalid) {
            continue;
        }

        for previous in 0..current {
            let previous_arch = item_arch(variants[previous].1);
            if matches!(previous_arch, Arch::Invalid) {
                continue;
            }

            let disjoint = matches!(
                (&previous_arch, &current_arch),
                (Arch::Valid(left), Arch::Valid(right)) if left & right == 0
            );

            if !disjoint {
                return duplicate(
                    kind,
                    name,
                    variants[current].0,
                    variants[current].1.name_span(),
                    variants[previous].0,
                    variants[previous].1.name_span(),
                );
            }
        }
    }

    Ok(())
}

fn item_arch(item: &Item) -> Arch {
    let Some(attr) = item
        .attrs()
        .iter()
        .find(|attr| attr.path().is_ident("arch"))
    else {
        return Arch::None;
    };

    let Ok(expr) = attr.parse_args::<syn::Expr>() else {
        return Arch::Invalid;
    };

    parse_arch_bitmask(&expr).map_or(Arch::Invalid, Arch::Valid)
}

fn validate_item(file: &File, item: &Item) -> Result<(), Error> {
    validate_common_controls(file, item.attrs())?;
    if !matches!(
        item,
        Item::Attribute(_) | Item::Enum(_) | Item::Interface(_) | Item::Struct(_)
    ) {
        validate_profile_controls(file, item.attrs())?;
    }

    match item {
        Item::Attribute(item) => validate_attribute(file, item),
        Item::Callback(item) => {
            validate_abi(file, item.abi.as_ref())?;
            reject_generics(file, &item.sig.generics, "callbacks")?;
            reject_variadic(file, &item.sig, "callbacks")?;
            validate_signature_params(file, &item.sig)
        }
        Item::Class(item) => validate_class(file, item),
        Item::Delegate(item) => {
            validate_guid_controls(file, &item.attrs)?;
            validate_type_generics(file, &item.sig.generics, "delegates")?;
            reject_variadic(file, &item.sig, "delegates")?;
            validate_signature_params(file, &item.sig)
        }
        Item::Enum(item) => validate_enum(file, item),
        Item::Fn(item) => {
            validate_abi(file, item.abi.as_ref())?;
            validate_library_control(file, item)?;
            reject_generics(file, &item.sig.generics, "functions")?;
            validate_signature_params(file, &item.sig)
        }
        Item::Interface(item) => validate_interface(file, item),
        Item::Struct(item) => {
            validate_layout_controls(file, &item.attrs)?;
            validate_fields(file, &item.fields)
        }
        Item::Union(item) => {
            validate_layout_controls(file, &item.attrs)?;
            validate_fields(file, &item.fields)
        }
        Item::Const(item) => validate_guid_controls(file, &item.attrs),
        Item::Module(_) | Item::Typedef(_) => Ok(()),
    }
}

fn validate_attribute(file: &File, item: &Attribute) -> Result<(), Error> {
    let mut properties = HashMap::<String, Span>::new();
    for (name, _) in &item.properties {
        check_name(file, "attribute property", name, &mut properties)?;
    }

    for method in &item.methods {
        validate_bare_params(file, method)?;
        if let Some(variadic) = &method.variadic {
            return unsupported(
                file,
                variadic.span(),
                "variadic attribute constructors are not supported",
            );
        }
        if let syn::ReturnType::Type(_, ty) = &method.output {
            return unsupported(
                file,
                ty.span(),
                "attribute constructors cannot return a value",
            );
        }
    }

    Ok(())
}

fn validate_class(_file: &File, _item: &Class) -> Result<(), Error> {
    Ok(())
}

fn validate_enum(file: &File, item: &Enum) -> Result<(), Error> {
    validate_control_once(file, &item.attrs, "repr", true)?;
    validate_control_once(file, &item.attrs, "flags", false)?;

    let Some(repr) = item.attrs.iter().find(|attr| attr.path().is_ident("repr")) else {
        return invalid_control(file, item.token.span(), "`repr` attribute not found");
    };
    if repr.parse_args::<syn::Path>().is_err() {
        return invalid_control(file, repr.span(), "`repr` requires a type path");
    }

    let mut variants = HashMap::<String, Span>::new();
    for variant in &item.variants {
        if !matches!(variant.fields, syn::Fields::Unit) {
            return unsupported(
                file,
                variant.fields.span(),
                "enum variants with fields are not supported",
            );
        }
        check_name(file, "enum variant", &variant.ident, &mut variants)?;
    }
    Ok(())
}

fn validate_interface(file: &File, item: &Interface) -> Result<(), Error> {
    validate_guid_controls(file, &item.attrs)?;
    validate_type_generics(file, &item.generics, "interfaces")?;

    let mut events = HashMap::<String, Span>::new();
    let mut member_kinds = HashMap::<String, (&str, Span)>::new();

    for member in &item.members {
        let (kind, name, span) = match member {
            InterfaceMember::Method(method) => (
                "method",
                method.sig.ident.to_string(),
                method.sig.ident.span(),
            ),
            InterfaceMember::Property(property) => {
                ("property", property.name.to_string(), property.name.span())
            }
            InterfaceMember::Event(event) => ("event", event.name.to_string(), event.name.span()),
        };

        if let Some((previous_kind, previous_span)) = member_kinds.get(&name) {
            if *previous_kind != kind {
                return duplicate("interface member", &name, file, span, file, *previous_span);
            }
        } else {
            member_kinds.insert(name, (kind, span));
        }

        match member {
            InterfaceMember::Method(method) => {
                validate_control_once(file, &method.attrs, "special", false)?;
                reject_generics(file, &method.sig.generics, "interface methods")?;
                reject_variadic(file, &method.sig, "interface methods")?;
                validate_signature_params(file, &method.sig)?;
            }
            InterfaceMember::Property(property) => validate_property_controls(file, property)?,
            InterfaceMember::Event(event) => {
                if let Some(attr) = event.attrs.first() {
                    return unsupported(
                        file,
                        attr.span(),
                        "attributes on event shorthand are not represented",
                    );
                }
                check_name(file, "event", &event.name, &mut events)?;
            }
        }
    }

    Ok(())
}

fn validate_fields(file: &File, fields: &[Field]) -> Result<(), Error> {
    let mut names = HashMap::<String, Span>::new();

    for field in fields {
        check_name(file, "field", &field.name, &mut names)?;

        let mut bitfields = HashMap::<String, Span>::new();
        for member in &field.bitfields {
            if let Some(name) = &member.name {
                check_name(file, "bit-field member", name, &mut bitfields)?;
            }
        }

        if let FieldType::Nested(record) = &field.ty {
            validate_layout_controls(file, &record.attrs)?;
            validate_fields(file, &record.fields)?;
        }
    }

    Ok(())
}

fn validate_generic_names(file: &File, generics: &syn::Generics) -> Result<(), Error> {
    let mut names = HashMap::<String, Span>::new();
    for param in &generics.params {
        match param {
            syn::GenericParam::Type(param) => {
                check_name(file, "generic parameter", &param.ident, &mut names)?;
            }
            syn::GenericParam::Const(param) => {
                check_name(file, "generic parameter", &param.ident, &mut names)?;
            }
            syn::GenericParam::Lifetime(param) => {
                let name = param.lifetime.ident.to_string();
                if let Some(previous) = names.insert(name.clone(), param.lifetime.ident.span()) {
                    return duplicate(
                        "generic parameter",
                        &name,
                        file,
                        param.lifetime.ident.span(),
                        file,
                        previous,
                    );
                }
            }
        }
    }
    Ok(())
}

fn reject_generics(file: &File, generics: &syn::Generics, target: &str) -> Result<(), Error> {
    if let Some(param) = generics.params.first() {
        return unsupported(
            file,
            param.span(),
            &format!("generic parameters are not supported on {target}"),
        );
    }
    Ok(())
}

fn validate_type_generics(
    file: &File,
    generics: &syn::Generics,
    target: &str,
) -> Result<(), Error> {
    validate_generic_names(file, generics)?;

    for param in &generics.params {
        let syn::GenericParam::Type(param) = param else {
            return unsupported(
                file,
                param.span(),
                &format!("only type generic parameters are supported on {target}"),
            );
        };

        if let Some(attr) = param.attrs.first() {
            return unsupported(
                file,
                attr.span(),
                "attributes on generic parameters are not represented",
            );
        }
        if let Some(bound) = param.bounds.first() {
            return unsupported(
                file,
                bound.span(),
                "generic parameter bounds are not represented",
            );
        }
        if let Some(default) = &param.default {
            return unsupported(
                file,
                default.span(),
                "generic parameter defaults are not represented",
            );
        }
    }

    Ok(())
}

fn reject_variadic(file: &File, signature: &syn::Signature, target: &str) -> Result<(), Error> {
    if let Some(variadic) = &signature.variadic {
        unsupported(
            file,
            variadic.span(),
            &format!("variadic parameters are not supported on {target}"),
        )
    } else {
        Ok(())
    }
}

fn validate_signature_params(file: &File, signature: &syn::Signature) -> Result<(), Error> {
    let mut names = HashMap::<String, Span>::new();
    for input in &signature.inputs {
        if let syn::FnArg::Typed(param) = input
            && let syn::Pat::Ident(name) = param.pat.as_ref()
        {
            validate_param_controls(file, &param.attrs)?;
            check_name(file, "parameter", &name.ident, &mut names)?;
        }
    }
    Ok(())
}

fn validate_bare_params(file: &File, signature: &syn::TypeBareFn) -> Result<(), Error> {
    let mut names = HashMap::<String, Span>::new();
    for input in &signature.inputs {
        validate_param_controls(file, &input.attrs)?;
        if let Some((name, _)) = &input.name {
            check_name(file, "parameter", name, &mut names)?;
        }
    }
    Ok(())
}

fn validate_common_controls(file: &File, attrs: &[syn::Attribute]) -> Result<(), Error> {
    validate_control_once(file, attrs, "arch", true)?;

    if let Some(attr) = attrs.iter().find(|attr| attr.path().is_ident("arch")) {
        let Ok(expr) = attr.parse_args::<syn::Expr>() else {
            return invalid_control(
                file,
                attr.span(),
                "`arch` requires architecture arguments such as `#[arch(X86)]`",
            );
        };
        if parse_arch_bitmask(&expr).is_none() {
            return invalid_control(
                file,
                attr.span(),
                "invalid `arch` value; expected `X86`, `X64`, `Arm64`, or a `|`-combination",
            );
        }
    }

    Ok(())
}

fn validate_profile_controls(file: &File, attrs: &[syn::Attribute]) -> Result<(), Error> {
    validate_control_once(file, attrs, "win32", false)?;
    validate_control_once(file, attrs, "winrt", false)
}

fn validate_guid_controls(file: &File, attrs: &[syn::Attribute]) -> Result<(), Error> {
    validate_control_once(file, attrs, "guid", true)?;
    validate_control_once(file, attrs, "no_guid", false)?;

    let guid = attrs.iter().find(|attr| attr.path().is_ident("guid"));
    let no_guid = attrs.iter().find(|attr| attr.path().is_ident("no_guid"));
    if guid.is_some()
        && let Some(no_guid) = no_guid
    {
        return invalid_control(
            file,
            no_guid.span(),
            "`guid` and `no_guid` attributes are mutually exclusive",
        );
    }
    if let Some(attr) = guid {
        let Ok(literal) = attr.parse_args::<syn::LitInt>() else {
            return invalid_control(file, attr.span(), "`guid` requires a single u128 literal");
        };
        if parse_guid_u128(&literal).is_err() {
            return invalid_control(file, attr.span(), "invalid u128 literal in `guid`");
        }
    }

    Ok(())
}

fn validate_layout_controls(file: &File, attrs: &[syn::Attribute]) -> Result<(), Error> {
    for name in ["packed", "align"] {
        validate_control_once(file, attrs, name, true)?;
        if let Some(attr) = attrs.iter().find(|attr| attr.path().is_ident(name)) {
            let Ok(literal) = attr.parse_args::<syn::LitInt>() else {
                return invalid_control(
                    file,
                    attr.span(),
                    &format!("`{name}` requires an integer argument"),
                );
            };
            if literal.base10_parse::<u16>().is_err() {
                return invalid_control(
                    file,
                    attr.span(),
                    &format!("`{name}` size must be a valid u16"),
                );
            }
        }
    }
    Ok(())
}

fn validate_library_control(file: &File, item: &Fn) -> Result<(), Error> {
    validate_control_once(file, &item.attrs, "library", true)?;
    let Some(attr) = item
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("library"))
    else {
        return invalid_control(file, item.sig.span(), "`library` attribute not found");
    };
    if let Err(error) = parse_library_attribute(attr) {
        return invalid_control(file, error.span(), &error.to_string());
    }
    Ok(())
}

fn validate_property_controls(file: &File, property: &Property) -> Result<(), Error> {
    validate_control_once(file, &property.attrs, "get", false)?;
    validate_control_once(file, &property.attrs, "set", false)?;

    for attr in &property.attrs {
        if !attr.path().is_ident("get") && !attr.path().is_ident("set") {
            return invalid_control(
                file,
                attr.span(),
                "only `get` and `set` attributes are supported on properties",
            );
        }
    }
    if property
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("get"))
        && property
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("set"))
    {
        return invalid_control(
            file,
            property.name.span(),
            "property cannot have both `get` and `set` attributes",
        );
    }
    Ok(())
}

fn validate_param_controls(file: &File, attrs: &[syn::Attribute]) -> Result<(), Error> {
    validate_control_once(file, attrs, "r#in", false)?;
    validate_control_once(file, attrs, "out", false)?;
    validate_control_once(file, attrs, "opt", false)
}

fn validate_abi(file: &File, abi: Option<&syn::LitStr>) -> Result<(), Error> {
    if let Some(abi) = abi
        && !matches!(abi.value().as_str(), "system" | "C" | "fastcall")
    {
        return invalid_control(file, abi.span(), "function ABI is not supported");
    }
    Ok(())
}

fn validate_control_once(
    file: &File,
    attrs: &[syn::Attribute],
    name: &str,
    accepts_arguments: bool,
) -> Result<(), Error> {
    let mut found = None;
    for attr in attrs.iter().filter(|attr| attr.path().is_ident(name)) {
        if let Some(previous) = found {
            return invalid_control_with_previous(
                file,
                attr.span(),
                previous,
                &format!("duplicate `{}` attribute", name.trim_start_matches("r#")),
            );
        }
        found = Some(attr.span());
        if !accepts_arguments && !matches!(attr.meta, syn::Meta::Path(_)) {
            return invalid_control(
                file,
                attr.span(),
                &format!(
                    "`{}` attribute does not accept arguments",
                    name.trim_start_matches("r#")
                ),
            );
        }
    }
    Ok(())
}

fn check_name(
    file: &File,
    kind: &str,
    name: &syn::Ident,
    names: &mut HashMap<String, Span>,
) -> Result<(), Error> {
    let name_string = name.to_string();
    if let Some(previous) = names.insert(name_string.clone(), name.span()) {
        duplicate(kind, &name_string, file, name.span(), file, previous)
    } else {
        Ok(())
    }
}

fn duplicate<T>(
    kind: &str,
    name: &str,
    current_file: &File,
    current_span: Span,
    previous_file: &File,
    previous_span: Span,
) -> Result<T, Error> {
    Err(duplicate_error(
        kind,
        name,
        current_file,
        current_span,
        previous_file,
        previous_span,
    ))
}

fn duplicate_error(
    kind: &str,
    name: &str,
    current_file: &File,
    current_span: Span,
    previous_file: &File,
    previous_span: Span,
) -> Error {
    let current_start = current_span.start();
    let current_end = current_span.end();
    let previous_start = previous_span.start();
    let previous_end = previous_span.end();
    let primary_message = format!("duplicate {kind}");

    Error::new(
        &format!("duplicate {kind} `{name}`"),
        &current_file.source,
        current_start.line,
        current_start.column,
    )
    .with_code(DUPLICATE_CODE)
    .with_primary_label(
        Label::primary(
            &current_file.source,
            current_start.line,
            current_start.column,
        )
        .with_end(current_end.line, current_end.column)
        .with_message(&primary_message),
    )
    .with_label(
        Label::secondary(
            &previous_file.source,
            previous_start.line,
            previous_start.column,
            "first declared here",
        )
        .with_end(previous_end.line, previous_end.column),
    )
}

fn unsupported<T>(file: &File, span: Span, message: &str) -> Result<T, Error> {
    let start = span.start();
    let end = span.end();

    Err(Error::new(message, &file.source, start.line, start.column)
        .with_code(UNREPRESENTABLE_CODE)
        .with_primary_label(
            Label::primary(&file.source, start.line, start.column)
                .with_end(end.line, end.column)
                .with_message("not represented in metadata"),
        ))
}

fn invalid_control<T>(file: &File, span: Span, message: &str) -> Result<T, Error> {
    Err(control_error(&file.source, span, message))
}

pub(super) fn control_error(source: &str, span: Span, message: &str) -> Error {
    let start = span.start();
    let end = span.end();

    Error::new(message, source, start.line, start.column)
        .with_code(CONTROL_CODE)
        .with_primary_label(
            Label::primary(source, start.line, start.column)
                .with_end(end.line, end.column)
                .with_message("invalid control attribute"),
        )
}

fn invalid_control_with_previous<T>(
    file: &File,
    span: Span,
    previous: Span,
    message: &str,
) -> Result<T, Error> {
    Err(duplicate_control_error(
        &file.source,
        span,
        previous,
        message,
    ))
}

pub(super) fn duplicate_control_error(
    source: &str,
    span: Span,
    previous: Span,
    message: &str,
) -> Error {
    let start = span.start();
    let end = span.end();
    let previous_start = previous.start();
    let previous_end = previous.end();

    Error::new(message, source, start.line, start.column)
        .with_code(CONTROL_CODE)
        .with_primary_label(
            Label::primary(source, start.line, start.column)
                .with_end(end.line, end.column)
                .with_message("duplicate control attribute"),
        )
        .with_label(
            Label::secondary(
                source,
                previous_start.line,
                previous_start.column,
                "first applied here",
            )
            .with_end(previous_end.line, previous_end.column),
        )
}
