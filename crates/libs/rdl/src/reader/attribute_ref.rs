use super::*;

/// A parsed and validated reference to a custom attribute defined in metadata or RDL.
///
/// Built-in RDL attributes (`win32`, `winrt`, `repr`, `link`, `activatable`, etc.) are
/// handled separately by the individual encode functions and are never represented here.
pub struct AttributeRef {
    pub type_name: metadata::TypeName,
    pub args: Vec<(String, metadata::Value)>,
}

/// Finds an attribute type (as `<ident>Attribute`) in the encoder's index and
/// reference, returning its `TypeName` and all constructor parameter-type lists.
fn find_attribute_type(
    encoder: &Encoder,
    path: &syn::Path,
) -> Option<(metadata::TypeName, Vec<Vec<metadata::Type>>)> {
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
        (1..=parts.len())
            .rev()
            .map(|len| parts[..len].join("."))
            .collect()
    };

    for ns in &namespaces {
        if let Some(result) = find_in_reference(encoder, ns, &attr_name)
            .or_else(|| find_in_index(encoder, ns, &attr_name))
        {
            return Some((metadata::TypeName::named(ns, &attr_name), result));
        }
    }

    None
}

/// Searches the metadata reference for a type with the given namespace/name that
/// has `TypeCategory::Attribute`, and returns all constructor type lists.
fn find_in_reference(
    encoder: &Encoder,
    namespace: &str,
    attr_name: &str,
) -> Option<Vec<Vec<metadata::Type>>> {
    let mut constructors = vec![];
    for typedef in encoder.reference.get(namespace, attr_name) {
        if typedef.category() == metadata::reader::TypeCategory::Attribute {
            for method in typedef.methods() {
                if method.name() == ".ctor" {
                    let sig = method.signature(&[]);
                    constructors.push(sig.types);
                }
            }
        }
    }
    if constructors.is_empty() {
        None
    } else {
        Some(constructors)
    }
}

/// Searches the RDL index for an `Item::Attribute` with the given namespace/name
/// and returns its constructor type lists (built from the `syn::TypeBareFn` methods).
fn find_in_index(
    encoder: &Encoder,
    namespace: &str,
    attr_name: &str,
) -> Option<Vec<Vec<metadata::Type>>> {
    let (_, item) = encoder
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
            .map(|arg| encode_type(encoder, &arg.ty))
            .collect();
        if let Ok(types) = types {
            constructors.push(types);
        }
    }
    Some(constructors)
}

/// Tries to match the attribute's arguments against the provided constructor
/// type lists and returns the validated argument values on success.
fn match_constructor_args(
    encoder: &Encoder,
    attr: &syn::Attribute,
    constructors: &[Vec<metadata::Type>],
) -> Result<Vec<(String, metadata::Value)>, Error> {
    // Parse the argument expressions.
    let args: Vec<syn::Expr> = match &attr.meta {
        syn::Meta::Path(_) => vec![],
        syn::Meta::List(_) => attr
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated,
            )
            .map_err(|e| {
                let start = e.span().start();
                Error::new(
                    &e.to_string(),
                    encoder.source_file,
                    start.line,
                    start.column,
                )
            })?
            .into_iter()
            .collect(),
        syn::Meta::NameValue(_) => {
            return encoder.err(attr, "attribute cannot use `name = value` syntax");
        }
    };

    // Try each constructor overload in order.
    for types in constructors {
        if types.len() != args.len() {
            continue;
        }

        let mut values = vec![];
        let mut matched = true;

        for (ty, arg) in types.iter().zip(args.iter()) {
            match encode_attr_value(encoder, ty, arg) {
                Ok(v) => values.push((String::new(), v)),
                Err(_) => {
                    matched = false;
                    break;
                }
            }
        }

        if matched {
            return Ok(values);
        }
    }

    encoder.err(attr, "no matching attribute constructor found")
}

/// Converts a `syn::Expr` to a `metadata::Value` for the given constructor
/// parameter type.  Extends the basic `encode_value` helper with support for
/// string literals and `System.Type` (which is serialised as a UTF-8 string).
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
            syn::Expr::Path(syn::ExprPath { path, .. }) => {
                match encode_path(encoder, path)? {
                    metadata::Type::Name(tn) => Ok(metadata::Value::TypeName(tn)),
                    _ => encoder.err(value, "expected type name"),
                }
            }
            _ => encoder.err(value, "expected type path"),
        },
        _ => encode_value(encoder, ty, value),
    }
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

    let (type_name, constructors) = find_attribute_type(encoder, path)
        .ok_or_else(|| encoder.error(attr, "attribute type not found"))?;

    let args = match_constructor_args(encoder, attr, &constructors)?;

    Ok(AttributeRef { type_name, args })
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
        types: attr_ref.args.iter().map(|(_, v)| v.ty()).collect(),
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
#[should_panic(
    expected = r#"{ message: "attribute type not found", file_name: ".rdl", line: 4, column: 4 }"#
)]
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
#[should_panic(
    expected = r#"{ message: "no matching attribute constructor found", file_name: ".rdl", line: 6, column: 4 }"#
)]
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
#[should_panic(
    expected = r#"{ message: "no matching attribute constructor found", file_name: ".rdl", line: 6, column: 4 }"#
)]
fn wrong_arg_count_errors() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    attribute FooAttribute { fn(value: u32); }

    #[Foo(1, 2)]
    class MyClass {}
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}
