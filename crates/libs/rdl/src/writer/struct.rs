use super::*;
use std::collections::{HashMap, HashSet};
use windows_metadata::AsRow;

/// Write a struct/union type definition, emitting any anonymous nested types
/// inline at the field that references them.
///
/// Returns a list of `(name, tokens)` pairs. The top-level type is always
/// present. Nested types that are referenced *directly* by a field (the common
/// C anonymous-member case) are emitted inline within it; any nested type that
/// is instead referenced through an array/pointer (or not referenced at all) is
/// hoisted to a flat top-level sibling using the `windows-bindgen` naming scheme
/// (`OUTER_0`, `OUTER_0_0`, …) and appears as an additional entry in the list.
pub fn write_struct_items(
    item: &metadata::reader::TypeDef,
) -> Result<Vec<(String, TokenStream)>, Error> {
    // Nested types are only emitted as part of their enclosing type.
    if item
        .flags()
        .contains(metadata::TypeAttributes::NestedPublic)
    {
        return Ok(vec![]);
    }

    let namespace = item.namespace();
    let mut hoisted: Vec<(String, TokenStream)> = vec![];
    let tokens = write_record(
        namespace,
        item,
        false,
        item.arches(),
        packing_of(item),
        &mut hoisted,
    )?;
    hoisted.push((item.name().to_string(), tokens));
    Ok(hoisted)
}

/// Recursively write a struct/union.
///
/// When `inline` is `true` the record is written as an anonymous inline nested
/// type (no name, keyword-first) suitable for use at a field's type position;
/// otherwise it is written as a named top-level type.
///
/// `parent_arches`/`parent_packing` carry the effective architecture and packing
/// inherited from enclosing types so that any *hoisted* (array/pointer-wrapped)
/// nested helper gets the correct `#[arch]`/`#[packed]` even when it has none of
/// its own. Directly-inlined nested types instead carry their own attributes,
/// which round-trip 1:1 through the reader.
fn write_record(
    namespace: &str,
    item: &metadata::reader::TypeDef,
    inline: bool,
    parent_arches: i32,
    parent_packing: Option<u16>,
    hoisted: &mut Vec<(String, TokenStream)>,
) -> Result<TokenStream, Error> {
    let nested: Vec<metadata::reader::TypeDef> = item.index().nested(*item).collect();

    // The leaf names referenced *directly* (bare) by a field — these are inlined.
    let bare: HashSet<String> = item
        .fields()
        .filter_map(|field| match field.ty() {
            metadata::Type::ValueName(tn) if tn.namespace.is_empty() && !tn.name.contains('/') => {
                Some(tn.name)
            }
            _ => None,
        })
        .collect();

    // Hoist any nested type that is not consumed by a bare field reference. Its
    // index in the parent's nested-class list drives the flat name, matching the
    // scheme used elsewhere so wrapped references resolve.
    let mut flat_names: HashMap<String, String> = HashMap::new();
    for (index, child) in nested.iter().enumerate() {
        if bare.contains(child.name()) {
            continue;
        }
        let flat_name = format!("{}_{index}", item.name());
        flat_names.insert(child.name().to_string(), flat_name.clone());
        let effective_arches = parent_arches | child.arches();
        let effective_packing = packing_of(child).or(parent_packing);
        hoist_subtree(
            namespace,
            child,
            &flat_name,
            effective_arches,
            effective_packing,
            hoisted,
        )?;
    }

    // Nested children consumed inline, keyed by leaf name.
    let inline_map: HashMap<String, metadata::reader::TypeDef> = nested
        .iter()
        .filter(|child| bare.contains(child.name()))
        .map(|child| (child.name().to_string(), *child))
        .collect();

    let fields: Vec<TokenStream> = item
        .fields()
        .map(|field| -> Result<TokenStream, Error> {
            if let metadata::Type::ValueName(tn) = field.ty()
                && tn.namespace.is_empty()
                && let Some(child) = inline_map.get(&tn.name)
            {
                let effective_arches = parent_arches | child.arches();
                let effective_packing = packing_of(child).or(parent_packing);
                let inner = write_record(
                    namespace,
                    child,
                    true,
                    effective_arches,
                    effective_packing,
                    hoisted,
                )?;
                let name = write_ident(field.name());
                let field_attrs =
                    write_custom_attributes(field.attributes(), namespace, field.index())?;
                return Ok(quote! { #(#field_attrs)* #name: #inner, });
            }
            write_field_flat(namespace, &field, &flat_names)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let keyword = struct_keyword(item);
    let packed_attr = write_packed_attr(item);
    let align_attr = write_align_attr(item);
    // A nested type's architecture is always that of its enclosing type — it cannot
    // diverge — so the `#[arch]` is redundant noise on an inline nested record and is
    // emitted only on top-level types. The reader re-derives it from the parent.
    let arch_attr = if inline {
        quote! {}
    } else {
        write_arch_attr(item.arches())
    };
    let custom_attrs = write_custom_attributes_except(
        item.attributes(),
        namespace,
        item.index(),
        &["SupportedArchitectureAttribute", "AlignmentAttribute"],
    )?;

    if inline {
        Ok(quote! {
            #packed_attr #align_attr #(#custom_attrs)*
            #keyword {
                #(#fields)*
            }
        })
    } else {
        let name_ident = write_ident(item.name());
        Ok(quote! {
            #packed_attr
            #align_attr
            #arch_attr
            #(#custom_attrs)*
            #keyword #name_ident {
                #(#fields)*
            }
        })
    }
}

/// Recursively hoist a nested type (and all of its descendants) to flat
/// top-level siblings, using `windows-bindgen`'s numeric-suffix naming scheme.
/// This is the fallback path for nested types that cannot be inlined because
/// they are referenced through an array/pointer.
fn hoist_subtree(
    namespace: &str,
    node: &metadata::reader::TypeDef,
    flat_name: &str,
    arches: i32,
    packing: Option<u16>,
    hoisted: &mut Vec<(String, TokenStream)>,
) -> Result<(), Error> {
    // Hoist descendants first so leaves precede their parents, collecting the
    // flat names used to rewrite this node's fields.
    let mut child_flat_names: HashMap<String, String> = HashMap::new();
    for (index, child) in node.index().nested(*node).enumerate() {
        let child_flat = format!("{flat_name}_{index}");
        child_flat_names.insert(child.name().to_string(), child_flat.clone());
        let effective_arches = arches | child.arches();
        let effective_packing = packing_of(&child).or(packing);
        hoist_subtree(
            namespace,
            &child,
            &child_flat,
            effective_arches,
            effective_packing,
            hoisted,
        )?;
    }

    let name_ident = write_ident(flat_name);
    let fields: Vec<TokenStream> = node
        .fields()
        .map(|field| write_field_flat(namespace, &field, &child_flat_names))
        .collect::<Result<Vec<_>, _>>()?;
    let keyword = struct_keyword(node);
    let arch_attr = write_arch_attr(arches);
    let packed_attr = write_packed_attr_value(packing);
    let align_attr = write_align_attr(node);
    let custom_attrs = write_custom_attributes_except(
        node.attributes(),
        namespace,
        node.index(),
        &["SupportedArchitectureAttribute", "AlignmentAttribute"],
    )?;

    hoisted.push((
        flat_name.to_string(),
        quote! { #packed_attr #align_attr #arch_attr #(#custom_attrs)* #keyword #name_ident { #(#fields)* } },
    ));
    Ok(())
}

/// The non-zero packing size of a type's `ClassLayout`, if any.
fn packing_of(item: &metadata::reader::TypeDef) -> Option<u16> {
    item.class_layout()
        .map(|l| l.packing_size())
        .filter(|&s| s > 0)
}

/// Write a single struct/union field, replacing any reference to a hoisted
/// nested type with the corresponding flat name from `flat_names`.
fn write_field_flat(
    namespace: &str,
    item: &metadata::reader::Field,
    flat_names: &HashMap<String, String>,
) -> Result<TokenStream, Error> {
    let name = write_ident(item.name());
    let resolved_ty = resolve_nested(&item.ty(), namespace, flat_names);
    let ty = write_type(namespace, &resolved_ty);

    // A backing bit-field unit renders in the concise C-like block form
    // (`_bitfield: u8 { a: 1, b: 2 }`) rather than as a plain field carrying
    // per-member `NativeBitfieldAttribute`s. Any *other* attributes on the field
    // (rare) still render normally.
    let members = collect_bitfield_members(item);
    if !members.is_empty() {
        let block = write_bitfield_block(&members);
        let field_attrs = write_custom_attributes_except(
            item.attributes(),
            namespace,
            item.index(),
            &["NativeBitfieldAttribute"],
        )?;
        return Ok(quote! { #(#field_attrs)* #name: #ty { #(#block)* }, });
    }

    let field_attrs = write_custom_attributes(item.attributes(), namespace, item.index())?;
    Ok(quote! { #(#field_attrs)* #name: #ty, })
}

/// Collects this field's `NativeBitfieldAttribute` members as `(name, offset, width)`,
/// sorted by offset. Empty when the field is not a bit-field backing unit.
fn collect_bitfield_members(item: &metadata::reader::Field) -> Vec<(String, u32, u32)> {
    let mut members: Vec<(String, u32, u32)> = item
        .attributes()
        .filter(|attr| {
            attr.namespace() == METADATA_NAMESPACE && attr.name() == "NativeBitfieldAttribute"
        })
        .filter_map(|attr| {
            let values = attr.value();
            let name = match values.first().map(|(_, v)| v) {
                Some(metadata::Value::Utf8(s)) => s.clone(),
                _ => return None,
            };
            let as_u32 = |v: Option<&(String, metadata::Value)>| match v.map(|(_, v)| v) {
                Some(metadata::Value::I64(n)) => Some(*n as u32),
                _ => None,
            };
            Some((name, as_u32(values.get(1))?, as_u32(values.get(2))?))
        })
        .collect();
    members.sort_by_key(|(_, offset, _)| *offset);
    members
}

/// Renders the members of a bit-field block, inserting anonymous padding (`_: n`)
/// for any leading or interior gap so the reader recomputes each member's implicit
/// offset (the cumulative width of the preceding members) faithfully.
fn write_bitfield_block(members: &[(String, u32, u32)]) -> Vec<TokenStream> {
    let mut out = vec![];
    let mut cursor = 0u32;
    for (name, offset, width) in members {
        if *offset > cursor {
            let pad = Literal::u32_unsuffixed(offset - cursor);
            out.push(quote! { _: #pad, });
        }
        let member = write_ident(name);
        let width_lit = Literal::u32_unsuffixed(*width);
        out.push(quote! { #member: #width_lit, });
        cursor = offset + width;
    }
    out
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
        metadata::Type::ValueName(tn) if tn.namespace.is_empty() => {
            let leaf = tn.name.rsplit('/').next().unwrap_or(&tn.name);
            if let Some(flat) = flat_names.get(leaf) {
                metadata::Type::value_named(namespace, flat)
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

/// Emits a `#[packed(N)]` token stream if the type has a `ClassLayout` with a
/// non-zero packing size, otherwise returns an empty token stream.
fn write_packed_attr(item: &metadata::reader::TypeDef) -> TokenStream {
    write_packed_attr_value(packing_of(item))
}

/// Emits a `#[packed(N)]` token stream for the given packing size, or an empty
/// token stream when `packing` is `None`.
fn write_packed_attr_value(packing: Option<u16>) -> TokenStream {
    if let Some(size) = packing {
        let size_literal = Literal::u16_unsuffixed(size);
        return quote! { #[packed(#size_literal)] };
    }
    quote! {}
}

/// Emits an `#[align(N)]` token stream if the type carries an
/// `AlignmentAttribute` (forced over-alignment from `__declspec(align(N))` /
/// `alignas(N)`), otherwise returns an empty token stream. Unlike packing,
/// forced alignment is *not* inherited by nested helper types — each type
/// carries its own attribute.
fn write_align_attr(item: &metadata::reader::TypeDef) -> TokenStream {
    let Some(attribute) = item.find_attribute("AlignmentAttribute") else {
        return quote! {};
    };
    let Some((_, metadata::Value::I32(alignment))) = attribute.value().into_iter().next() else {
        return quote! {};
    };
    let size_literal = Literal::i32_unsuffixed(alignment);
    quote! { #[align(#size_literal)] }
}
