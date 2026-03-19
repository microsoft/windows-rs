use super::*;
use std::collections::HashMap;
use windows_metadata::AsRow;

/// Write a struct/union type definition and any nested types it contains.
///
/// Returns a list of `(name, tokens)` pairs — the un-nested helper types first,
/// followed by the top-level type itself — so all of them can be inserted into
/// the layout as independent, flat definitions.
///
/// Nested types are promoted to the enclosing namespace with a synthesised name
/// using the same numeric-suffix scheme as `windows-bindgen`: `OUTER_0`,
/// `OUTER_1`, `OUTER_0_0`, etc., where the index is the position of the nested
/// type in the parent's nested-class list.
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
    let packed_attr = write_packed_attr(item);
    let custom_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    let main_tokens = quote! {
        #packed_attr
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
///
/// The naming scheme matches `windows-bindgen`: each nested type is named
/// `{outer_flat_name}_{index}` where `index` is the 0-based position of the
/// nested type in the parent's nested-class list.
fn collect_nested(
    namespace: &str,
    parent: &metadata::reader::TypeDef,
    outer_flat_name: &str,
    output: &mut Vec<(String, TokenStream)>,
) -> HashMap<String, String> {
    let mut flat_names: HashMap<String, String> = HashMap::new();

    for (index, nested) in parent.index().nested(*parent).enumerate() {
        let nested_leaf = nested.name();
        let flat_name = format!("{outer_flat_name}_{index}");
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

/// Emits a `#[packed(N)]` token stream if the type has a `ClassLayout` with a
/// non-zero packing size, otherwise returns an empty token stream.
fn write_packed_attr(item: &metadata::reader::TypeDef) -> TokenStream {
    if let Some(layout) = item.class_layout() {
        let size = layout.packing_size();
        if size > 0 {
            let size_literal = proc_macro2::Literal::u16_unsuffixed(size);
            return quote! { #[packed(#size_literal)] };
        }
    }
    quote! {}
}
