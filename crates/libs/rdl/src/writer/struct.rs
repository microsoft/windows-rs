use super::*;
use std::collections::HashMap;
use windows_metadata::AsRow;

/// Write a struct/union type definition and any nested types it contains.
///
/// Returns a list of `(name, tokens)` pairs — the un-nested helper types first,
/// followed by the top-level type itself — so all of them can be inserted into
/// the layout as independent, flat definitions.
///
/// Nested types that arise from Windows metadata (e.g. anonymous inner structs)
/// are promoted to the enclosing namespace with a synthesised name derived from
/// the outer type name and, where possible, the field name that refers to them.
pub fn write_struct_items(item: &metadata::reader::TypeDef) -> Vec<(String, TokenStream)> {
    // Nested types are only emitted as part of their enclosing type.
    if item
        .flags()
        .contains(metadata::TypeAttributes::NestedPublic)
    {
        return vec![];
    }

    let namespace = item.namespace();
    let outer_name = item.name();

    // Collect all un-nested helper types (depth-first so leaves come first).
    let mut unnested: Vec<(String, TokenStream)> = vec![];
    let flat_names = collect_nested(namespace, item, outer_name, &mut unnested);

    // Write the main type using flat name references for any nested fields.
    let name_ident = write_ident(outer_name);
    let fields: Vec<_> = item
        .fields()
        .map(|field| write_field_flat(namespace, &field, &flat_names))
        .collect();

    let keyword = struct_keyword(item);
    let custom_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    let main_tokens = quote! {
        #(#custom_attrs)*
        #keyword #name_ident {
            #(#fields)*
        }
    };

    unnested.push((outer_name.to_string(), main_tokens));
    unnested
}

/// Recursively collect all nested types of `parent`, emitting each as a flat
/// top-level type.  Returns a map from the nested type's leaf name (as stored
/// in the metadata) to the synthesised flat name used in the emitted rdl.
fn collect_nested(
    namespace: &str,
    parent: &metadata::reader::TypeDef,
    outer_flat_name: &str,
    output: &mut Vec<(String, TokenStream)>,
) -> HashMap<String, String> {
    let mut flat_names: HashMap<String, String> = HashMap::new();

    for nested in parent.index().nested(*parent) {
        let nested_leaf = nested.name();
        let flat_name = compute_flat_name(outer_flat_name, parent, &nested);
        flat_names.insert(nested_leaf.to_string(), flat_name.clone());

        // Recurse before emitting so that leaves appear before their parents.
        let child_flat_names = collect_nested(namespace, &nested, &flat_name, output);

        let name_ident = write_ident(&flat_name);
        let fields: Vec<_> = nested
            .fields()
            .map(|field| write_field_flat(namespace, &field, &child_flat_names))
            .collect();
        let keyword = struct_keyword(&nested);

        output.push((flat_name, quote! { #keyword #name_ident { #(#fields)* } }));
    }

    flat_names
}

/// Compute the flat (un-nested) name for `nested` whose enclosing type has the
/// flat name `outer_flat_name`.
///
/// * If the nested type name starts with `_Anonymous_` we try to find a field of
///   `outer` that refers to this nested type and derive the suffix from that
///   field's name.  This converts e.g. `_Anonymous_e__Struct` referenced by a
///   field called `Options` inside `OUTER` into `OUTER_OPTIONS`.
/// * Otherwise the suffix is the nested type's own name, giving `OUTER_INNER`.
fn compute_flat_name(
    outer_flat_name: &str,
    outer: &metadata::reader::TypeDef,
    nested: &metadata::reader::TypeDef,
) -> String {
    let nested_name = nested.name();

    let suffix = if nested_name.starts_with("_Anonymous_") {
        referring_field_name(outer, nested_name)
            .map(|n| n.to_uppercase())
            .unwrap_or_else(|| nested_name.to_string())
    } else {
        nested_name.to_string()
    };

    format!("{}_{}", outer_flat_name, suffix)
}

/// Return the name of the first field in `parent` whose type directly references
/// `nested_leaf` (the leaf name of a nested type).
fn referring_field_name(parent: &metadata::reader::TypeDef, nested_leaf: &str) -> Option<String> {
    for field in parent.fields() {
        if type_references_nested(&field.ty(), nested_leaf) {
            return Some(field.name().to_string());
        }
    }
    None
}

/// Return `true` if `ty` (or any type reachable from it via pointer / array
/// indirection) directly references a nested type named `nested_leaf`.
fn type_references_nested(ty: &metadata::Type, nested_leaf: &str) -> bool {
    match ty {
        metadata::Type::Name(tn) if tn.namespace.is_empty() => {
            let leaf = tn.name.rsplit('/').next().unwrap_or(&tn.name);
            leaf == nested_leaf
        }
        metadata::Type::ArrayFixed(inner, _)
        | metadata::Type::PtrMut(inner, _)
        | metadata::Type::PtrConst(inner, _) => type_references_nested(inner, nested_leaf),
        _ => false,
    }
}

/// Write a single struct/union field, replacing any reference to a nested type
/// with the corresponding flat name from `flat_names`.
fn write_field_flat(
    namespace: &str,
    item: &metadata::reader::Field,
    flat_names: &HashMap<String, String>,
) -> TokenStream {
    let name = write_ident(item.name());
    let resolved_ty = resolve_nested(&item.ty(), namespace, flat_names);
    let ty = write_type(namespace, &resolved_ty);
    let field_attrs = write_custom_attributes(item.attributes(), namespace, item.index());
    quote! { #(#field_attrs)* #name: #ty, }
}

/// Recursively replace nested-type references inside `ty` with their flat
/// equivalents.  Any type whose name does not appear in `flat_names` is left
/// unchanged (it is a regular, already-flat reference).
fn resolve_nested(
    ty: &metadata::Type,
    namespace: &str,
    flat_names: &HashMap<String, String>,
) -> metadata::Type {
    match ty {
        metadata::Type::Name(tn) if tn.namespace.is_empty() => {
            let leaf = tn.name.rsplit('/').next().unwrap_or(&tn.name);
            if let Some(flat) = flat_names.get(leaf) {
                metadata::Type::named(namespace, flat)
            } else {
                ty.clone()
            }
        }
        metadata::Type::ArrayFixed(inner, len) => {
            metadata::Type::ArrayFixed(Box::new(resolve_nested(inner, namespace, flat_names)), *len)
        }
        metadata::Type::PtrMut(inner, ptrs) => metadata::Type::PtrMut(
            Box::new(resolve_nested(inner, namespace, flat_names)),
            *ptrs,
        ),
        metadata::Type::PtrConst(inner, ptrs) => metadata::Type::PtrConst(
            Box::new(resolve_nested(inner, namespace, flat_names)),
            *ptrs,
        ),
        _ => ty.clone(),
    }
}

fn struct_keyword(item: &metadata::reader::TypeDef) -> TokenStream {
    if item
        .flags()
        .contains(metadata::TypeAttributes::ExplicitLayout)
    {
        quote! { union }
    } else {
        quote! { struct }
    }
}
