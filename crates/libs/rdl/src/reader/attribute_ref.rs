use super::*;

/// A parsed and validated reference to a custom attribute defined in metadata or RDL.
///
/// Built-in RDL attributes (`win32`, `winrt`, `repr`, `link`, etc.) are
/// handled separately by the individual encode functions and are never represented here.
pub struct AttributeRef {
    pub type_name: metadata::TypeName,
    pub args: Vec<(String, metadata::Value)>,
}

/// Collected information about an attribute type: constructor overloads and named
/// instance-field properties (e.g. `version: u32`).
struct AttributeInfo {
    type_name: metadata::TypeName,
    constructors: Vec<Vec<metadata::Type>>,
    /// Named instance fields: `(field_name, field_type)`.
    properties: Vec<(String, metadata::Type)>,
}

/// Finds an attribute type (as `<ident>Attribute`) in the encoder's index and
/// reference, returning its full `AttributeInfo`.
fn find_attribute_type(encoder: &Encoder, path: &syn::Path) -> Option<AttributeInfo> {
    // Convert Rust-style `A::B::C` path to metadata-style `A.B.C` namespace + name.
    let mut segments: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();

    let name = segments.pop()?;
    let attr_name = format!("{}Attribute", name);

    let explicit_namespace = if segments.is_empty() {
        None
    } else {
        Some(segments.join("."))
    };

    // Determine the candidate namespaces to search.
    let namespaces: Vec<String> = if let Some(ns) = explicit_namespace {
        // Fully-qualified: only search the given namespace.
        vec![ns]
    } else {
        // Unqualified: search the current namespace and every parent up to the root,
        // mirroring the behaviour of encode_path.
        let parts: Vec<&str> = encoder.namespace.split('.').collect();
        let mut nss: Vec<String> = (1..=parts.len())
            .rev()
            .map(|len| parts[..len].join("."))
            .collect();
        // Also search glob use declaration namespaces as a last resort.
        for use_item in &encoder.file.uses {
            if let Some(ns) = glob_use_namespace(use_item) {
                if !nss.contains(&ns) {
                    nss.push(ns);
                }
            }
        }
        nss
    };

    for ns in &namespaces {
        if let Some(info) = find_in_reference(encoder, ns, &attr_name)
            .or_else(|| find_in_index(encoder, ns, &attr_name))
        {
            return Some(info);
        }
    }

    None
}

/// Searches the metadata reference for a type with the given namespace/name that
/// has `TypeCategory::Attribute`, and returns its full `AttributeInfo`.
fn find_in_reference(encoder: &Encoder, namespace: &str, attr_name: &str) -> Option<AttributeInfo> {
    let mut constructors = vec![];
    let mut properties = vec![];

    for typedef in encoder.reference.get(namespace, attr_name) {
        if typedef.category() == metadata::reader::TypeCategory::Attribute {
            for method in typedef.methods() {
                if method.name() == ".ctor" {
                    let sig = method.signature(&[]);
                    constructors.push(sig.types);
                }
            }
            for field in typedef.fields() {
                let flags = field.flags();
                // Named instance fields only – skip literals (enum variants), statics, and
                // special-name fields like the enum discriminant (value__).
                if flags.contains(metadata::FieldAttributes::Public)
                    && !flags.contains(metadata::FieldAttributes::Static)
                    && !flags.contains(metadata::FieldAttributes::Literal)
                    && !flags.contains(metadata::FieldAttributes::SpecialName)
                {
                    properties.push((field.name().to_string(), field.ty()));
                }
            }
        }
    }

    if constructors.is_empty() && properties.is_empty() {
        None
    } else {
        Some(AttributeInfo {
            type_name: metadata::TypeName::named(namespace, attr_name),
            constructors,
            properties,
        })
    }
}

/// Searches the RDL index for an `Item::Attribute` with the given namespace/name
/// and returns its full `AttributeInfo`.
fn find_in_index(encoder: &Encoder, namespace: &str, attr_name: &str) -> Option<AttributeInfo> {
    let (_, item) = *encoder
        .index
        .namespaces
        .get(namespace)?
        .types
        .get(attr_name)?;
    let Item::Attribute(attr_item) = item else {
        return None;
    };

    let mut constructors = vec![];
    for method in &attr_item.methods {
        let types: Result<Vec<_>, _> = method
            .inputs
            .iter()
            .map(|arg| encode_type_in_attr_ns(encoder, namespace, &arg.ty))
            .collect();
        if let Ok(types) = types {
            constructors.push(types);
        }
    }

    let mut properties = vec![];
    for (prop_name, prop_ty) in &attr_item.properties {
        if let Ok(ty) = encode_type_in_attr_ns(encoder, namespace, prop_ty) {
            properties.push((prop_name.to_string(), ty));
        }
    }

    Some(AttributeInfo {
        type_name: metadata::TypeName::named(namespace, attr_name),
        constructors,
        properties,
    })
}

/// Positional and named arguments split from a raw argument list.
struct SplitArgs<'a> {
    positional: Vec<&'a syn::Expr>,
    named: Vec<(String, &'a syn::Expr)>,
}

/// Splits a list of `syn::Expr` argument expressions into positional arguments
/// (emitted before any named argument) and named `name = value` arguments.
///
/// Returns an error if a positional argument follows a named one, or if the
/// left-hand side of an `=` expression is not a plain identifier.
fn split_args<'a>(encoder: &Encoder, args: &'a [syn::Expr]) -> Result<SplitArgs<'a>, Error> {
    let mut positional: Vec<&syn::Expr> = vec![];
    let mut named: Vec<(String, &syn::Expr)> = vec![];

    for arg in args {
        if let syn::Expr::Assign(syn::ExprAssign { left, right, .. }) = arg {
            // `name = value` — named argument.
            if let syn::Expr::Path(syn::ExprPath { path, .. }) = left.as_ref() {
                if path.leading_colon.is_none() && path.segments.len() == 1 {
                    named.push((path.segments[0].ident.to_string(), right.as_ref()));
                    continue;
                }
            }
            return encoder.err(arg, "expected `name = value` for named attribute argument");
        }
        // Positional argument – must not come after a named one.
        if !named.is_empty() {
            return encoder.err(
                arg,
                "positional attribute arguments must come before named arguments",
            );
        }
        positional.push(arg);
    }

    Ok(SplitArgs { positional, named })
}

/// Tries to match the positional arguments against the constructor overloads,
/// then validates and encodes any named arguments against the attribute's
/// instance-field properties.
///
/// Returns the combined ordered `(name, value)` list ready for the blob writer.
fn resolve_attribute_args(
    encoder: &Encoder,
    attr: &syn::Attribute,
    info: &AttributeInfo,
    positional: &[&syn::Expr],
    named: &[(String, &syn::Expr)],
) -> Result<Vec<(String, metadata::Value)>, Error> {
    // Match positional args against constructor overloads.
    let mut last_type_error: Option<Error> = None;
    let mut ctor_values: Option<Vec<(String, metadata::Value)>> = None;

    for types in &info.constructors {
        if types.len() != positional.len() {
            continue;
        }

        let mut values = vec![];
        let mut type_error: Option<Error> = None;

        for (ty, arg) in types.iter().zip(positional.iter()) {
            match encode_attr_value(encoder, ty, arg) {
                Ok(v) => values.push((String::new(), v)),
                Err(e) => {
                    type_error = Some(e);
                    break;
                }
            }
        }

        match type_error {
            None => {
                ctor_values = Some(values);
                break;
            }
            Some(e) => last_type_error = Some(e),
        }
    }

    let mut result = match ctor_values {
        Some(v) => v,
        None => {
            if let Some(err) = last_type_error {
                return Err(err);
            } else {
                return encoder.err(attr, "no matching attribute constructor found");
            }
        }
    };

    // Encode named args, validating each against the attribute's declared properties.
    for (name, value_expr) in named {
        let prop_ty = info
            .properties
            .iter()
            .find(|(pname, _)| pname == name)
            .map(|(_, ty)| ty)
            .ok_or_else(|| encoder.error(attr, &format!("attribute has no property `{name}`")))?;
        let value = encode_attr_value(encoder, prop_ty, value_expr)?;
        result.push((name.clone(), value));
    }

    Ok(result)
}

/// Converts a `syn::Expr` to a `metadata::Value` for the given constructor
/// parameter type.  Extends the basic `encode_value` helper with support for
/// string literals, `System.Type` (serialised as a UTF-8 string), and enum
/// types (accepting an unqualified variant-name identifier).
fn encode_attr_value(
    encoder: &Encoder,
    ty: &metadata::Type,
    value: &syn::Expr,
) -> Result<metadata::Value, Error> {
    match ty {
        metadata::Type::String => match value {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) => Ok(metadata::Value::Utf8(s.value())),
            _ => encoder.err(value, "expected string literal"),
        },
        metadata::Type::Name(tn) if tn == ("System", "Type") => match value {
            syn::Expr::Path(syn::ExprPath { path, .. }) => match encode_path(encoder, path)? {
                metadata::Type::Name(tn) => Ok(metadata::Value::TypeName(tn)),
                _ => encoder.err(value, "expected type name"),
            },
            _ => encoder.err(value, "expected type path"),
        },
        metadata::Type::Name(tn) => {
            // Single unqualified variant name, e.g. `Agile`.
            if let syn::Expr::Path(syn::ExprPath { path, .. }) = value {
                if path.leading_colon.is_none() && path.segments.len() == 1 {
                    let variant_name = path.segments[0].ident.to_string();
                    let inner = find_enum_variant_value(encoder, tn, &variant_name, value)?;
                    return Ok(metadata::Value::EnumValue(tn.clone(), Box::new(inner)));
                }
            }
            // The remaining forms are only valid for flag enums (those with FlagsAttribute
            // in metadata or `#[flags]` in RDL).
            if enum_is_flags(encoder, tn) {
                // Flags combination: `Flag1 | Flag2 | ...` written by the writer for flag enums.
                // Collect all variant names from the binary-OR chain and OR their values.
                if let Some(names) = collect_bitor_variants(value) {
                    let mut combined: i32 = 0;
                    for name in &names {
                        let inner = find_enum_variant_value(encoder, tn, name, value)?;
                        let v = match inner {
                            metadata::Value::I32(v) => v,
                            _ => {
                                return encoder
                                    .err(value, &format!("expected `{}` variant name", tn.name))
                            }
                        };
                        combined |= v;
                    }
                    return Ok(metadata::Value::EnumValue(
                        tn.clone(),
                        Box::new(metadata::Value::I32(combined)),
                    ));
                }
                // Numeric literal fallback for values that have no named representation
                // (e.g. `3967`).  This preserves round-trip fidelity for unknown flag values.
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(int),
                    ..
                }) = value
                {
                    if let Ok(v) = int.base10_parse::<i32>() {
                        return Ok(metadata::Value::EnumValue(
                            tn.clone(),
                            Box::new(metadata::Value::I32(v)),
                        ));
                    }
                }
            }
            encoder.err(value, &format!("expected `{}` variant name", tn.name))
        }
        _ => encode_value(encoder, ty, value),
    }
}

/// Collects the individual variant name identifiers from a chain of binary `|`
/// expressions such as `Flag1 | Flag2 | Flag3`.  Returns `None` if the
/// expression is not a pure chain of simple single-segment path expressions
/// joined by `|`.
///
/// Single-segment paths (e.g. bare `Class`) are handled by the caller before
/// this function is reached, so we only recognise true multi-part OR chains.
fn collect_bitor_variants(expr: &syn::Expr) -> Option<Vec<String>> {
    let mut names = Vec::new();
    collect_bitor_variants_inner(expr, &mut names)?;
    if names.len() >= 2 {
        Some(names)
    } else {
        None
    }
}

fn collect_bitor_variants_inner(expr: &syn::Expr, names: &mut Vec<String>) -> Option<()> {
    match expr {
        syn::Expr::Binary(syn::ExprBinary {
            left,
            op: syn::BinOp::BitOr(_),
            right,
            ..
        }) => {
            collect_bitor_variants_inner(left, names)?;
            collect_bitor_variants_inner(right, names)?;
            Some(())
        }
        syn::Expr::Path(syn::ExprPath { path, .. })
            if path.leading_colon.is_none() && path.segments.len() == 1 =>
        {
            names.push(path.segments[0].ident.to_string());
            Some(())
        }
        _ => None,
    }
}

/// Returns `true` if the enum type referred to by `tn` carries the flags marker —
/// either `System.FlagsAttribute` in a metadata reference or the `#[flags]`
/// attribute in the RDL index.
fn enum_is_flags(encoder: &Encoder, tn: &metadata::TypeName) -> bool {
    // Check in the metadata reference (external winmd files).
    for typedef in encoder.reference.get(&tn.namespace, &tn.name) {
        if typedef.category() == metadata::reader::TypeCategory::Enum
            && metadata::HasAttributes::attributes(&typedef).any(|attr| {
                attr.name() == "FlagsAttribute" && attr.ctor().parent().namespace() == "System"
            })
        {
            return true;
        }
    }
    // Check in the RDL index (types defined in current input files).
    if let Some(ns) = encoder.index.namespaces.get(&tn.namespace) {
        if let Some((_, Item::Enum(enum_item))) = ns.types.get(&tn.name) {
            if enum_item.attrs.iter().any(|a| a.path().is_ident("flags")) {
                return true;
            }
        }
    }
    false
}

/// Looks up the integer value of an enum variant by name, searching first the
/// metadata reference (external winmd files) then the RDL index.
fn find_enum_variant_value(
    encoder: &Encoder,
    tn: &metadata::TypeName,
    variant_name: &str,
    spanned: &syn::Expr,
) -> Result<metadata::Value, Error> {
    // Search in the metadata reference (external winmd files).
    for typedef in encoder.reference.get(&tn.namespace, &tn.name) {
        if typedef.category() == metadata::reader::TypeCategory::Enum {
            for field in typedef.fields() {
                if field.flags().contains(metadata::FieldAttributes::Literal)
                    && field.name() == variant_name
                {
                    if let Some(constant) = field.constant() {
                        // Attribute blobs always encode enum values as i32, so
                        // normalise both possible underlying types to I32.
                        return Ok(match constant.value() {
                            metadata::Value::I32(v) => metadata::Value::I32(v),
                            metadata::Value::U32(v) => metadata::Value::I32(v as i32),
                            other => {
                                return encoder.err(
                                    spanned,
                                    &format!("unsupported enum constant type: {other:?}"),
                                )
                            }
                        });
                    }
                }
            }
        }
    }

    // Search in the RDL index (types defined in current input files).
    if let Some(ns) = encoder.index.namespaces.get(&tn.namespace) {
        if let Some((_, Item::Enum(enum_item))) = ns.types.get(&tn.name) {
            for variant in &enum_item.variants {
                if variant.ident == variant_name {
                    if let Some((_, discriminant)) = &variant.discriminant {
                        // Attribute blobs encode enum values as I32.  Try I32 first,
                        // then fall back to U32 for repr(u32) enums whose values
                        // exceed i32::MAX (e.g. `All = 0xFFFF_FFFF`), converting to
                        // I32 via a bit-reinterpret cast — mirroring the metadata
                        // reference path above.
                        let result = encode_value(encoder, &metadata::Type::I32, discriminant)
                            .or_else(|_| {
                                encode_value(encoder, &metadata::Type::U32, discriminant).map(|v| {
                                    match v {
                                        metadata::Value::U32(n) => metadata::Value::I32(n as i32),
                                        other => other,
                                    }
                                })
                            });
                        return result;
                    }
                }
            }
        }
    }

    encoder.err(spanned, "enum variant not found")
}

/// Resolves a single `syn::Attribute` into a validated `AttributeRef`.
///
/// The caller is responsible for filtering out built-in RDL attributes before
/// calling this function.
pub fn resolve_attribute_ref(
    encoder: &Encoder,
    attr: &syn::Attribute,
) -> Result<AttributeRef, Error> {
    let path = attr.path();

    let info = find_attribute_type(encoder, path)
        .ok_or_else(|| encoder.error(attr, "attribute type not found"))?;

    // Parse the argument expressions.
    let raw_args: Vec<syn::Expr> = match &attr.meta {
        syn::Meta::Path(_) => vec![],
        syn::Meta::List(_) => attr
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated,
            )
            .map_err(|e| {
                let start = e.span().start();
                Error::new(
                    &e.to_string(),
                    &encoder.file.source,
                    start.line,
                    start.column,
                )
            })?
            .into_iter()
            .collect(),
        syn::Meta::NameValue(_) => {
            return encoder.err(attr, "attribute cannot use top-level `name = value` syntax");
        }
    };

    let split = split_args(encoder, &raw_args)?;

    let args = resolve_attribute_args(encoder, attr, &info, &split.positional, &split.named)?;

    Ok(AttributeRef {
        type_name: info.type_name,
        args,
    })
}

/// Emits a custom attribute onto `has_attribute` in the metadata output.
pub fn encode_named_attribute(
    encoder: &mut Encoder,
    has_attribute: metadata::writer::HasAttribute,
    attr_ref: &AttributeRef,
) {
    let attribute_typeref = encoder
        .output
        .TypeRef(&attr_ref.type_name.namespace, &attr_ref.type_name.name);

    let signature = metadata::Signature {
        flags: metadata::MethodCallAttributes::HASTHIS,
        return_type: metadata::Type::Void,
        // Only positional (empty-name) args belong in the constructor signature;
        // named args are encoded separately in the attribute blob named-args section.
        types: attr_ref
            .args
            .iter()
            .filter(|(name, _)| name.is_empty())
            .map(|(_, v)| v.ty())
            .collect(),
    };

    let ctor = encoder.output.MemberRef(
        ".ctor",
        &signature,
        metadata::writer::MemberRefParent::TypeRef(attribute_typeref),
    );

    encoder.output.Attribute(
        has_attribute,
        metadata::writer::AttributeType::MemberRef(ctor),
        &attr_ref.args,
    );
}

/// Iterates `attrs`, skipping the built-in RDL attributes listed in `skip` (as
/// well as the unconditionally-skipped `win32`/`winrt`), and resolves every
/// remaining attribute as an `AttributeRef` defined in the encoder's index or
/// reference.  Each resolved attribute is immediately emitted onto `has_attribute`.
///
/// Returns an error if any remaining attribute cannot be resolved.
pub fn encode_attrs(
    encoder: &mut Encoder,
    has_attribute: metadata::writer::HasAttribute,
    attrs: &[syn::Attribute],
    skip: &[&str],
) -> Result<(), Error> {
    for attr in attrs {
        let path = attr.path();

        // win32/winrt are handled by resolve_winrt and never become metadata attributes.
        if path.is_ident("win32") || path.is_ident("winrt") {
            continue;
        }

        // Element-specific built-in attributes handled elsewhere.
        if skip.iter().any(|s| path.is_ident(s)) {
            continue;
        }

        let attr_ref = resolve_attribute_ref(encoder, attr)?;
        encode_named_attribute(encoder, has_attribute, &attr_ref);
    }

    Ok(())
}

#[test]
#[should_panic(expected = "error: attribute type not found\n --> .rdl:4:5")]
fn unknown_attribute_errors() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    #[Unknown(42)]
    class MyClass {}
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: value not valid\n --> .rdl:6:11")]
fn wrong_arg_type_errors() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    attribute FooAttribute { fn(value: u32); }

    #[Foo("not a u32")]
    class MyClass {}
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: expected `Color` variant name\n --> .rdl:8:15")]
fn enum_arg_requires_variant_name() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    #[repr(i32)]
    enum Color { Red = 0, Green = 1, Blue = 2, }
    attribute PaletteAttribute { fn(value: Color); }

    #[Palette(1)]
    class MyClass {}
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: enum variant not found\n --> .rdl:8:15")]
fn enum_arg_unknown_variant_errors() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    #[repr(i32)]
    enum Color { Red = 0, Green = 1, Blue = 2, }
    attribute PaletteAttribute { fn(value: Color); }

    #[Palette(Purple)]
    class MyClass {}
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: positional attribute arguments must come before named arguments\n --> .rdl:6:27")]
fn positional_after_named_errors() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    attribute FooAttribute { fn(value: u32); }

    #[Foo(named_prop = 1, 42)]
    class MyClass {}
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: no matching attribute constructor found\n --> .rdl:6:5")]
fn no_matching_ctor_errors() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    attribute FooAttribute { fn(value: u32); }

    #[Foo(1, 2, 3)]
    class MyClass {}
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: attribute has no property `unknown`\n --> .rdl:6:5")]
fn unknown_property_errors() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    attribute FooAttribute { fn(); version: u32, }

    #[Foo(unknown = 42)]
    class MyClass {}
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: attribute cannot use top-level `name = value` syntax\n --> .rdl:6:5")]
fn top_level_name_value_syntax_errors() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    attribute FooAttribute { fn(); }

    #[Foo = "bar"]
    class MyClass {}
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}
